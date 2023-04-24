use std::error::Error;
use std::{cell::RefCell, rc::Rc};

use aiken_lang::tipo::error::Warning;
use leptos::*;
use leptos_icons::*;
use miette::Diagnostic;
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions},
    sys::editor::BuiltinTheme,
};

mod compiler_error;
mod project;

use project::Project;

use crate::compiler_error::CompilerError;
use crate::project::TestResult;

const INITIAL_CONTENT: &str = r#"use aiken/builtin

validator {
  fn hello(_d: Data, _r: Data, ctx: Data) -> Bool {
    True
  }
}

fn map(list: List<a>, apply: fn(a) -> b) -> List<b> {
  when list is {
    [] -> []
    [x, ..xs] -> [apply(x), ..map(xs, apply)]
  }
}

test thing() {
  [2, 3, @"foo"] == map([1, 2, 3], fn(x) { x + 1 })
}

test other() {
  builtin.add_integer(1, 2) == 3
}
"#;

type ModelCell = Rc<RefCell<Option<CodeEditorModel>>>;

#[component]
fn Header<F>(cx: Scope, on_check: F) -> impl IntoView
where
    F: Fn(web_sys::MouseEvent) + 'static,
{
    view! { cx,
        <header class="flex justify-between items-center p-3 border-b border-solid border-gray-40">
            <div class="flex items-center gap-x-3">
                <img
                    class="w-8 h-8"
                    type="image/png"
                    src="https://raw.githubusercontent.com/aiken-lang/branding/main/assets/icon.png"
                />
                <span class="text-white text-lg font-semibold">"AIKEN PLAYGROUND"</span>
            </div>
            <div class="flex gap-x-4">
                <button
                    on:click=on_check
                    class="bg-gray-40 flex justify-center items-center gap-x-2 text-sm font-semibold text-white w-24 py-1.5 rounded"
                >
                    <LeptosIcon icon=RiIcon::RiPlayMediaFill/>
                    "Check"
                </button>
                <button class="bg-share-button flex justify-center items-center gap-x-2 text-sm font-semibold text-white px-3 py-1.5 rounded">
                    <LeptosIcon icon=RiIcon::RiShareForwardSystemFill/>
                    "Share"
                </button>
            </div>
        </header>
    }
}

#[component]
fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col justify-between p-4 text-gray-0 border-r border-solid border-gray-40">
            <LeptosIcon icon=RiIcon::RiSettings3SystemFill class="w-6 h-6"/>
            <div class="flex flex-col gap-y-7">
                <a target="_blank" href="https://aiken-lang.org/installation-instructions">
                    <LeptosIcon icon=RiIcon::RiBook2DocumentFill class="w-6 h-6"/>
                </a>
                <a target="_blank" href="https://discord.gg/Vc3x8N9nz2">
                    <LeptosIcon icon=RiIcon::RiDiscordLogosFill class="w-6 h-6"/>
                </a>
                <a target="_blank" href="https://github.com/aiken-lang">
                    <LeptosIcon icon=RiIcon::RiGithubLogosFill class="w-6 h-6"/>
                </a>
            </div>
        </div>
    }
}

#[component]
fn CodeEditor(cx: Scope, set_editor: WriteSignal<ModelCell>) -> impl IntoView {
    let node_ref = create_node_ref(cx);

    node_ref.on_load(cx, move |element| {
        use wasm_bindgen::JsCast;

        let div_element: &web_sys::HtmlDivElement = &element;
        let html_element = div_element.unchecked_ref::<web_sys::HtmlElement>();

        let options = CodeEditorOptions::default()
            .with_language("rust".to_string())
            .with_value(INITIAL_CONTENT.to_string())
            .with_builtin_theme(BuiltinTheme::VsDark)
            .with_new_dimension(784, 960);

        let e = CodeEditorModel::create(html_element, Some(options));

        set_editor.update(|editor| {
            editor.replace(Some(e));
        });
    });

    view! { cx, <div _ref=node_ref></div> }
}

