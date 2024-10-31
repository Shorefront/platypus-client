//! Product Offering Model

use leptos::*;
use leptos_router::Outlet;
use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;

use tmflib::tmf620::product_offering::ProductOffering;

#[component]
pub fn ProductOfferingTable() -> impl IntoView {
    let offer1 = ProductOffering::new("Internet");
    let offer2 = ProductOffering::new("WAN");
    let items = vec![offer1,offer2];
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