//! Product Offering Model

use leptos::*;
use leptos_router::Outlet;
use crate::model::common::GenericTable;

use tmflib::tmf620::product_offering::ProductOffering;

#[component]
pub fn ProductOfferingTable() -> impl IntoView {
    let offer1 = ProductOffering::new("Offer1".to_string());
    let items = vec![offer1];
    view! {
        <div class="list">
            
            <GenericTable items=items/>
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