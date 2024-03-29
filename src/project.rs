use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    ast::{DeBruijn, NamedDeBruijn, Program},
    machine::cost_model::ExBudget,
};

use crate::{compiler_error::CompilerError, stdlib};

pub struct EvalHint {
    pub bin_op: BinOp,
    pub left: Program<NamedDeBruijn>,
    pub right: Program<NamedDeBruijn>,
}

#[derive(Clone)]
pub struct Project {
    id_gen: IdGenerator,
    module_types: HashMap<String, TypeInfo>,
    functions: IndexMap<FunctionAccessKey, TypedFunction>,
    data_types: IndexMap<DataTypeKey, TypedDataType>,
    is_stdlib_setup: bool,
}

#[derive(Clone)]
pub struct TestResult {
    pub success: bool,
    pub spent_budget: ExBudget,
    pub logs: Vec<String>,
    pub name: String,
}

impl Project {
    pub fn new() -> Rc<RefCell<Self>> {
        let id_gen = IdGenerator::new();

        let mut module_types = HashMap::new();
        module_types.insert("aiken".to_string(), builtins::prelude(&id_gen));
        module_types.insert("aiken/builtin".to_string(), builtins::plutus(&id_gen));

        let functions = builtins::prelude_functions(&id_gen);
        let data_types = builtins::prelude_data_types(&id_gen);

        RefCell::new(Project {
            id_gen,
            module_types,
            functions,
            data_types,
            is_stdlib_setup: false,
        })
        .into()
    }

    pub fn build(
        &mut self,
        source_code: &str,
        set_validators: WriteSignal<Vec<(usize, String, String)>>,
        set_warnings: WriteSignal<Vec<(usize, Warning)>>,
        set_errors: WriteSignal<Vec<(usize, CompilerError)>>,
        set_test_results: WriteSignal<Vec<(usize, TestResult)>>,
    ) {
        if !self.is_stdlib_setup {
            self.setup_stdlib();
        }

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

                        let mut generator =
                            CodeGenerator::new(functions, data_types, module_types, false);

                        run_tests(tests, &mut generator, set_test_results);

                        for (index, validator) in validators.into_iter().enumerate() {
                            let name = format!(
                                "{}{}",
                                validator.fun.name,
                                validator
                                    .other_fun
                                    .clone()
                                    .map(|o| format!(".{}", o.name))
                                    .unwrap_or_else(|| "".to_string())
                            );

                            let program = generator.generate(validator);

                            let program: Program<DeBruijn> = program.try_into().unwrap();

                            let program = program.to_hex().unwrap();

                            set_validators.update(|v| v.push((index, name, program)))
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

    pub fn setup_stdlib(&mut self) {
        for (module_name, module_src) in stdlib::MODULES {
            log!("{}", module_name);
            let (mut ast, _extra) = parser::module(module_src, ModuleKind::Lib).unwrap();

            ast.name = module_name.to_string();

            let mut warnings = vec![];

            let typed_ast = ast
                .infer(
                    &self.id_gen,
                    ModuleKind::Lib,
                    module_name,
                    &self.module_types,
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
                        self.functions.insert(
                            FunctionAccessKey {
                                module_name: module_name.to_string(),
                                function_name: func.name.clone(),
                            },
                            func,
                        );
                    }
                    Definition::DataType(data) => {
                        self.data_types.insert(
                            DataTypeKey {
                                module_name: module_name.to_string(),
                                defined_type: data.name.clone(),
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

            self.module_types
                .insert(module_name.to_string(), typed_ast.type_info);
        }

        self.is_stdlib_setup = true
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
                    success: !eval_result.failed(test.can_error),
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
