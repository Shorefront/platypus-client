//! Product Offering Model

use leptos::prelude::*;
use leptos_router::components::Outlet;
// use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;
use tmf_leptos::traits::has_name::NamedClass;

use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::{HasName,HasDescription,HasValidity};
use tmf_leptos::traits::has_description::HasDescription;
use tmf_leptos::traits::has_validity::HasValidity;

#[component]
pub fn ProductOfferingTable() -> impl IntoView {
    let offer1 = ProductOffering::new("Internet");
    let offer2 = ProductOffering::new("WAN");
    let items = vec![offer1, offer2];
    view! {
        <div class="list">
            <GenericListWithAdd items=items/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn ProductOfferingView() -> impl IntoView {
    view! {
        <p>"Product Offering View"</p>
    }
}

#[component]
pub fn ProductOfferingAdd() -> impl IntoView {
    let mut new_item = ProductOffering::new("New Offering");
    // Has the validity period been updated. We pass this flag in
    let valid_dirty : bool = false;
    let (name, set_name) = signal(new_item.get_name());
    let (desc, set_desc) = signal(new_item.get_description());
    let (valid,valid_write) = signal(valid_dirty);
    // Update proto-object
    desc.with(|d| new_item.set_description(d));
    name.with(|n| new_item.set_name(n));
    let mut validity = new_item.get_validity().unwrap_or_default();
    valid.with(|f| {
        // Flag has been set to dirty, so we know to update the validity in the proto-object
        if *f {
            new_item.set_validity(validity.clone());
            // Reset dirty flag
            valid_write.set(false);
        }
    });
    view! {
        <form>
            <NamedClass item=&new_item signal=set_name />
            <HasDescription description_read=desc description_write=set_desc />
            <HasValidity period=&mut validity dirty=valid_write />
            <button type="submit">"Submit"</button>
        </form>
        <div class="debug">"Will create new offering called: " { name } " with description " { desc } </div>
        <Outlet />
    }
}

