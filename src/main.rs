use leptos::prelude::*;
mod nav;
use nav::*;
mod hero;
use hero::*;

fn main () {
    
    leptos::mount::mount_to_body(App); 
    
    
} 

#[component]
fn App () ->  impl IntoView {
    view! {
    <Nav/>
    <main>
    <Hero/>
    </main>
    
    }
}

