//! TMF Form components
//! 
use leptos::*;
use tmflib::{HasId,HasName};

#[component]
pub fn NamedClass<T : HasId + HasName> (item : T,signal : WriteSignal<String>) -> impl IntoView {
    let id = item.get_id();
    let href = item.get_href();
    let name = item.get_name();
    view!{
        <fieldset>
            <legend>"Class "{ T::get_class()}</legend>
            <tr>
                <td><label for="id">"Id"</label></td>
                <td><input id="id" value=id /></td>
            </tr>
            <tr>
                <td><label for="href">"HREF"</label></td>
                <td><input id="href" value=href /></td>
            </tr>
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
pub fn Validity<T : HasId + HasName> (item : T) -> impl IntoView {
    view! {
        <fieldset>
            <legend>Validity</legend>
            <tr>
                <td><label for="start_date_time">Start</label></td>
                <td><input id="start_date_time" type="datetime-local" /></td>
            </tr>
            <tr>
                <td><label for="end_date_time">End</label></td>
                <td><input id="end_date_time" type="datetime-local" /></td>
            </tr>
        </fieldset>
    }
}