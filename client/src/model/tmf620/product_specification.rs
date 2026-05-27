//! Product Specification Model

use leptos::prelude::*;
use leptos_router::components::Outlet;
// use tmflib::HasId;

// use crate::model::common::table::GenericTable;
use tmf_leptos::common::has_name::NamedClass;
use crate::model::common::list::GenericListWithAdd;
use tmf_leptos::common::SingleRow;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::{HasName,HasDescription};
use tmf_leptos::common::has_description::HasDescription;

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
    let (desc, set_desc) = signal("".to_string());
    let (brand, set_brand) = signal("".to_string());
    brand.with(|b| println!("Brand changed to: {}", b));
    let mut new_ps = ProductSpecification::new(name.get());

    // Update proto-object
    name.with(|n| new_ps.set_name(n));
    desc.with(|d| new_ps.set_description(d));
    view! {
        <div class="form">
        <NamedClass item=&new_ps signal=set_name />
        <HasDescription description_read=desc description_write=set_desc />
        <fieldset>
            <legend>"Details"</legend>
            <SingleRow id="brand" label="Brand" read=brand write=set_brand />
        </fieldset>
        <div class="debug">"Will create new speciication called: " { name } " with description " { desc } " and brand " { brand }</div>
        </div>
    }
}
