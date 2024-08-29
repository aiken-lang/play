use super::code_editor::ModelCell;
use base64::Engine;
use leptos::*;
use leptos_icons::*;
use std::io::{Cursor, Write};

#[component]
pub fn Share<F1, F2>(
    cx: Scope,
    display: ReadSignal<bool>,
    editor: ReadSignal<ModelCell>,
    on_close: F1,
    on_cancel: F2,
) -> impl IntoView
where
    F1: Fn(web_sys::MouseEvent) + 'static,
    F2: Fn(web_sys::MouseEvent) + 'static,
{
    let (copied, set_copy) = create_signal(cx, false);

    let share_url = move || get_share_url(editor);

    let on_copy = move |_| {
        let _ = window().navigator().clipboard().write_text(&share_url());
        set_copy.set(true);
    };

    let on_close = move |e| {
        set_copy.set(false);
        on_close(e);
    };

    let on_cancel = move |e| {
        set_copy.set(false);
        on_cancel(e);
    };

    let copy_text = move || if copied.get() { "Copied!" } else { "Copy" };

    view! { cx,
        <div
            class="fixed inset-0 z-10 overflow-y-auto h-full w-full bg-black opacity-30"
            hidden=move || !display.get()
            on:click=on_cancel
        ></div>
        <aside
            class="fixed top-1/4 inset-x-1/4 z-10 rounded bg-neutral-800 drop-shadow-md text-white"
            hidden=move || !display.get()
        >
            <div class="grid grid-cols-2 px-5 py-3">
                <h2 class="text-lg font-semibold">"SHARE"</h2>
                <button class="justify-self-end" on:click=on_close>
                    <Icon icon=RiIcon::RiCloseSystemLine class="w-6 h-6"/>
                </button>
            </div>
            <hr class="border-1 border-gray-500"/>
            <div class="px-5 p-3">
                <p>"Share a" <strong>" snapshot "</strong> "of your playground with this link."</p>
                <fieldset class="flex container inset-0 my-3">
                    <button
                        class="flex items-center gap-x-2 p-2 bg-share-button rounded-l-md"
                        on:click=on_copy
                    >
                        <Icon icon=RiIcon::RiClipboardDocumentLine class="w-4 h-4"/>
                        {copy_text}
                    </button>
                    <input
                        class="w-full px-3 text-purple-200 bg-neutral-600 rounded-r-md"
                        type="text"
                        value=share_url
                    />
                </fieldset>
                <p class="text-sm text-gray-300">
                    "This link will open a new playground in the same state as yours is. "
                    "Use it to share snippets of Aiken code and to make it easy for people to elaborate on your code."
                </p>
            </div>
        </aside>
    }
}

fn get_share_url(editor: ReadSignal<ModelCell>) -> String {
    let text = editor
        .get()
        .borrow()
        .as_ref()
        .and_then(|editor| editor.get_model())
        .map(|model| model.get_value())
        .unwrap_or_default();

    let compressed_code = [0u8; 4096];
    let cursor = Cursor::new(compressed_code);
    let mut writer = brotli::CompressorWriter::new(cursor, 4096, 11, 22);

    writer.write_all(text.as_bytes()).ok();

    let cursor = writer.into_inner();

    let bytes_written = cursor.position() as usize;

    let bytes = cursor.into_inner();

    let code = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes[..bytes_written]);

    format!("https://play.aiken-lang.org?code={}", code)
}
