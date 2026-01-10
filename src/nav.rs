use leptos::prelude::*;
use stylance::import_style;



pub fn Nav() -> impl IntoView {
    
    view! {
        <header  class= "header">
            <nav class="navbar">
                <h1>"Leptos Starter"</h1>
                    <DesktopNav/>
                    <button> "Start building" </button>
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
