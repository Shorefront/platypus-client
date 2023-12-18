use leptos::*;
use leptos_router::*;

use tmflib::tmf620::catalog::Catalog;
use super::super::common::GenericTable;

async fn load_catalogs() -> Vec<Catalog> {
    // Call Platypus API to retrieve Catalogs
    let cat1 = Catalog::new("Design");
    let cat2 = Catalog::new("Run");
    let catalogs = vec![cat1,cat2];
    catalogs
}

#[component]
pub fn CatalogList() -> impl IntoView {
    let stable = create_resource(|| (), |_| async move {
        load_catalogs().await
    });
    let catalogs = stable.get().unwrap_or_default();
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