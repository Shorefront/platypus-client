use leptos::*;

use tmflib::tmf620::catalog::Catalog;
use super::super::common::GenericTable;

#[component]
pub fn CatalogTable() -> impl IntoView {
    let cat1 = Catalog::new("Design");
    let cat2 = Catalog::new("Run");
    let catalogs = vec![cat1,cat2];
    view! {
        <GenericTable items=catalogs/>
    }
}

#[component]
pub fn CatalogView() -> impl IntoView {
    view! {
        <p>"Some data"</p>
    }
}