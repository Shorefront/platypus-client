//! Product Offering Model

use leptos::prelude::*;
use leptos_router::components::Outlet;
// use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;

use tmflib::tmf620::product_offering::ProductOffering;

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
    let new_item = ProductOffering::new("New Offering");
    view! {
        <form>
            <fieldset>
                <legend>"New Product Offering"</legend>
                <label for="name">"Name"</label>
                <input type="text" id="name" name="name" /><br />
                <label for="description">"Description"</label>
                <input type="text" id="description" name="description" /><br />
            </fieldset>
            <button type="submit">"Submit"</button>
        </form>
        <Outlet />
    }
}

