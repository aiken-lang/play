use aiken_lang::tipo::error::Warning;
use leptos::*;

use crate::{
    compiler_error::CompilerError,
    components::prelude::*,
    project::{Project, TestResult},
};

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
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

        set_test_results.set(vec![]);
        set_warnings.set(vec![]);
        set_errors.set(vec![]);

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
