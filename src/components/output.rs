use crate::{
    compiler_error::CompilerError,
    project::{TestResult, TestResultMeta},
};
use aiken_lang::tipo::error::Warning;
use leptos::*;
use leptos_icons::*;
use miette::Diagnostic;
use std::error::Error;

#[component]
pub fn Output(
    cx: Scope,
    test_results: ReadSignal<Vec<(usize, TestResult)>>,
    warnings: ReadSignal<Vec<(usize, Warning)>>,
    errors: ReadSignal<Vec<(usize, CompilerError)>>,
    validators: ReadSignal<Vec<(usize, String, String)>>,
) -> impl IntoView {
    let test_result_meta_view = |meta: TestResultMeta, scope: Scope| match meta {
        TestResultMeta::ExBudget(budget) => view! { scope,
            <div class="flex items-center justify-start gap-x-9 text-gray-70 mr-9 text-sm font-normal">
                <div class="flex items-center gap-x-1">
                    <Icon icon=RiIcon::RiCpuDeviceLine class="w-3.5 h-3.5"/>
                    {budget.cpu}
                </div>
                <div class="flex items-center gap-x-1">
                    <Icon icon=RiIcon::RiDatabase2DeviceLine class="w-3.5 h-3.5"/>
                    {budget.mem}
                </div>
            </div>
        },
        TestResultMeta::Iterations(iterations) => view! { scope,
            <div class="flex items-center justify-start gap-x-9 text-gray-70 mr-9 text-sm font-normal">
                <div class="flex items-center gap-x-1">
                    <Icon icon=LuIcon::LuDices class="w-3.5 h-3.5"/>
                    {iterations}
                </div>
            </div>
        },
    };

    view! { cx,
        <div class="p-4 overflow-y-scroll flex grow flex-col gap-y-11">
            <div>
                <div class="flex items-center mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Validators"
                    <span class="py-1 px-2 bg-gray-90 rounded text-sm font-semibold">
                        {move || validators.get().len()}
                    </span>
                </div>
                <ul class="flex flex-col gap-y-4">
                    <For
                        each=move || validators.get()
                        key=|validator| validator.0
                        view=move |cx, (_, name, program)| {
                            view! { cx,
                                <li class="bg-blue-0 output-item rounded-lg pl-1 overflow-hidden">
                                    <div class="flex gap-x-4 items-center justify-start bg-gray-80 pr-2 pt-2 pb-2 pl-3">
                                        <div>
                                            <span class="text-blue-40 font-semibold text-xs">{name}</span>
                                        </div>
                                        <div class="flex items-center justify-between text-gray-70 text-sm font-normal w-full">
                                            <input
                                                class="w-full px-2 py-1 text-purple-200 bg-neutral-600 rounded"
                                                type="text"
                                                disabled=true
                                                value=program.clone()
                                            />
                                            <button
                                                class="flex items-center px-2 py-y"
                                                on:click=move |_| {
                                                    let _ = window().navigator().clipboard().write_text(&program);
                                                }
                                            >
                                                <Icon icon=RiIcon::RiClipboardDocumentLine class="w-2 h-full"/>
                                            </button>
                                        </div>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
            <div>
                <div class="flex items-center mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Tests"
                    <span class="py-1 px-2 bg-gray-90 rounded text-sm font-semibold">
                        {move || test_results.get().len()}
                    </span>
                </div>
                <ul class="flex flex-col gap-y-4">
                    <For
                        each=move || test_results.get()
                        key=|test_result| test_result.0
                        view=move |cx, (_, test_result)| {
                            let pass_or_fail = if test_result.success { "PASS" } else { "FAIL" };
                            view! { cx,
                                <li
                                    class="bg-blue-0 output-item rounded-lg pl-1 overflow-hidden"
                                    class:bg-pink=!test_result.success
                                >
                                    <div class="flex items-center justify-between bg-gray-80 pr-2 pt-2 pb-2 pl-3">
                                        <div class="flex items-center gap-x-4">
                                            <span
                                                class="text-blue-40 font-semibold text-xs"
                                                class:text-pink=!test_result.success
                                            >
                                                {pass_or_fail}
                                            </span>
                                            <span class="text-white text-sm font-normal">{test_result.name}</span>
                                        </div>
                                        { test_result_meta_view(test_result.meta, cx) }
                                    </div>
                                    { move || {
                                        if !test_result.logs.is_empty() {
                                            view! { cx,
                                                <div class="flex flex-col items-left bg-gray-80 pr-2 pt-2 pb-2 pl-3 space-y-2">
                                                    {test_result.logs.iter().map(|log| {
                                                        view! { cx, <pre
                                                            class="test-trace text-xs text-gray-70 font-mono"
                                                            class:success=test_result.success>{log}</pre>
                                                        }
                                                    }).collect_view(cx)}
                                                </div>
                                            }
                                        } else {
                                            view! { cx, <div></div> }
                                        }
                                    }}
                                    { move || {
                                        if !test_result.labels.is_empty() {
                                            view! { cx,
                                                <div class="flex flex-col items-left bg-gray-80 pr-2 pb-2 pl-3 space-y-2">
                                                    <div><span class="text-blue-40 font-semibold text-xs">"COVERAGE"</span></div>
                                                    {test_result.labels.iter().map(|(p, l)| {
                                                        view! { cx, <div
                                                            class="text-xs text-gray-70 font-mono flex flex-row gap-x-2"
                                                            class:success=test_result.success>
                                                              <span class="percentage font-semibold">{p}"%"</span>
                                                              <span class="label">{l}</span>
                                                            </div>
                                                        }
                                                    }).collect_view(cx)}
                                                </div>
                                            }
                                        } else {
                                            view! { cx, <div></div> }
                                        }
                                    }}
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
            <div>
                <div class="flex items-center mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Errors"
                    <span class="py-1 px-2 bg-gray-90 rounded text-sm font-semibold">
                        {move || errors.get().len()}
                    </span>
                </div>
                <ul class="flex flex-col gap-y-4">
                    <For
                        each=move || errors.get()
                        key=|error| error.0
                        view=move |cx, (_, error)| {
                            let message = error.message();
                            let code = error.code();
                            let help = error
                                .help()
                                .map(|help_message| {
                                    view! { cx,
                                        <div class="text-gray-70 text-sm flex gap-x-3 items-start">
                                            <span class="text-blue-40 text-xs leading-5">"HELP"</span>
                                            {help_message}
                                        </div>
                                    }
                                });
                            view! { cx,
                                <li class="bg-error-gradient output-item rounded-lg pl-1 overflow-hidden">
                                    <div class="bg-gray-80 pr-2 pt-2 pb-4 pl-3 flex flex-col gap-y-4">
                                        <div class="flex items-center gap-x-3.5 text-pink">
                                            <Icon icon=RiIcon::RiErrorWarningSystemLine class="w-3.5 h-3.5"/>
                                            <span class="text-sm">{code}</span>
                                        </div>
                                        <div class="text-gray-70 text-sm">{message}</div>
                                        {help}
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
            <div>
                <div class="flex items-center mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Warnings"
                    <span class="py-1 px-2 bg-gray-90 rounded text-sm font-semibold">
                        {move || warnings.get().len()}
                    </span>
                </div>
                <ul class="flex flex-col gap-y-4">
                    <For
                        each=move || warnings.get()
                        key=|warning| warning.0
                        view=move |cx, (_, warning)| {
                            let message = warning
                                .source()
                                .map_or_else(|| warning.to_string(), |e| e.to_string());
                            let code = warning.code().map(|c| c.to_string());
                            let help = warning
                                .help()
                                .map(|h| {
                                    view! { cx,
                                        <div class="text-gray-70 text-sm flex gap-x-3 items-start">
                                            <span class="text-blue-40 text-xs leading-5">"HELP"</span>
                                            {h.to_string()}
                                        </div>
                                    }
                                });
                            view! { cx,
                                <li class="bg-warning-gradient output-item rounded-lg pl-1 overflow-hidden">
                                    <div class="bg-gray-80 pr-2 pt-2 pb-4 pl-3 flex flex-col gap-y-4">
                                        <div class="flex items-center gap-x-3.5 text-orange-0">
                                            <Icon icon=RiIcon::RiAlertSystemLine class="w-3.5 h-3.5"/>
                                            <span class="text-sm">{code}</span>
                                        </div>
                                        <div class="text-gray-70 text-sm">{message}</div>
                                        {help}
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
            <div class="mt-auto">
                <div class="flex flex-col items-left mb-5 text-gray-40 gap-x-2 text-lg font-normal">
                    "Available modules"
                    <ul class="text-sm text-violet-200">
                        <li>
                            <a class="flex items-center hover:underline hover:text-violet-300" target="_blank" href="https://aiken-lang.github.io/prelude">
                                "aiken-lang/prelude"
                                <Icon icon=BiIcon::BiLinkExternalRegular class="w-3 h-3 ml-1" />
                            </a>
                        </li>
                        <li>
                            <a class="flex items-center hover:underline hover:text-violet-300" target="_blank" href="https://aiken-lang.github.io/stdlib/v2.0.0">
                                "aiken-lang/stdlib (v2.0.0)"
                                <Icon icon=BiIcon::BiLinkExternalRegular class="w-3 h-3 ml-1" />
                            </a>
                        </li>
                        <li>
                            <a class="flex items-center hover:underline hover:text-violet-300" target="_blank" href="https://aiken-lang.github.io/fuzz">
                                "aiken-lang/fuzz (v2.0.0)"
                                <Icon icon=BiIcon::BiLinkExternalRegular class="w-3 h-3 ml-1" />
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
