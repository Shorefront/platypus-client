//! Product Specification Model

use leptos::*;
use leptos_router::Outlet;

use crate::model::common::GenericTable;
use tmflib::tmf620::product_specification::ProductSpecification;

#[component]
pub fn ProductSpecificationTable() -> impl IntoView {
    let spec1 = ProductSpecification::new("Spec One".to_string());
    let specs = vec![spec1];
    view! {
        <div class="list">
            <GenericTable items=specs />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn ProductSpecificationView() -> impl IntoView {
    view! {
        <p>"A view of specifications"</p>
    }
}