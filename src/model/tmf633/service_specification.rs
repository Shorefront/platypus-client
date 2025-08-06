//! Service Specification Module
//!

use crate::model::common::list::GenericListWithAdd;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use tmflib::tmf633::service_specification::ServiceSpecification;

#[component]
pub fn ServiceSpecificationList() -> impl IntoView {
    let spec1 = ServiceSpecification::new("Specification 1");
    let spec2 = ServiceSpecification::new("Specification 2");
    let items = vec![spec1, spec2];
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
pub fn ServiceSpecificationView() -> impl IntoView {
    view! {}
}

#[component]
pub fn ServiceSpecificationForm() -> impl IntoView {
    view! {}
}
