//! Product Offering Price Module
//! This module defines the Product Offering Price Model
//! It is part of TMForum Open API 6.2.0 Product Offering module
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::HasName;
// use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;
// use tmf_leptos::common::has_description::HasDescription;
use tmf_leptos::common::has_name::NamedClass;

use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn ProductOfferingPriceList() -> impl IntoView {
    let price1 = ProductOfferingPrice::new("Internet Pricing");
    let price2 = ProductOfferingPrice::new("WAN Pricing");
    let prices = vec![price1, price2];
    view! {
        <div class="list">
            <GenericListWithAdd items=prices />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn ProductOfferingPriceDetail() -> impl IntoView {
    view! {
        <p>"Product Offering Price"</p>
    }
}

#[component]
pub fn ProductOfferingPriceAdd() -> impl IntoView {
    let mut new_item = ProductOfferingPrice::new("New Price");
    let (name, set_name) = signal(new_item.get_name());
    // let (desc, set_desc) = signal(new_item.get_description());
    // Update proto-object
    name.with(|n| new_item.set_name(n));
    // desc.with(|d| new_item.set_description(d));
    view! {
        <form>
            <NamedClass item=&new_item signal=set_name />
            <div class="debug">"Will create new catalog called: " { name }</div>
            <button type="submit">"Submit"</button>
        </form>
        <Outlet />
    }
}
