
use tmflib::HasLastUpdate;
use leptos::prelude::*;

#[component]
pub fn LastUpdate<T : HasLastUpdate,'a>(item: &'a T) -> impl IntoView {
    view! {
        <label>"Last Update: "</label>
        <input type="text" value={T::get_timestamp()} readonly=true disabled=true />
    }
}