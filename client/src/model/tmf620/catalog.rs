use leptos::prelude::*;
use leptos_router::components::Outlet;
use log::info;
use reqwest_wasm::Client;
use tmflib::HasRelatedParty;

use tmf_leptos::traits::has_name::NamedClass;
use crate::model::common::list::GenericListWithAdd;
use crate::model::common::table::GenericTable;
use tmflib::{tmf620::catalog::Catalog, HasId, HasName, HasDescription,HasValidity};
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf632::individual_v4::Individual;
// use tmf_leptos::common::time_period::TimePeriod;
use tmf_leptos::traits::has_validity::HasValidity;
use tmf_leptos::common::related_party::RelatedPartyList;
use tmf_leptos::traits::has_description::HasDescription;

fn get_catalogs() -> Vec<Catalog> {
    let cat1 = Catalog::new("Mobile")
        .description("Mobile plans / offers");
    let cat2 = Catalog::new("Fixed")
        .description("Fixed products and offerings");
    let cat3 = Catalog::new("MNS / ICT")
        .description("Overlay and ICT / MNS products");
    vec![
        cat1,
        cat2,
        cat3,
    ]
}

#[component]
pub fn CatalogAdd() -> impl IntoView {
    let valid_dirty : bool = false;
    let rp = RelatedParty::from(&Individual::new("John Smith"));
    let mut new_item = Catalog::new("New Catalog")
        .party(rp);
    let (name, set_name) = signal(new_item.get_name());
    let (desc, set_desc) = signal(new_item.get_description());
    let (valid,valid_write) = signal(valid_dirty);
    
    // Update proto-object
    name.with(|n| new_item.set_name(n));
    desc.with(|d| new_item.set_description(d));
    let mut validity = new_item.get_validity().unwrap_or_default();
    valid.with(|f| {
        // Flag has been set to dirty, so we know to update the validity in the proto-object
        if *f {
            new_item.set_validity(validity.clone());
            // Reset dirty flag
            valid_write.set(false);
        }
    });
    view! {
        <form>
            <NamedClass item=&new_item signal=set_name />
            <HasDescription description_read=desc description_write=set_desc />
            <HasValidity period=&mut validity dirty=valid_write />
            <RelatedPartyList item=&new_item />
            <button type="submit">"Submit"</button>
        </form>
        <div class="debug">"Will create new catalog called: " { name } " with description " { desc } " State: " { valid }</div>

        <Outlet />
    }
}

#[component]
pub fn CatalogList() -> impl IntoView {
    // let cat1 = Catalog::new("Components");
    // let cat2 = Catalog::new("Products");
    // let cat_list = vec![cat1, cat2];
    let cat_list = get_catalogs();
    
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
