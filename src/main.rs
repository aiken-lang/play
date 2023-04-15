use leptos::*;
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions},
    sys::editor::BuiltinTheme,
};

#[component]
fn CodeEditor(cx: Scope, initial_value: i32) -> impl IntoView {
    let node_ref = create_node_ref(cx);
    let (editor, set_editor) = create_signal(cx, None);

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

    view! { cx,
        <div _ref=node_ref>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <p>
                <CodeEditor initial_value=0 />
            </p>
        }
    })
}
