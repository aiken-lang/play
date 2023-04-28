use std::{cell::RefCell, rc::Rc};

use js_sys::JSON;
use leptos::*;
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions},
    sys::{
        editor::BuiltinTheme,
        languages::{register, set_monarch_tokens_provider, ILanguageExtensionPoint},
    },
};
use wasm_bindgen::JsValue;

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

const HIGHLIGHTING: &str = r#"
{
  "tokenizer": {
    "root": [
      ["\/\/.*", "comment"],
      ["\\b(if|else|when|is|fn|use|let|pub|type|opaque|const|todo|expect|check|test|trace|error|validator)\\b", "keyword"],
      ["->", "operator"],
      ["\\|>", "operator"],
      ["\\.\\.", "operator"],
      ["(<=|>=|==|!=|<|>)", "operator"],
      ["(&&|\\|\\|)", "operator"],
      ["\\|", "operator"],
      ["(\\+|\\-|/|\\*|%)", "operator"],
      ["=", "operator"],
      ["\"", { "token": "string", "nextEmbedded": "allowEmbeddedDoubleQuote", "next": "@string" }],
      ["\\b(True|False)\\b", "constant.language.boolean"],
      ["\\b0b[0-1]+\\b", "constant.numeric.binary"],
      ["\\b0o[0-7]+\\b", "constant.numeric.octal"],
      ["\\b0x[[:xdigit:]]+\\b", "constant.numeric.hexadecimal"],
      ["\\b[[:digit:]][[:digit:]_]*(\\.[[:digit:]]*)?\\b", "constant.numeric.decimal"],
      ["[[:upper:]][[:word:]]*", "entity.name.type"],
      ["\\b([[:lower:]][[:word:]]*)([[:space:]]*)?\\(", "entity.name.function"],
      ["\\b([[:lower:]][[:word:]]*):\\s", "variable.parameter"],
      ["\\b([[:lower:]][[:word:]]*):", "entity.name.namespace"]
    ],
    "string": [
      ["\\\\.", "string.escape"],
      ["\"", { "token": "string", "nextEmbedded": "@pop", "next": "@pop" }],
      ["[^\"]", "string"]
    ]
  },
  "embeddedLanguages": {
    "allowEmbeddedDoubleQuote": {
      "embeds": "none"
    }
  }
}
"#;

pub type ModelCell = Rc<RefCell<Option<CodeEditorModel>>>;

#[component]
pub fn CodeEditor(cx: Scope, set_editor: WriteSignal<ModelCell>) -> impl IntoView {
    use wasm_bindgen::JsCast;

    let node_ref = create_node_ref(cx);

    let language_extension: ILanguageExtensionPoint = js_sys::Object::new().unchecked_into();

    language_extension.set_id("aiken");

    let extensions = js_sys::Array::new();

    extensions.push(&JsValue::from_str(".ak"));

    language_extension.set_extensions(Some(&extensions));

    register(&language_extension);

    set_monarch_tokens_provider("aiken", &JSON::parse(HIGHLIGHTING).unwrap());

    node_ref.on_load(cx, move |element| {
        let div_element: &web_sys::HtmlDivElement = &element;
        let html_element = div_element.unchecked_ref::<web_sys::HtmlElement>();

        let options = CodeEditorOptions::default()
            .with_language("aiken".to_string())
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
