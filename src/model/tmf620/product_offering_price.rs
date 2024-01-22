//! Product Offering Price Module

use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use crate::model::common::table::GenericTable;

use leptos::*;
use leptos_router::*;

#[component]
pub fn ProductOfferingPriceList() -> impl IntoView {
    let price1 = ProductOfferingPrice::new("Price1".to_string());
    let prices = vec![price1];
    view! {
        <div class="list">
            <GenericTable items=prices />
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