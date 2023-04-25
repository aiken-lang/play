use leptos::*;
use leptos_icons::*;

#[component]
pub fn Header<F>(cx: Scope, on_check: F) -> impl IntoView
where
    F: Fn(web_sys::MouseEvent) + 'static,
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
            <div class="flex gap-x-4">
                <button
                    on:click=on_check
                    class="bg-gray-40 flex justify-center items-center gap-x-2 text-sm font-semibold text-white w-24 py-1.5 rounded"
                >
                    <LeptosIcon icon=RiIcon::RiPlayMediaFill/>
                    "Check"
                </button>
                <button class="bg-share-button flex justify-center items-center gap-x-2 text-sm font-semibold text-white px-3 py-1.5 rounded">
                    <LeptosIcon icon=RiIcon::RiShareForwardSystemFill/>
                    "Share"
                </button>
            </div>
        </header>
    }
}
