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

    let check_action = create_action(cx, move |_: &()| {
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

        let project = project.clone();

        async move {
            project
                .borrow_mut()
                .build(&text, set_warnings, set_errors, set_test_results);
        }
    });

    let run_check = move |_ev: web_sys::MouseEvent| check_action.dispatch(());

    let (share, set_share) = create_signal(cx, false);
    let toggle_share = move |_| set_share.update(|visible| *visible = !*visible);
    let hide_share = move |_| set_share.set(false);

    view! { cx,
        <Router>
            <Header
                checking=check_action
                on_format=run_format
                on_check=run_check
                on_share=toggle_share
            />
            <Share display=share editor=editor on_close=hide_share on_cancel=hide_share/>
            <div class="grow hidden md:flex">
                <Navigation/>
                <CodeEditor set_editor=set_editor/>
                <Output test_results=test_results warnings=warnings errors=errors/>
            </div>
            <div class="grow text-left md:hidden text-gray-0 pt-4 px-4">
                "The playground is not optimized for small screens. You're probably using a mobile device, please come back on desktop to try out the playground."
                <ul class="pt-4 px-4 flex flex-col gap-y-2 underline text-blue-40 list-disc">
                    <li>
                        <a target="_blank" href="https://aiken-lang.org/installation-instructions">
                            "Install"
                        </a>
                    </li>
                    <li>
                        <a target="_blank" href="https://discord.gg/Vc3x8N9nz2">
                            "Discord"
                        </a>
                    </li>
                    <li>
                        <a target="_blank" href="https://github.com/aiken-lang">
                            "Website"
                        </a>
                    </li>
                </ul>
            </div>
        </Router>
    }
}
