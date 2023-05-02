use std::io::{Cursor, Write};

use aiken_lang::tipo::error::Warning;
use base64::Engine;
use leptos::*;
use leptos_router::*;
use monaco::api::TextModel;

use crate::{
    compiler_error::CompilerError,
    components::prelude::*,
    project::{format, Project, TestResult},
};

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    let project = Project::new();

    let (checking, set_checking) = create_signal(cx, false);

    let (editor, set_editor) = create_signal(cx, ModelCell::default());
    let (test_results, set_test_results) = create_signal::<Vec<(usize, TestResult)>>(cx, vec![]);
    let (warnings, set_warnings) = create_signal::<Vec<(usize, Warning)>>(cx, vec![]);
    let (errors, set_errors) = create_signal::<Vec<(usize, CompilerError)>>(cx, vec![]);

    let run_format = move |_ev: web_sys::MouseEvent| {
        let text = editor
            .get()
            .borrow()
            .as_ref()
            .unwrap()
            .get_model()
            .unwrap()
            .get_value();

        if let Some(formatted) = format(&text, set_errors) {
            let new_text = TextModel::create(&formatted, Some("aiken"), None).unwrap();

            editor.get().borrow().as_ref().unwrap().set_model(&new_text);
        };
    };

    let run_check = move |_ev: web_sys::MouseEvent| {
        let text = editor
            .get()
            .borrow()
            .as_ref()
            .unwrap()
            .get_model()
            .unwrap()
            .get_value();

        set_checking.set(true);
        set_test_results.set(vec![]);
        set_warnings.set(vec![]);
        set_errors.set(vec![]);

        project.build(&text, set_warnings, set_errors, set_test_results);
        set_checking.set(false);
    };

    let run_share = move |_ev: web_sys::MouseEvent| {
        let text = editor
            .get()
            .borrow()
            .as_ref()
            .unwrap()
            .get_model()
            .unwrap()
            .get_value();

        let compressed_code = [0u8; 4096];
        let cursor = Cursor::new(compressed_code);
        let mut writer = brotli::CompressorWriter::new(cursor, 4096, 11, 22);

        writer.write_all(text.as_bytes()).ok();

        let cursor = writer.into_inner();

        let bytes_written = cursor.position() as usize;

        let bytes = cursor.into_inner();

        let code = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes[..bytes_written]);

        let share_url = format!("https://play.aiken-lang.org?code={}", code);

        let _ = window()
            .navigator()
            .clipboard()
            .unwrap()
            .write_text(&share_url);
    };

    view! { cx,
        <Router>
            <Header checking=checking on_format=run_format on_check=run_check on_share=run_share/>
            <div class="flex grow">
                <Navigation/>
                <CodeEditor set_editor=set_editor/>
                <Output test_results=test_results warnings=warnings errors=errors/>
            </div>
        </Router>
    }
}
