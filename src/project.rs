use crate::{
    compiler_error::CompilerError,
    vendor::{fuzz, stdlib},
};
use aiken_lang::{
    ast::{
        DataTypeKey, Definition, FunctionAccessKey, ModuleKind, OnTestFailure, TraceLevel, Tracing,
        TypedDataType, TypedFunction, TypedModule, TypedTest, TypedValidator, UntypedModule,
    },
    builtins,
    expr::TypedExpr,
    format::Formatter,
    gen_uplc::CodeGenerator,
    line_numbers::LineNumbers,
    parser,
    parser::{error::ParseError, extra::ModuleExtra},
    plutus_version::PlutusVersion,
    test_framework::{self, Test},
    tipo::{error::Warning, Type, TypeInfo},
    utils, IdGenerator,
};
use indexmap::IndexMap;
use leptos::{log, SignalSet, SignalUpdate, WriteSignal};
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    path::PathBuf,
    rc::Rc,
};
use supports_color::Stream::Stderr;
use uplc::{
    ast::{DeBruijn, Program},
    machine::cost_model::ExBudget,
    PlutusData,
};

const KIND: ModuleKind = ModuleKind::Validator;
const NAME: &str = "play";
const PLUTUS_VERSION: PlutusVersion = PlutusVersion::V3;
const PROPERTY_MAX_SUCCESS: usize = 30;
const TRACING: Tracing = Tracing::All(TraceLevel::Verbose);

#[derive(Clone)]
pub struct TestResult {
    pub name: String,
    pub success: bool,
    pub logs: Vec<String>,
    pub labels: Vec<(String, String)>,
    pub meta: TestResultMeta,
}

#[derive(Clone)]
pub enum TestResultMeta {
    ExBudget(ExBudget),
    Iterations(usize),
}

#[derive(Clone)]
pub struct Project {
    id_gen: IdGenerator,
    module_types: HashMap<String, TypeInfo>,
    functions: IndexMap<FunctionAccessKey, TypedFunction>,
    constants: IndexMap<FunctionAccessKey, TypedExpr>,
    data_types: IndexMap<DataTypeKey, TypedDataType>,
    module_sources: HashMap<String, (String, LineNumbers)>,
    dependencies: BTreeSet<String>,
}

impl Project {
    pub fn new() -> Rc<RefCell<Self>> {
        let id_gen = IdGenerator::new();

        let mut module_types = HashMap::new();
        module_types.insert("aiken".to_string(), builtins::prelude(&id_gen));
        module_types.insert("aiken/builtin".to_string(), builtins::plutus(&id_gen));

        let functions = builtins::prelude_functions(&id_gen, &module_types);
        let data_types = builtins::prelude_data_types(&id_gen);
        let constants = IndexMap::new();

        RefCell::new(Project {
            id_gen,
            module_types,
            functions,
            constants,
            data_types,
            module_sources: HashMap::new(),
            dependencies: BTreeSet::new(),
        })
        .into()
    }

    pub fn package_name(&self) -> String {
        format!("aiken-lang/{}", NAME)
    }

    pub fn parse(
        &self,
        source_code: &str,
    ) -> Result<(UntypedModule, ModuleExtra), Vec<ParseError>> {
        let (mut ast, extra) = parser::module(source_code, KIND)?;
        ast.name = NAME.to_string();
        Ok((ast, extra))
    }

