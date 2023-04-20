use std::collections::HashMap;

use aiken_lang::{
    ast::{BinOp, Definition, ModuleKind, Tracing, TypedDataType, TypedFunction},
    builtins,
    gen_uplc::{
        builder::{DataTypeKey, FunctionAccessKey},
        CodeGenerator,
    },
    parser,
    tipo::{error::Warning, TypeInfo},
    IdGenerator,
};
use indexmap::IndexMap;
use leptos::{SignalSet, WriteSignal};
use uplc::{
    ast::{NamedDeBruijn, Program},
    machine::cost_model::ExBudget,
};

pub struct EvalHint {
    pub bin_op: BinOp,
    pub left: Program<NamedDeBruijn>,
    pub right: Program<NamedDeBruijn>,
}

pub struct Project {
    id_gen: IdGenerator,
    module_types: HashMap<String, TypeInfo>,
    functions: IndexMap<FunctionAccessKey, TypedFunction>,
    data_types: IndexMap<DataTypeKey, TypedDataType>,
}

impl Project {
    pub fn new() -> Self {
        let id_gen = IdGenerator::new();

        let mut module_types = HashMap::new();
        module_types.insert("aiken".to_string(), builtins::prelude(&id_gen));
        module_types.insert("aiken/builtin".to_string(), builtins::plutus(&id_gen));

        let functions = builtins::prelude_functions(&id_gen);
        let data_types = builtins::prelude_data_types(&id_gen);

        Project {
            id_gen,
            module_types,
            functions,
            data_types,
        }
    }

    pub fn build(&self, source_code: &str, set_warnings: WriteSignal<Vec<Warning>>) {
        let kind = ModuleKind::Validator;
        let (mut ast, _extra) = parser::module(source_code, kind).expect("Failed to parse module");
        let name = "play".to_string();
        ast.name = name.clone();

        let mut warnings = vec![];

        let typed_ast = ast
            .infer(
                &self.id_gen,
                kind,
                &name,
                &self.module_types,
                Tracing::NoTraces,
                &mut warnings,
            )
            .expect("Failed to type-check module");

        set_warnings.set(warnings);

        let mut module_types: IndexMap<&String, &TypeInfo> = self.module_types.iter().collect();

        module_types.insert(&name, &typed_ast.type_info);

        let mut functions = IndexMap::new();
        for (k, v) in &self.functions {
            functions.insert(k.clone(), v);
        }

        let mut data_types = IndexMap::new();
        for (k, v) in &self.data_types {
            data_types.insert(k.clone(), v);
        }

        let mut tests = vec![];
        let mut validators = vec![];

        for def in typed_ast.definitions() {
            match def {
                Definition::Fn(func) => {
                    functions.insert(
                        FunctionAccessKey {
                            module_name: name.clone(),
                            function_name: func.name.clone(),
                            variant_name: String::new(),
                        },
                        func,
                    );
                }
                Definition::DataType(dt) => {
                    data_types.insert(
                        DataTypeKey {
                            module_name: name.clone(),
                            defined_type: dt.name.clone(),
                        },
                        dt,
                    );
                }
                Definition::Test(t) => tests.push(t),
                Definition::Validator(v) => validators.push(v),

                Definition::TypeAlias(_) | Definition::ModuleConstant(_) | Definition::Use(_) => {}
            }
        }

        let mut generator = CodeGenerator::new(functions, data_types, module_types);

        for test in tests {
            let _evaluation_hint = test.test_hint().map(|(bin_op, left_src, right_src)| {
                let left = generator
                    .clone()
                    .generate_test(&left_src)
                    .try_into()
                    .unwrap();

                let right = generator
                    .clone()
                    .generate_test(&right_src)
                    .try_into()
                    .unwrap();

                EvalHint {
                    bin_op,
                    left,
                    right,
                }
            });

            let program = generator.generate_test(&test.body);

            let program: Program<NamedDeBruijn> = program.try_into().unwrap();

            let eval_result = program.eval(ExBudget::default());

            leptos::log!("{}", eval_result.result().unwrap().to_pretty());
        }

        for validator in validators {
            let program = generator.generate(validator);

            leptos::log!("{}", program.to_pretty());
        }
    }
}
