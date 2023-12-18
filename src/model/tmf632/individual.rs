//! Individual Model

use leptos::*;
use leptos_router::*;

use tmflib::tmf632::individual::Individual;

use crate::GenericTable;

#[component]
pub fn IndividualTable() -> impl IntoView {
    let ind1 = Individual::new("Ryan");
    let ind2 = Individual::new("John");
    let ind3 = Individual::new("Fred");
    let individuals = vec![ind1,ind2,ind3];
    view! {
        <div class="list">
            <GenericTable items=individuals/>
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