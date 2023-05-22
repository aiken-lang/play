use leptos::*;
use leptos_icons::*;

#[component]
pub fn Header<F1, F2, F3>(
    cx: Scope,
    checking: ReadSignal<bool>,
    on_format: F1,
    on_check: F2,
    on_share: F3,
) -> impl IntoView
where
    F1: Fn(web_sys::MouseEvent) + 'static,
    F2: Fn(web_sys::MouseEvent) + 'static,
    F3: Fn(web_sys::MouseEvent) + 'static,
{
    view! { cx,
        <header class="flex justify-between items-center p-3 border-b border-solid border-gray-40">
            <div class="flex items-center gap-x-3">
                <img
                    class="w-8 h-8"
                    type="image/png"
                    src="https://raw.githubusercontent.com/aiken-lang/branding/main/assets/icon.png"
                />
                <span class="text-white text-lg font-semibold">"AIKEN PLAYGROUND"</span>
            </div>
            <div class="gap-x-4 hidden md:flex">
                <button
                    on:click=on_format
                    class="bg-gray-40 flex justify-center items-center gap-x-2 text-sm font-semibold text-white w-24 py-1.5 rounded"
                >
                    <Show
                        when=move || !checking.get()
                        fallback=|cx| {
                            view! { cx, <LeptosIcon icon=RiIcon::RiRefreshSystemLine/> }
                        }
                    >
                        <LeptosIcon icon=RiIcon::RiFileEditDocumentLine/>
                    </Show>
                    "Format"
                </button>
                <button
                    on:click=on_check
                    class="bg-gray-40 flex justify-center items-center gap-x-2 text-sm font-semibold text-white w-24 py-1.5 rounded"
                >
                    <Show
                        when=move || !checking.get()
                        fallback=|cx| {
                            view! { cx, <LeptosIcon icon=RiIcon::RiRefreshSystemLine/> }
                        }
                    >
                        <LeptosIcon icon=RiIcon::RiPlayMediaFill/>
                    </Show>
                    "Check"
                </button>
                <button
                    on:click=on_share
                    class="bg-share-button flex justify-center items-center gap-x-2 text-sm font-semibold text-white px-3 py-1.5 rounded"
                >
                    <LeptosIcon icon=RiIcon::RiShareForwardSystemFill/>
                    "Share"
                </button>
            </div>
        </header>
    }
}
