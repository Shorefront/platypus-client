//! Generate Route components

use leptos::prelude::*;
// use leptos_router::*;

use tmflib::{HasId,HasName};

#[component]
pub fn GenericHome() -> impl IntoView {
    view! {
        <p>"Select something"</p>
    }
}

#[component]
pub fn GenericList<T : HasId + HasName + Clone>(_item : T) -> impl IntoView {
    let class_name = T::get_class();
    view! {
        <p>"A List of "{class_name }</p>
    }
}

// #[component(transparent)]
// pub fn GenericRoute<T : HasId + HasName + Clone>(item : T) -> impl IntoView {
//     let generic_path = T::get_mod_path();
//     let class_path = T::get_class();
//     view! {
//         <Route path=generic_path view=GenericHome />
//             <Route path=class_path view=GenericList/>
//         </Route>
//     }
// }

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"No option selected"</p>
    }
}