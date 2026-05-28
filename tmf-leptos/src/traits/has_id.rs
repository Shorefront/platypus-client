//! HasId Forms

use leptos::prelude::*;

use tmflib::HasId;

#[component]
pub fn BasicClass<T: HasId>(item: T) -> impl IntoView {
    let id = item.get_id();
    let href = item.get_href();
    view! {
        <label for="id">"Link"</label>
        <a href=href>{ id }</a><br />
    }
}