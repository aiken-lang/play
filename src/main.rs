use leptos::*;
use monaco::api::CodeEditor as CodeEditorModel;
use wasm_bindgen::JsCast;

#[component]
fn CodeEditor(cx: Scope, initial_value: i32) -> impl IntoView {
    let node_ref = create_node_ref(cx);
    let (editor, set_editor) = create_signal(cx, None);

    node_ref.on_load(cx, |element| {
        let e = CodeEditorModel::create(&element, None);

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
