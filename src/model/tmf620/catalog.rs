use leptos::*;

use tmflib::tmf620::catalog::Catalog;

use crate::GenericTable;

#[component]
pub fn CatalogTable() -> impl IntoView {
    let cat1 = Catalog::new("Cat1");
    let cat2 = Catalog::new("Cat2");
    let catalogs = vec![cat1,cat2];
    view! {
            <p>"A Catalog Table"</p>
            <GenericTable items=catalogs/>
    }
}

#[component]
pub fn CatalogView(_id : String) -> impl IntoView {
    view! {
        <p>
        "Some catalogue view"
        </p>
    }
}