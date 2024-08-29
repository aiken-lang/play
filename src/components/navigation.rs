use leptos::*;
use leptos_icons::*;

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col justify-between p-4 text-gray-0 border-r border-solid border-gray-40">
            <div class="flex flex-col gap-y-7">
                <a target="_blank" href="https://aiken-lang.org/fundamentals/getting-started">
                    <Icon icon=RiIcon::RiBook2DocumentFill class="w-6 h-6"/>
                </a>
                <a target="_blank" href="https://discord.gg/Vc3x8N9nz2">
                    <Icon icon=RiIcon::RiDiscordLogosFill class="w-6 h-6"/>
                </a>
                <a target="_blank" href="https://github.com/aiken-lang">
                    <Icon icon=RiIcon::RiGithubLogosFill class="w-6 h-6"/>
                </a>
            </div>
        </div>
    }
}
