
use tmflib::HasLastUpdate;
use leptos::prelude::*;

/// TODO: use get_last_update() when tmflib updated to 0.1.34
#[component]
pub fn LastUpdate<T : HasLastUpdate,'a>(item: &'a T) -> impl IntoView {
    view! {
        <label>"Last Update: "</label>
        <input type="text" value={T::get_timestamp()} readonly=true disabled=true />
    }
}