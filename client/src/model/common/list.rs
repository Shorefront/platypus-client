//! List rendering
//!

use leptos::prelude::*;
use tmflib::{HasDescription, HasId, HasName};

#[component]
pub fn ListItem<T: HasId + HasName>(item: T) -> impl IntoView {
    view! {
        <li><a href={item.get_href()}>{ item.get_name() }</a></li>
    }
}

#[component]
pub fn GenericList<T: HasId + HasName>(items: Vec<T>) -> impl IntoView {
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
pub fn GenericListWithAdd<T: HasId + HasName>(items: Vec<T>) -> impl IntoView {
    let add_href = format!("{}/add", T::get_class_href());
    let add_title = format!("Add {}", T::get_class());
    view! {
        <GenericList items = items />
        <a href=add_href>{ add_title} </a>
    }
}

#[component]
pub fn DescItem<T: HasId + HasDescription>(item: T) -> impl IntoView {
    view! {
        <li><a href={item.get_href()}>{ item.get_description() }</a></li>
    }
}

#[component]
pub fn DescList<T: HasId + HasDescription>(items: Vec<T>) -> impl IntoView {
    view! {
        <h3>{ T::get_class()}s</h3>
        <ul>
            {
                items.into_iter()
                .map(|c| {
                    DescItem(DescItemProps{ item: c})
                }).collect_view()
            }
        </ul>
    }
}

#[component]
pub fn DescListWithAdd<T: HasId + HasDescription>(_items: Vec<T>) -> impl IntoView {
    let add_href = format!("{}/add",T::get_class_href());
    let add_title = format!("Add {}",T::get_class());
    view! {
        <p>"Add new item"</p>
        <a href="{add_href}">"{add_title}"</a>
    }
}
