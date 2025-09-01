//! Entity Module

use leptos::*;

use tmflib::HasId;

#[component]
pub fn entity<T : HasId>(_item : T) -> impl IntoView {
    let class_name = T::get_class();
    view!{
        <input type="hidden" name="baseClass" value=class_name />
    }
}