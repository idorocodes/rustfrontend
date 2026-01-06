use leptos::{prelude::*, svg::view};
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
    <p> "Hello World" </p>   
    </main>
    
    }
}

