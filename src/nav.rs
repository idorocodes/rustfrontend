use leptos::prelude::*;
use stylance::import_style;



pub fn Nav() -> impl IntoView {
    let (is_menu_open, set_is_menu_open) = signal(false);

    let toggle_menu = move || {
        set_is_menu_open.update(|v| *v = !*v);
    };

    view! {
        <header  class= "header">
            <nav class="navbar">
                <h1>"Slink"</h1>
                    <DesktopNav/>
                    <button> "Send a slink" </button>
            </nav>
            
           

        </header>
    }
}


#[component]
pub fn DesktopNav() -> impl IntoView {
    view! {
        <div class="desktop-nav">
            <a href="/">"Home"</a>
            <a href="/about">"About"</a>
            <a href="/contact">"Contact"</a>
        </div>
    }
}


#[component]
pub fn MobileNav() -> impl IntoView {
    view! {
        <div class="mobile-nav">
            <a href="/">"Home"</a>
            <a href="/about">"About"</a>
            <a href="/contact">"Contact"</a>
        </div>
    }
}
