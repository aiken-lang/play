use std::{
    cell::RefCell,
    io::{Cursor, Read},
    rc::Rc,
};

use base64::Engine;
use js_sys::JSON;
use leptos::*;
use leptos_router::*;
use monaco::{
    api::{CodeEditor as CodeEditorModel, CodeEditorOptions},
    sys::{
        editor::BuiltinTheme,
        languages::{register, set_monarch_tokens_provider, ILanguageExtensionPoint},
    },
};
use wasm_bindgen::JsValue;

const INITIAL_CONTENT: &str = r#"use aiken/collection/list
use aiken/fuzz
use cardano/assets
use cardano/transaction.{Transaction}

validator main {
  mint(redeemer: List<Int>, policy_id: ByteArray, self: Transaction) {
    trace @"minting": policy_id, @"with", redeemer

    let quantities =
      self.mint
        |> assets.flatten
        |> list.map(fn(t) { t.3rd })

    (quicksort(redeemer) == quantities)?
  }

  else(_) {
    fail
  }
}

fn quicksort(xs: List<Int>) -> List<Int> {
  when xs is {
    [] ->
      []
    [p, ..tail] -> {
      let before =
        tail
          |> list.filter(fn(x) { x < p })
          |> quicksort
      let after =
        tail
          |> list.filter(fn(x) { x >= p })
          |> quicksort
      list.concat(before, [p, ..after])
    }
  }
}

test quicksort_0() {
  quicksort([]) == []
}

test quicksort_1() {
  quicksort([3, 2, 1, 4]) == [1, 2, 3, 4]
}

test quicksort_2() {
  quicksort([1, 2, 3, 4]) == [1, 2, 3, 4]
}

test quicksort_prop(xs via fuzz.list(fuzz.int())) {
  fuzz.label_when(list.is_empty(xs), @"empty", @"non-empty")
  quicksort(xs) == quicksort(quicksort(xs))
}"#;

const HIGHLIGHTING: &str = r#"
{
    "keywords": [
        "if", "else", "when", "is", "fn", "use",
        "let", "pub", "type", "opaque", "const",
        "todo", "expect", "test", "trace", "fail",
        "once", "validator", "and", "or", "via"
    ],
    "operators": [
        "->", "|>", "..", "<=", ">=", "==", "!=", "<", ">", "&&", "||",
        "|", "+", "-", "/", "*", "%", "=", "<-", "?"
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
			["(@digits)", "number"],
            ["\"([^\"\\\\]|\\\\.)*$", "string.invalid"],
            ["\"([^\"\\\\]|\\\\.)*\"", "string"]
        ]
    }
}
"#;

pub type ModelCell = Rc<RefCell<Option<CodeEditorModel>>>;

#[derive(Debug, Params, PartialEq)]
struct CodeQuery {
    code: Option<String>,
}

#[component]
pub fn CodeEditor(cx: Scope, set_editor: WriteSignal<ModelCell>) -> impl IntoView {
    use wasm_bindgen::JsCast;

    let query = use_query::<CodeQuery>(cx);

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

        let initial_content = query.with(|query| {
            query
                .as_ref()
                .ok()
                .and_then(|query| query.code.clone())
                .and_then(|code| {
                    let mut compressed_code = [0u8; 4096];
                    let compressed_code_byte_len = base64::engine::general_purpose::URL_SAFE_NO_PAD
                        .decode_slice(code, &mut compressed_code)
                        .ok()?;

                    let cursor = Cursor::new(compressed_code[..compressed_code_byte_len].to_vec());
                    let mut decompressor =
                        brotli::Decompressor::new(cursor, compressed_code_byte_len);

                    let mut code_bytes = [0u8; 4096];
                    let code_bytes_len = decompressor.read(&mut code_bytes).ok()?;

                    String::from_utf8(code_bytes[..code_bytes_len].to_vec()).ok()
                })
                .unwrap_or(INITIAL_CONTENT.to_string())
        });

        let options = CodeEditorOptions::default()
            .with_language("aiken".to_string())
            .with_value(initial_content)
            .with_builtin_theme(BuiltinTheme::VsDark)
            .with_automatic_layout(true);

        let e = CodeEditorModel::create(html_element, Some(options));

        set_editor.update(|editor| {
            editor.replace(Some(e));
        });
    });

    view! { cx, <div class="w-1/2" _ref=node_ref></div> }
}
