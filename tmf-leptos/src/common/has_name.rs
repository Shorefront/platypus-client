//! Forms 

use leptos::prelude::*;

use tmflib::{HasId,HasName};

#[component]
pub fn NamedClass<T: HasId + HasName + ,'a>(
    item: &'a T,
    signal: WriteSignal<String>,
) -> impl IntoView {
    let name = item.get_name();
    view! {
        <fieldset>
            <legend>{ T::get_class()}</legend>
            <label for="name">"Name"</label>
            <input id="name" value=name on:input=move |ev| {
                signal.set(event_target_value(&ev));
            }/>
        </fieldset>
    }
}