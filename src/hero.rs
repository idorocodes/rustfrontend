use std::{thread, time::Duration};

use leptos::prelude::*;




pub fn Hero () -> impl IntoView {
    
    let (visible, set_visible) = signal(false);
    
    let timeout_cb = move || {
        set_visible.set(true);
    };
    
    set_timeout(timeout_cb, Duration::from_secs(1));
    
    view! {
        {move || visible.get().then(|| view! {
            <h1>"This is the Hero Section"</h1>
        })}
        
    } 
}