use std::{cell::RefCell, rc::Rc};

use leptos::*;
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions},
    sys::editor::BuiltinTheme,
};

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

pub type ModelCell = Rc<RefCell<Option<CodeEditorModel>>>;

#[component]
pub fn CodeEditor(cx: Scope, set_editor: WriteSignal<ModelCell>) -> impl IntoView {
    let node_ref = create_node_ref(cx);

    node_ref.on_load(cx, move |element| {
        use wasm_bindgen::JsCast;

        let div_element: &web_sys::HtmlDivElement = &element;
        let html_element = div_element.unchecked_ref::<web_sys::HtmlElement>();

        let options = CodeEditorOptions::default()
            .with_language("rust".to_string())
            .with_value(INITIAL_CONTENT.to_string())
            .with_builtin_theme(BuiltinTheme::VsDark)
            .with_automatic_layout(true);

        let e = CodeEditorModel::create(html_element, Some(options));

        set_editor.update(|editor| {
            editor.replace(Some(e));
        });
    });

    view! { cx, <div class="w-1/2" _ref=node_ref></div> }
}
