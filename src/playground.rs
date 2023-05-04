use crate::{
    compiler_error::CompilerError,
    components::prelude::*,
    project::{format, Project, TestResult},
};
use aiken_lang::tipo::error::Warning;
use leptos::*;
use leptos_router::*;
use monaco::api::TextModel;

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

    let (share, set_share) = create_signal(cx, false);
    let toggle_share = move |_| set_share.update(|visible| *visible = !*visible);
    let hide_share = move |_| set_share.set(false);

    view! { cx,
        <Router>
            <Header checking=checking on_format=run_format on_check=run_check on_share=toggle_share />
            <Share display=share editor=editor on_close=hide_share on_cancel=hide_share />
            <div class="flex grow">
                <Navigation/>
                <CodeEditor set_editor=set_editor/>
                <Output test_results=test_results warnings=warnings errors=errors/>
            </div>
        </Router>
    }
}
