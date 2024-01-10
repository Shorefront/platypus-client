use leptos::*;
use leptos_router::*;
use reqwest_wasm::Client;
use log::info;

use tmflib::tmf620::catalog::Catalog;
use super::super::common::GenericTable;

async fn get_catalogs() -> Vec<Catalog> {

    let href = format!("http://localhost:8000/tmf-api/productCatalogManagement/v4/catalog");
    let client = Client::new();
    let res = client.get(href).send().await;
    match res {
        Ok(r) => {
            info!("Fetched Catalog data");
            let body = r.text().await;
            match body {
                Ok(b) => {
                    vec![]
                },
                Err(e) => {
                    vec![]
                }
            }
        },
        Err(e) => {
            vec![]
        }
    }
}

#[component]
pub fn CatalogList() -> impl IntoView {
    let load_cat_list = create_resource(|| (),|_| async move {
        get_catalogs().await
    });
    let category_list = load_cat_list.get();
    let catalogs = match category_list {
        Some(cl) => cl,
        None => vec![],
    };
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
pub fn CatalogTable() -> impl IntoView {

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