//! AccountRef 
//! 
//! 
use leptos::prelude::*;

use tmflib::tmf666::AccountRef;

#[component]
pub fn account_ref(item : AccountRef) -> impl IntoView {
    let href = item.href;
    let name = item.name;

    view!{
        <a href={href}>{name}</a>
    }
}