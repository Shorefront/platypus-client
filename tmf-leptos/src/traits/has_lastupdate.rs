//! Forms for HasLastUpdate trait

use super::SingleRow;
use leptos::prelude::*;

#[component]
pub fn HasLastUpdate(lu_read : ReadSignal<String>,lu_write : WriteSignal<String>) -> impl IntoView {
    
    view! {
        <SingleRow id="last_update" label="Last Update" read=lu_read write=lu_write />
    }
}