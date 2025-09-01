//! Common components
//!

use leptos::prelude::*;

pub mod form;
pub mod list;
pub mod menu;
pub mod routes;
pub mod table;

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <div class="banner">Platypus</div>
    }
}
