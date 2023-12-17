//! Organization Model

use tmflib::tmf632::organization::Organization;

use leptos::*;
use leptos_router::Outlet;

use crate::GenericTable;

#[component]
pub fn OrganizationTable() -> impl IntoView {
    let org1 = Organization::new(String::from("Shorefront"));
    let org2 = Organization::new(String::from("Platypus Potential"));
    let org3 = Organization::new(String::from("Apples"));
    let orgs = vec![org1,org2,org3];
    view! {
        <div style="width:80%; padding: 5px;">
        <div style="float: left;">
            <GenericTable items=orgs/>
        </div>
        <div style="float: right;">
            <Outlet/>
        </div>
        </div>
    }
}

#[component]
pub fn OrganizationView() -> impl IntoView {
    view! {
        <h3>Organization View</h3>
        <table>
        </table>
    }
}