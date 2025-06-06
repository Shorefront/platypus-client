//! TMF Form components
//! 
use leptos::prelude::*;
use tmflib::{HasId,HasName,HasValidity};

#[component]
pub fn SingleRow(id: String, label: String, signal : WriteSignal<String>) ->impl IntoView {
    view! {
        <tr>
            <td><label for=id.clone()>{ label }</label></td>
            <td><input 
                id=id.clone() 
                on:input=move |ev| { signal.set(event_target_value(&ev)); } 
                />
            </td>
        </tr>
    }
}

#[component]
pub fn BasicClass<T : HasId> (item : T) -> impl IntoView {
    let id = item.get_id();
    let href = item.get_href();
    view! {
            <tr>
                <td><label for="id">"Id"</label></td>
                <td><a href=href>{ id }</a></td>
            </tr> 
    }
}

#[component]
pub fn NamedClass<T : HasId + HasName + Clone> (item : T,signal : WriteSignal<String>) -> impl IntoView {
    let name = item.get_name();
    view!{
        <fieldset>
            <legend>"Class "{ T::get_class()}</legend>
            <BasicClass item=item />
            <tr>
                <td><label for="name">"Name"</label></td>
                <td><input id="name" value=name on:input=move |ev| {
                    signal.set(event_target_value(&ev));
                }/></td>
            </tr>
        </fieldset>
    }
}

#[component]
pub fn Validity<T : HasId + HasValidity> (item : T) -> impl IntoView {
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