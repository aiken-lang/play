use std::collections::HashMap;

use aiken_lang::{
    ast::{
        BinOp, Definition, ModuleKind, Tracing, TypedDataType, TypedDefinition, TypedFunction,
        TypedValidator,
    },
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
use leptos::{log, SignalSet, SignalUpdate, WriteSignal};
use uplc::{
    ast::{NamedDeBruijn, Program},
    machine::cost_model::ExBudget,
};

use crate::{compiler_error::CompilerError, stdlib};

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

#[derive(Clone)]
pub struct TestResult {
    pub success: bool,
    pub spent_budget: ExBudget,
    pub logs: Vec<String>,
    pub name: String,
}

impl Project {
    pub fn new() -> Self {
        let id_gen = IdGenerator::new();

        let mut module_types = HashMap::new();
        module_types.insert("aiken".to_string(), builtins::prelude(&id_gen));
        module_types.insert("aiken/builtin".to_string(), builtins::plutus(&id_gen));

        let mut functions = builtins::prelude_functions(&id_gen);
        let mut data_types = builtins::prelude_data_types(&id_gen);

        for (module_name, module_src) in stdlib::MODULES {
            let (mut ast, _extra) = parser::module(module_src, ModuleKind::Lib).unwrap();

            ast.name = module_name.to_string();

            let mut warnings = vec![];

            let typed_ast = ast
                .infer(
                    &id_gen,
                    ModuleKind::Lib,
                    module_name,
                    &module_types,
                    Tracing::NoTraces,
                    &mut warnings,
                )
                .map_err(|e| {
                    log!("{}", e);
                })
                .unwrap();

            for def in typed_ast.definitions.into_iter() {
                match def {
                    Definition::Fn(func) => {
                        functions.insert(
                            FunctionAccessKey {
                                module_name: module_name.to_string(),
                                function_name: func.name.clone(),
                                variant_name: "".to_string(),
                            },
                            func,
                        );
                    }
                    Definition::DataType(data) => {
                        data_types.insert(
                            DataTypeKey {
                                module_name: module_name.to_string(),
                                defined_type: "".to_string(),
                            },
                            data,
                        );
                    }
                    Definition::TypeAlias(_)
                    | Definition::Use(_)
                    | Definition::ModuleConstant(_)
                    | Definition::Test(_)
                    | Definition::Validator(_) => (),
                }
            }

            module_types.insert(module_name.to_string(), typed_ast.type_info);
        }

        Project {
            id_gen,
            module_types,
            functions,
            data_types,
        }
    }

    pub fn build(
        &self,
        source_code: &str,
        set_warnings: WriteSignal<Vec<(usize, Warning)>>,
        set_errors: WriteSignal<Vec<(usize, CompilerError)>>,
        set_test_results: WriteSignal<Vec<(usize, TestResult)>>,
    ) {
        let kind = ModuleKind::Validator;

        match parser::module(source_code, kind) {
            Ok((mut ast, _extra)) => {
                let name = "play".to_string();
                ast.name = name.clone();

                let mut warnings = vec![];

                match ast.infer(
                    &self.id_gen,
                    kind,
                    &name,
                    &self.module_types,
                    Tracing::NoTraces,
                    &mut warnings,
                ) {
                    Ok(typed_ast) => {
                        let mut module_types: IndexMap<&String, &TypeInfo> =
                            self.module_types.iter().collect();

                        module_types.insert(&name, &typed_ast.type_info);

                        let (tests, validators, functions, data_types) =
                            self.collect_definitions(name.clone(), typed_ast.definitions());

                        let mut generator = CodeGenerator::new(functions, data_types, module_types);

                        run_tests(tests, &mut generator, set_test_results);

                        for validator in validators {
                            let program = generator.generate(validator);

                            leptos::log!("{}", program.to_pretty());
                        }
                    }
                    Err(err) => set_errors.set(vec![(0, CompilerError::Type(err))]),
                }

                set_warnings.set(warnings.into_iter().enumerate().collect());
            }
            Err(errs) => {
                set_errors.set(
                    errs.into_iter()
                        .map(CompilerError::Parse)
                        .enumerate()
                        .collect(),
                );
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn collect_definitions<'a>(
        &'a self,
        name: String,
        definitions: impl Iterator<Item = &'a TypedDefinition>,
    ) -> (
        Vec<&'a TypedFunction>,
        Vec<&'a TypedValidator>,
        IndexMap<FunctionAccessKey, &'a TypedFunction>,
        IndexMap<DataTypeKey, &'a TypedDataType>,
    ) {
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

        for def in definitions {
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

        (tests, validators, functions, data_types)
    }
}

fn run_tests(
    tests: Vec<&TypedFunction>,
    generator: &mut CodeGenerator,
    set_test_results: WriteSignal<Vec<(usize, TestResult)>>,
) {
    for (index, test) in tests.into_iter().enumerate() {
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

        let mut eval_result = program.eval(ExBudget::default());

        set_test_results.update(|t| {
            t.push((
                index,
                TestResult {
                    success: !eval_result.failed(),
                    spent_budget: eval_result.cost(),
                    logs: eval_result.logs(),
                    name: test.name.clone(),
                },
            ))
        });
    }
}

pub fn format(src: &str, set_errors: WriteSignal<Vec<(usize, CompilerError)>>) -> Option<String> {
    match parser::module(src, ModuleKind::Validator) {
        Ok((ast, extra)) => {
            let mut output = String::new();

            aiken_lang::format::pretty(&mut output, ast, extra, src);

            Some(output)
        }
        Err(errs) => {
            set_errors.set(
                errs.into_iter()
                    .map(CompilerError::Parse)
                    .enumerate()
                    .collect(),
            );

            None
        }
    }
}
