use leptos::prelude::*;
use leptos_router::components::Outlet;
use log::info;
use reqwest_wasm::Client;
use tmflib::HasRelatedParty;

use crate::model::common::list::GenericListWithAdd;
use crate::model::common::table::GenericTable;
use tmflib::{tmf620::catalog::Catalog, HasId};
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf632::individual_v4::Individual;
use tmf_leptos::common::time_period::TimePeriod;
use tmf_leptos::common::related_party::RelatedPartyList;

const DEFAULT_HOST: &str = "http://localhost:8001";

async fn get_catalogs() -> Vec<Catalog> {
    let href = format!("{}{}", DEFAULT_HOST, Catalog::get_class_href());
    let client = Client::new();
    let res = client.get(href).send().await;
    match res {
        Ok(r) => {
            info!("Fetched Catalog data");
            let body = r.text().await;
            match body {
                Ok(_b) => {
                    vec![]
                }
                Err(_e) => {
                    vec![]
                }
            }
        }
        Err(_e) => {
            // Could not get catalog, return some defaults
            let cat1 = Catalog::new("Design");
            let cat2 = Catalog::new("Production");
            let catalogs = vec![cat1, cat2];
            catalogs
        }
    }
}

#[component]
pub fn CatalogAdd() -> impl IntoView {
    let rp = RelatedParty::from(&Individual::new("John Smith"));
    let new_item = Catalog::new("New Catalog")
        .party(rp);
    view! {
        <form>
            <fieldset>
                <legend>"New Catalog"</legend>
                <label for="name">"Name"</label>
                <input type="text" id="name" name="name" /><br />
                <label for="description">"Description"</label>
                <input type="text" id="description" name="description" /><br />
            </fieldset>
            <TimePeriod item=&new_item />
            <RelatedPartyList item=&new_item />
            <button type="submit">"Submit"</button>
        </form>
        <Outlet />
    }
}

#[component]
pub fn CatalogList() -> impl IntoView {
    let cat1 = Catalog::new("Components");
    let cat2 = Catalog::new("Products");
    let cat_list = vec![cat1, cat2];

    view! {
        <div class="list">
            <GenericListWithAdd items=cat_list />
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
    let add_href = format!("{}/add", Catalog::get_class_href());
    let catalogs = vec![cat1, cat2];
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
