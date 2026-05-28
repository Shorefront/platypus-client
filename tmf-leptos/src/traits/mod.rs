
pub mod has_description;
pub mod has_id;
pub mod has_name;
pub mod has_validity;
pub mod has_lastupdate;

use leptos::prelude::*;

#[component]
pub fn SingleRow(id: impl Into<String>, label: impl Into<String>, read: ReadSignal<String>, write: WriteSignal<String>) -> impl IntoView {
    let id = id.into();
    let label = label.into();
    view! {
            <label for=id.clone()>{ label }</label>
            <input
                id=id.clone()
                on:input=move |ev| { write.set(event_target_value(&ev)); }
                value={read.get()}
                />
            <br />
    }
}