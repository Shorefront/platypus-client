//! Category View
use leptos::*;

#[component]
pub fn CategoryTable() -> impl IntoView {
    view! {    
        <p>"Category Table"</p>
        <table>
        </table>
    }
}

#[component]
pub fn CategoryView(_id : String) -> impl IntoView {
    view! {
        <p>
        "Some Category View"
        </p>
    }
}