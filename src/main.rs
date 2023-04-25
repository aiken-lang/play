use leptos::*;

mod compiler_error;
mod components;
mod playground;
mod project;

use playground::{Playground, PlaygroundProps};

fn main() {
    mount_to_body(|cx| {
        view! { cx, <Playground/> }
    })
}
