//! Common components
//! 

use leptos::*;

pub mod table;
pub mod menu;
pub mod form;
pub mod routes;

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <div class="banner">Platypus</div>
    }
}