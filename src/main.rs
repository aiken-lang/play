use leptos::*;
use leptos_icons::*;
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions},
    sys::editor::BuiltinTheme,
};

#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="flex justify-between items-center p-3">
            <div class="flex items-center gap-x-3">
                <img
                    class="w-8 h-8"
                    type="image/png"
                    src="https://raw.githubusercontent.com/aiken-lang/branding/main/assets/icon.png"
                />
                <span class="text-white text-lg font-semibold">"AIKEN PLAYGROUND"</span>
            </div>
            <div class="flex gap-x-4">
                <button class="bg-gray-40 flex justify-center items-center gap-x-2 text-sm font-semibold text-white w-24 py-1.5 rounded">
                    <LeptosIcon icon=FaIcon::FaPlaySolid />
                    "Check"
                </button>
                <button class="bg-share-button flex justify-center items-center gap-x-2 text-sm font-semibold text-white px-3 py-1.5 rounded">
                    <LeptosIcon icon=FaIcon::FaShareSolid />
                    "Share"
                </button>
            </div>
        </header>
    }
}

#[component]
fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx, <div></div> }
}

#[component]
fn CodeEditor(cx: Scope) -> impl IntoView {
    let node_ref = create_node_ref(cx);
    let (_editor, set_editor) = create_signal(cx, None);

    node_ref.on_load(cx, move |element| {
        use wasm_bindgen::JsCast;

        let div_element: &web_sys::HtmlDivElement = &element;
        let html_element = div_element.unchecked_ref::<web_sys::HtmlElement>();

        let options = CodeEditorOptions::default()
            .with_language("rust".to_string())
            .with_value("main.rs".to_string())
            .with_builtin_theme(BuiltinTheme::VsDark)
            .with_new_dimension(1000, 1000);

        let e = CodeEditorModel::create(html_element, Some(options));

        set_editor(Some(e))
    });

    view! { cx, <div _ref=node_ref></div> }
}

#[component]
fn Tests(cx: Scope) -> impl IntoView {
    view! { cx, <div></div> }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <Header/>
            <div>
                <Navigation/>
                <CodeEditor/>
                <Tests/>
            </div>
        }
    })
}
