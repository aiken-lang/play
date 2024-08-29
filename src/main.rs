use leptos::*;

mod compiler_error;
mod components;
mod macros;
mod playground;
mod project;
pub(crate) mod vendor;

use playground::Playground;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! { cx, <Playground/> }
    })
}
