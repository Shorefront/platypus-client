//! Product Offering Price Module
//! This module defines the Product Offering Price Model
//! It is part of TMForum Open API 6.2.0 Product Offering module
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
// use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;

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
