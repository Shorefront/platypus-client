//! Individual Model

use leptos::prelude::*;
use leptos_router::components::*;

#[cfg(feature = "V4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "V5")]
use tmflib::tmf632::individual_v5::Individual;

// use crate::GenericTable;
use crate::model::common::list::GenericListWithAdd;

#[component]
pub fn IndividualTable() -> impl IntoView {
    let ind1 = Individual::new("Ryan");
    let ind2 = Individual::new("John");
    let ind3 = Individual::new("Fred");
    let individuals = vec![ind1, ind2, ind3];
    view! {
        <div class="list">
            <GenericListWithAdd items=individuals/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn IndividualView() -> impl IntoView {
    view! {
        <p>"Placeholder for individual data"</p>
    }
}
