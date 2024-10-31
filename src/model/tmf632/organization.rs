//! Organization Model

#[cfg(feature = "tmf632_v4")]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(feature = "tmf632_v5")]
use tmflib::tmf632::organization_v5::Organization;

use leptos::*;
use leptos_router::*;


use crate::model::common::list::GenericListWithAdd;

#[component]
pub fn OrganizationList() -> impl IntoView {
    let org1 = Organization::new(String::from("Shorefront"));
    let org2 = Organization::new(String::from("Platypus Potential"));
    let org3 = Organization::new(String::from("Apples"));
    let orgs = vec![org1,org2,org3];
    view! {
        <div class="list">
            <GenericListWithAdd items=orgs/>
        </div>
        <div class="detail">
            <Outlet/>
        </div>
    }
}

#[derive(Default,Params,PartialEq)]
struct OrgParams {
    id: Option<String>,
}

#[component]
pub fn OrganizationView() -> impl IntoView {
    let _params = use_params::<OrgParams>();

    view! {
        <h3>"Organization View"</h3>
        <table>
        </table>
    }
}