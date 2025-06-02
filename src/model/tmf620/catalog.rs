use leptos::prelude::*;
use leptos_router::components::Outlet;
use log::info;

use tmflib::{tmf620::catalog::Catalog, HasId};
use crate::model::common::list::GenericListWithAdd;
use crate::model::common::table::GenericTable;
// use tmf_client::{Operations, TMFClient};
use reqwest_wasm::Client;

const DEFAULT_HOST : &str = "https://localhost:8001";

async fn get_catalogs() -> Vec<Catalog> {
    let url = format!("{}/catalogManagement/v4/catalog",DEFAULT_HOST);
    let client = reqwest_wasm::get(url).await.unwrap();
    let catalogs : Vec<Catalog> = serde_json::from_str(client.text().await.unwrap().as_str()).unwrap();
    catalogs
}

#[component]
pub fn CatalogAdd() -> impl IntoView {
    view! {
    }
}

#[component]
pub fn CatalogList() -> impl IntoView {
    let once = OnceResource::new(get_catalogs());

    view! {
        <div class="list">
            <GenericListWithAdd items=once.get().unwrap() />
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
    let add_href = format!("{}/add",Catalog::get_class_href());
    let catalogs = vec![cat1,cat2];
    view! {
        <div class="list">
            <GenericTable items=catalogs />
            <a href=add_href>"New Catalogue"</a>
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