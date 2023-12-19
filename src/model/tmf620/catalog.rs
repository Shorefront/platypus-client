use leptos::*;
use leptos_router::*;

use tmflib::tmf620::catalog::Catalog;
use super::super::common::GenericTable;

#[component]
pub fn CatalogList() -> impl IntoView {
    let cat1 = Catalog::new("Design");
    let cat2 = Catalog::new("Production");
    let catalogs = vec![cat1,cat2];
    view! {
        <div class="list">
            <GenericTable items=catalogs />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn CatalogDetails() -> impl IntoView {
    view! {
        <p>"Catalog Details"</p>
    }
}