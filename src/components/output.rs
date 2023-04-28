use std::error::Error;

use aiken_lang::tipo::error::Warning;
use leptos::*;
use leptos_icons::*;
use miette::Diagnostic;

use crate::{compiler_error::CompilerError, project::TestResult};

#[component]
pub fn Output(
    cx: Scope,
    test_results: ReadSignal<Vec<(usize, TestResult)>>,
    warnings: ReadSignal<Vec<(usize, Warning)>>,
    errors: ReadSignal<Vec<(usize, CompilerError)>>,
) -> impl IntoView {
    view! { cx,
        <div class="p-4 overflow-y-scroll flex grow flex-col gap-y-11">
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
                                        <div class="flex items-center justify-start gap-x-9 text-gray-70 mr-9 text-sm font-normal">
                                            <div class="flex items-center gap-x-1">
                                                <LeptosIcon icon=RiIcon::RiCpuDeviceLine class="w-3.5 h-3.5"/>
                                                {test_result.spent_budget.cpu}
                                            </div>
                                            <div class="flex items-center gap-x-1">
                                                <LeptosIcon icon=RiIcon::RiDatabase2DeviceLine class="w-3.5 h-3.5"/>
                                                {test_result.spent_budget.mem}
                                            </div>
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
                                            <LeptosIcon icon=RiIcon::RiErrorWarningSystemLine class="w-3.5 h-3.5"/>
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
                                            <LeptosIcon icon=RiIcon::RiAlertSystemLine class="w-3.5 h-3.5"/>
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
        </div>
    }
}
