//! Product Specification Model

use leptos::prelude::*;
use leptos_router::components::Outlet;
// use tmflib::HasId;

// use crate::model::common::table::GenericTable;
use crate::model::common::form::NamedClass;
use crate::model::common::list::GenericListWithAdd;
use crate::model::common::form::SingleRow;
use tmflib::tmf620::product_specification::ProductSpecification;

#[component]
pub fn ProductSpecificationTable() -> impl IntoView {
    let spec1 = ProductSpecification::new("Internet Specification");
    let spec2 = ProductSpecification::new("WAN Specification");
    let specs = vec![spec1, spec2];
    view! {
        <div class="list">
            <GenericListWithAdd items=specs />
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
    let (name, set_name) = signal("New Specification".to_string());
    let (description, set_description) = signal("".to_string());
    let (brand, set_brand) = signal("".to_string());
    brand.with(|b| println!("Brand changed to: {}", b));
    let new_ps = ProductSpecification::new(name.get());
    view! {
        <div class="form">
        <NamedClass item=&new_ps signal=set_name />
        <fieldset>
            <legend>"Details"</legend>
            <SingleRow id="desc" label="Description" read=description write=set_description />
            <SingleRow id="brand" label="Brand" read=brand write=set_brand />
        </fieldset>
        <p>"Will create new speciication called: " { name } " with description " { description } " and brand " { brand }</p>
        </div>
    }
}
