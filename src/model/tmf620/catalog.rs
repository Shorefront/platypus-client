use leptos::*;
use leptos_router::*;

use tmflib::tmf620::catalog::Catalog;
use super::super::common::GenericTable;

#[component]
pub fn CatalogTable() -> impl IntoView {
    let cat1 = Catalog::new("Design");
    let cat2 = Catalog::new("Run");
    let catalogs = vec![cat1,cat2];
    view! {
        <div class="list">
            <GenericTable items=catalogs/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn CatalogView() -> impl IntoView {
    view! {
        <p>"Catalog Item"</p>
    }
}