use leptos::prelude::*;
use js_sys::Date;

pub fn Hero() -> impl IntoView {
    let now = Date::new_0();

    let day = now.get_date();
    let hour = now.get_hours();
    let minute = now.get_minutes();

    view! {
        <section class="hero">
            <h1 class="heroheader">
                "Build Fast. Think in Rust. Build Often, Ship Often."
            </h1>

            <p class="herosub">
                "A modern Rust-powered web experience using " <a href="https://leptos.dev"> "Leptos " </a> "and " <a href="https://webassembly.org"> WebAssembly</a> "."
            </p>

            <div class="herometa">
                <span>
                    "Local time: "
                    {hour} ":" {minute} " · Day " {day}
                </span>
            </div>

            <div class="heroactions">
                <button class="primary-btn">
                    "Get Started"
                </button>

            </div>
        </section>
    }
}
