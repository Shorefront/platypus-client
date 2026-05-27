//! Product Offering Model

use leptos::prelude::*;
use leptos_router::components::Outlet;
// use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;
use tmf_leptos::common::has_name::NamedClass;

use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::{HasId,HasName,HasDescription};
use tmf_leptos::common::has_description::HasDescription;

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
    let (name, set_name) = signal(new_item.get_name());
    let (desc, set_desc) = signal(new_item.get_description());  
    // Update proto-object
    name.with(|n| new_item.set_name(n));
    desc.with(|d| new_item.set_description(d));
    view! {
        <form>
            <NamedClass item=&new_item signal=set_name />
            <HasDescription description_read=desc description_write=set_desc />
            <button type="submit">"Submit"</button>
        </form>
        <div class="debug">"Will create new offering called: " { name } " with description " { desc }</div>
        <Outlet />
    }
}

