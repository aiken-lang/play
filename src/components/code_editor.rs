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
    "keywords": [
        "if", "else", "when", "is", "fn", "use",
        "let", "pub", "type", "opaque", "const",
        "todo", "expect", "check", "test", "trace",
        "error", "validator"
    ],
    "operators": [
        "->", "|>", "..", "<=", ">=", "==", "!=", "<", ">", "&&", "||",
        "|", "+", "-", "/", "*", "%", "="
    ],
    "digits": "\\d+(_+\\d+)*",
	"octaldigits": "[0-7]+(_+[0-7]+)*",
	"binarydigits": "[0-1]+(_+[0-1]+)*",
	"hexdigits": "[0-9a-fA-F]+(_+[0-9a-fA-F]+)*",
    "tokenizer": {
        "root": [
            ["[a-z_$][\\w$]*", {
				"cases": {
					"@keywords": "keyword",
					"@default": "identifier"
				}
	        }],
            ["\/\/.*", "comment"],
            ["[A-Z][\\w\\$]*", "type.identifier"],
            ["[a-z][\\w\\$]*", "identifier"],
            ["0[xX](@hexdigits)", "number.hex"],
			["0[oO]?(@octaldigits)", "number.octal"],
			["0[bB](@binarydigits)", "number.binary"],
			["(@digits)", "number"]
        ]
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

    set_monarch_tokens_provider(
        "aiken",
        &JSON::parse(HIGHLIGHTING)
            .map_err(|e| {
                log!("{:#?}", e);
                e
            })
            .unwrap(),
    );

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
