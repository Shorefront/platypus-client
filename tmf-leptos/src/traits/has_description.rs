//! Form for entities that have a description field.

use leptos::prelude::*;

#[component]
pub fn HasDescription(
    description_read: ReadSignal<String>,
    description_write: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"Description"</legend>
            <label for="description" class="form-label">"Description"</label>
            <input
                type="text"
                class="form-control"
                id="description"
                value=description_read.get()
                on:input=move |ev| {
                    let input = event_target_value(&ev);
                    description_write.set(input);
                }
            />
        </fieldset>
    }
}