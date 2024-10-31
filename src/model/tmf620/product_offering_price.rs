//! Product Offering Price Module

use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use crate::model::common::table::GenericTable;
use crate::model::common::list::{GenericListWithAdd, GenericListWithAddProps};

use leptos::*;
use leptos_router::*;

#[component]
pub fn ProductOfferingPriceList() -> impl IntoView {
    let price1 = ProductOfferingPrice::new("Internet Pricing");
    let price2 = ProductOfferingPrice::new("WAN Pricing");
    let prices = vec![price1,price2];
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