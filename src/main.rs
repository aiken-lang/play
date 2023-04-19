use std::{cell::RefCell, rc::Rc};

use leptos::*;
use leptos_icons::*;
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions},
    sys::editor::BuiltinTheme,
};

mod project;
use project::Project;

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
  [2, 3, 4] == map([1, 2, 3], fn(x) { x + 1 })
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
        <div class="flex flex-col justify-between p-3.5 text-gray-0 border-r border-solid border-gray-40">
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
fn Tests(cx: Scope) -> impl IntoView {
    view! { cx, <div></div> }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let project = Project::new();
    let (editor, set_editor) = create_signal(cx, ModelCell::default());

    let run_check = move |_ev: web_sys::MouseEvent| {
        let text = editor
            .get()
            .borrow()
            .as_ref()
            .unwrap()
            .get_model()
            .unwrap()
            .get_value();

        project.build(&text);
    };

    view! { cx,
        <Header on_check=run_check/>
        <div class="flex grow">
            <Navigation/>
            <CodeEditor set_editor=set_editor/>
            <Tests/>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}
