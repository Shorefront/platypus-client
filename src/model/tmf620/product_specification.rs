//! Product Specification Model

use leptos::*;
use leptos_router::Outlet;
use tmflib::HasId;

use crate::model::common::table::GenericTable;
use crate::model::common::form::NamedClass;
use tmflib::tmf620::product_specification::ProductSpecification;

#[component]
pub fn ProductSpecificationTable() -> impl IntoView {
    let spec1 = ProductSpecification::new("Spec One".to_string());
    let specs = vec![spec1];
    let spec_add = format!("{}/add",ProductSpecification::get_class_href());
    view! {
        <div class="list">
            <GenericTable items=specs />
            <a href=spec_add>"Add"</a>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn ProductSpecificationView() -> impl IntoView {
    view! {
        <p>"A view of specifications"</p>
    }
}

#[component]
pub fn ProductSpecificationAdd() -> impl IntoView {
    let (name,set_name) = create_signal("New Specification".to_string());
    let new_ps = ProductSpecification::new(name.get());
    view! {
        <div class="form">
        <NamedClass item=new_ps.clone() signal=set_name />
        <p>"Will create new category called: " { name }</p>
        </div>
    }
}