#[component]
fn Output(
    cx: Scope,
    test_results: ReadSignal<Vec<(usize, TestResult)>>,
    warnings: ReadSignal<Vec<(usize, Warning)>>,
    errors: ReadSignal<Vec<(usize, CompilerError)>>,
) -> impl IntoView {
    view! { cx,
        <div class="p-4 overflow-y-scroll flex grow flex-col gap-y-11">
            <div>
                <div class="flex items-center mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Tests"
                    <span class="py-1 px-2 bg-gray-90 rounded text-sm font-semibold">
                        {move || test_results.get().len()}
                    </span>
                </div>
                <ul class="flex flex-col gap-y-4">
                    <For
                        each=move || test_results.get()
                        key=|test_result| test_result.0
                        view=move |cx, (_, test_result)| {
                            let pass_or_fail = if test_result.success { "PASS" } else { "FAIL" };
                            view! { cx,
                                <li class="bg-blue-0 output-item rounded-lg pl-1 overflow-hidden">
                                    <div class="flex items-center justify-between bg-gray-80 pr-2 pt-2 pb-2 pl-3">
                                        <div class="flex items-center gap-x-4">
                                            <span class="text-blue-40 font-semibold text-xs">{pass_or_fail}</span>
                                            <span class="text-white text-sm font-normal">{test_result.name}</span>
                                        </div>
                                        <div class="flex items-center justify-start gap-x-9 text-gray-70 mr-9 text-sm font-normal">
                                            <div class="flex items-center gap-x-1">
                                                <LeptosIcon icon=RiIcon::RiCpuDeviceLine class="w-3.5 h-3.5"/>
                                                {test_result.spent_budget.mem}
                                            </div>
                                            <div class="flex items-center gap-x-1">
                                                <LeptosIcon icon=RiIcon::RiDatabase2DeviceLine class="w-3.5 h-3.5"/>
                                                {test_result.spent_budget.cpu}
                                            </div>
                                        </div>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
            <div>
                <div class="flex items-center mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Errors"
                    <span class="py-1 px-2 bg-gray-90 rounded text-sm font-semibold">
                        {move || errors.get().len()}
                    </span>
                </div>
                <ul class="flex flex-col gap-y-4">
                    <For
                        each=move || errors.get()
                        key=|error| error.0
                        view=move |cx, (_, error)| {
                            let message = error.message();
                            let code = error.code();
                            let help = error
                                .help()
                                .map(|help_message| {
                                    view! { cx,
                                        <div class="text-gray-70 text-sm flex gap-x-3 items-center">
                                            <span class="text-blue-40 text-xs">"HELP"</span>
                                            {help_message}
                                        </div>
                                    }
                                });
                            view! { cx,
                                <li class="bg-error-gradient output-item rounded-lg pl-1 overflow-hidden">
                                    <div class="bg-gray-80 pr-2 pt-2 pb-4 pl-3 flex flex-col gap-y-4">
                                        <div class="flex items-center gap-x-3.5 text-pink-0">
                                            <LeptosIcon icon=RiIcon::RiErrorWarningSystemLine class="w-3.5 h-3.5"/>
                                            <span class="text-sm">{code}</span>
                                        </div>
                                        <div class="text-gray-70 text-sm">{message}</div>
                                        {help}
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
            <div>
                <div class="flex items-center mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Warnings"
                    <span class="py-1 px-2 bg-gray-90 rounded text-sm font-semibold">
                        {move || warnings.get().len()}
                    </span>
                </div>
                <ul class="flex flex-col gap-y-4">
                    <For
                        each=move || warnings.get()
                        key=|warning| warning.0
                        view=move |cx, (_, warning)| {
                            let message = warning
                                .source()
                                .map_or_else(|| warning.to_string(), |e| e.to_string());
                            let code = warning.code().map(|c| c.to_string());
                            let help = warning
                                .help()
                                .map(|h| {
                                    view! { cx,
                                        <div class="text-gray-70 text-sm flex gap-x-3 items-center">
                                            <span class="text-blue-40 text-xs">"HELP"</span>
                                            {h.to_string()}
                                        </div>
                                    }
                                });
                            view! { cx,
                                <li class="bg-warning-gradient output-item rounded-lg pl-1 overflow-hidden">
                                    <div class="bg-gray-80 pr-2 pt-2 pb-4 pl-3 flex flex-col gap-y-4">
                                        <div class="flex items-center gap-x-3.5 text-orange-0">
                                            <LeptosIcon icon=RiIcon::RiAlertSystemLine class="w-3.5 h-3.5"/>
                                            <span class="text-sm">{code}</span>
                                        </div>
                                        <div class="text-gray-70 text-sm">{message}</div>
                                        {help}
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let project = Project::new();
    let (editor, set_editor) = create_signal(cx, ModelCell::default());
    let (test_results, set_test_results) = create_signal::<Vec<(usize, TestResult)>>(cx, vec![]);
    let (warnings, set_warnings) = create_signal::<Vec<(usize, Warning)>>(cx, vec![]);
    let (errors, set_errors) = create_signal::<Vec<(usize, CompilerError)>>(cx, vec![]);

    let run_check = move |_ev: web_sys::MouseEvent| {
        let text = editor
            .get()
            .borrow()
            .as_ref()
            .unwrap()
            .get_model()
            .unwrap()
            .get_value();

        log!("yooo");
        set_test_results.set(vec![]);

        project.build(&text, set_warnings, set_errors, set_test_results);
    };

    view! { cx,
        <Header on_check=run_check/>
        <div class="flex grow">
            <Navigation/>
            <CodeEditor set_editor=set_editor/>
            <Output test_results=test_results warnings=warnings errors=errors/>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}
