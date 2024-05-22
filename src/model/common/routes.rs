//! Generate Route components

use tmflib::{HasId,HasName};

use leptos::*;
use leptos_router::*;



#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"No option selected"</p>
    }
}