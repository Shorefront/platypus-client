//! List rendering
//! 

use leptos::prelude::*;
use tmflib::{HasId,HasName};

#[component]
pub fn ListItem<T : HasId + HasName>(item : T) -> impl IntoView {
    view! {
        <li><a href={item.get_href()}>{ item.get_name() }</a></li>
    }
}

#[component]
pub fn GenericList<T : HasId + HasName>(items : Vec<T>) -> impl IntoView {
    view! {
        <h3>{ T::get_class()}s</h3>
        <ul>
            {
                items.into_iter()
                .map(|c| {
                    ListItem(ListItemProps{ item: c})
                }).collect_view()
            }  
        </ul>
    }
}

#[component]
pub fn GenericListWithAdd<T : HasId + HasName>(items : Vec<T>) -> impl IntoView {
    let add_href = format!("{}/add",T::get_class_href());
    let add_title = format!("Add {}",T::get_class());
    view! {
        <GenericList items = items />
        <a href=add_href>{ add_title} </a>
    }   
}