    pub fn build(
        &mut self,
        source_code: &str,
        set_validators: WriteSignal<Vec<(usize, String, String)>>,
        set_warnings: WriteSignal<Vec<(usize, Warning)>>,
        set_errors: WriteSignal<Vec<(usize, CompilerError)>>,
        set_test_results: WriteSignal<Vec<(usize, TestResult)>>,
    ) {
        if !self.dependencies.contains("stdlib") {
            self.setup_dependency("stdlib", stdlib::modules(), &stdlib::MODULES_SEQUENCE[..]);
            self.dependencies.insert("stdlib".to_string());
        }

        if !self.dependencies.contains("fuzz") {
            self.setup_dependency("fuzz", fuzz::modules(), &fuzz::MODULES_SEQUENCE[..]);
            self.dependencies.insert("fuzz".to_string());
        }

        match self.parse(source_code) {
            Ok((ast, _extra)) => {
                let mut warnings = vec![];
                match ast.infer(
                    &self.id_gen,
                    KIND,
                    &self.package_name(),
                    &self.module_types,
                    TRACING,
                    &mut warnings,
                    None,
                ) {
                    Ok(ast) => {
                        // Register module sources for an easier access later.
                        self.module_sources.insert(
                            NAME.to_string(),
                            (source_code.to_string(), LineNumbers::new(source_code)),
                        );

                        // Register the types from this module so they can be
                        // imported into other modules.
                        self.module_types
                            .insert(NAME.to_string(), ast.type_info.clone());

                        // Register function definitions & data-types for easier access later.
                        ast.register_definitions(
                            &mut self.functions,
                            &mut self.constants,
                            &mut self.data_types,
                        );

                        // Run all tests
                        self.run_tests(&ast, set_test_results);

                        let mut generator = self.new_generator();

                        for (index, validator) in
                            self.collect_validators(&ast).into_iter().enumerate()
                        {
                            let program = generator.generate(validator, NAME);
                            let program: Program<DeBruijn> = program.try_into().unwrap();
                            let program = program.to_hex().unwrap();
                            set_validators.update(|v| {
                                v.push((index, validator.name.clone(), program.clone()))
                            });
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

    pub fn collect_validators<'a>(&'_ self, ast: &'a TypedModule) -> Vec<&'a TypedValidator> {
        ast.definitions()
            .filter_map(|def| match def {
                Definition::Validator(validator) => Some(validator),
                Definition::Test { .. }
                | Definition::ModuleConstant { .. }
                | Definition::Fn { .. }
                | Definition::TypeAlias { .. }
                | Definition::DataType { .. }
                | Definition::Use { .. } => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn collect_tests<'a>(&'_ self, ast: &'a TypedModule) -> Vec<&'a TypedTest> {
        ast.definitions()
            .filter_map(|def| match def {
                Definition::Test(test) => Some(test),
                Definition::ModuleConstant { .. }
                | Definition::Validator { .. }
                | Definition::Fn { .. }
                | Definition::TypeAlias { .. }
                | Definition::DataType { .. }
                | Definition::Use { .. } => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn new_generator(&'_ self) -> CodeGenerator<'_> {
        CodeGenerator::new(
            PLUTUS_VERSION,
            utils::indexmap::as_ref_values(&self.functions),
            utils::indexmap::as_ref_values(&self.constants),
            utils::indexmap::as_ref_values(&self.data_types),
            utils::indexmap::as_str_ref_values(&self.module_types),
            utils::indexmap::as_str_ref_values(&self.module_sources),
            TRACING,
        )
    }

    fn run_tests(
        &self,
        ast: &TypedModule,
        set_test_results: WriteSignal<Vec<(usize, TestResult)>>,
    ) {
        let mut generator = self.new_generator();
        let mut rng = fastrand::Rng::new();
        for (index, test) in self.collect_tests(ast).into_iter().enumerate() {
            let test = Test::from_function_definition(
                &mut generator,
                test.to_owned(),
                NAME.to_string(),
                PathBuf::new(),
            );

            set_test_results.update(|t| {
                t.push((
                    index,
                    self.test_result(match test {
                        Test::UnitTest(unit_test) => unit_test.run(&PLUTUS_VERSION),
                        Test::PropertyTest(property_test) => {
                            property_test.run(rng.u32(..), PROPERTY_MAX_SUCCESS, &PLUTUS_VERSION)
                        }
                    }),
                ))
            });
        }
    }

    fn test_result(
        &self,
        result: test_framework::TestResult<(uplc::ast::Constant, Rc<Type>), PlutusData>,
    ) -> TestResult {
        let data_types = utils::indexmap::as_ref_values(&self.data_types);

        let success = result.is_success();

        match result {
            test_framework::TestResult::UnitTestResult(unit_test) => {
                let unit_test = unit_test.reify(&data_types);

                let mut logs = Vec::new();
                if !unit_test.success {
                    let expect_failure = match unit_test.test.on_test_failure {
                        OnTestFailure::FailImmediately => false,
                        OnTestFailure::SucceedEventually | OnTestFailure::SucceedImmediately => {
                            true
                        }
                    };

                    if let Some(assertion) = unit_test.assertion {
                        logs.push(format!(
                            "assertion failure\n{}",
                            assertion.to_string(Stderr, expect_failure)
                        ));
                    }
                }
                logs.extend(unit_test.traces);

                TestResult {
                    name: unit_test.test.name,
                    success,
                    logs,
                    labels: Vec::new(),
                    meta: TestResultMeta::ExBudget(unit_test.spent_budget),
                }
            }
            test_framework::TestResult::PropertyTestResult(prop_test) => {
                let prop_test = prop_test.reify(&data_types);

                let mut logs = Vec::new();
                if let Ok(Some(counterexample)) = prop_test.counterexample {
                    logs.push(format!(
                        "counterexample\n{}",
                        Formatter::new()
                            .expr(&counterexample, false)
                            .to_pretty_string(80)
                    ));
                }

                let mut labels = Vec::new();
                if success {
                    let mut total = 0;
                    let mut pad = 0;
                    for (k, v) in prop_test.labels.iter() {
                        total += v;
                        if k.len() > pad {
                            pad = k.len();
                        }
                    }

                    let mut xs = prop_test.labels.iter().collect::<Vec<_>>();
                    xs.sort_by(|a, b| b.1.cmp(a.1));

                    for (k, v) in xs {
                        labels.push((
                            format!("{}", 100.0 * (*v as f64) / (total as f64)),
                            k.clone(),
                        ));
                    }
                }

                TestResult {
                    name: prop_test.test.name,
                    success,
                    meta: TestResultMeta::Iterations(prop_test.iterations),
                    labels,
                    logs,
                }
            }
        }
    }

    fn setup_dependency(&mut self, context: &str, modules: HashMap<&str, &str>, sequence: &[&str]) {
        for module_name in sequence {
            let module_src = modules.get(module_name).unwrap_or_else(|| {
                panic!("couldn't find sources for '{module_name}' when compiling {context}")
            });
            let (mut ast, _extra) = parser::module(module_src, ModuleKind::Lib).unwrap();

            ast.name = module_name.to_string();

            let mut warnings = vec![];

            let ast = ast
                .infer(
                    &self.id_gen,
                    ModuleKind::Lib,
                    module_name,
                    &self.module_types,
                    Tracing::silent(),
                    &mut warnings,
                    None,
                )
                .map_err(|e| {
                    log!("failed to type-checked {context}: {e}");
                })
                .unwrap();

            ast.register_definitions(
                &mut self.functions,
                &mut self.constants,
                &mut self.data_types,
            );

            self.module_sources.insert(
                module_name.to_string(),
                (module_src.to_string(), LineNumbers::new(module_src)),
            );

            self.module_types
                .insert(module_name.to_string(), ast.type_info);
        }
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
