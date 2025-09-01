//! TMF Form components
//!
use leptos::prelude::*;
use tmflib::{HasId, HasName, HasValidity};

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

#[component]
pub fn BasicClass<T: HasId>(item: T) -> impl IntoView {
    let id = item.get_id();
    let href = item.get_href();
    view! {
        <label for="id">"Link"</label>
        <a href=href>{ id }</a><br />
    }
}

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

#[component]
pub fn Validity<T: HasId + HasValidity>(item: T) -> impl IntoView {
    let valid_start = item.get_validity_start();
    let valid_end = item.get_validity_end();
    view! {
        <fieldset>
            <legend>Validity</legend>
            <tr>
                <td><label for="start_date_time">Start</label></td>
                <td><input id="start_date_time" type="datetime-local" value=valid_start /></td>
            </tr>
            <tr>
                <td><label for="end_date_time">End</label></td>
                <td><input id="end_date_time" type="datetime-local" value=valid_end/></td>
            </tr>
        </fieldset>
    }
}
