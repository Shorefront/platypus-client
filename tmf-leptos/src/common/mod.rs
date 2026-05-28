
pub mod entity;
pub mod time_period;
pub mod related_party;
pub mod money;
pub mod price;

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