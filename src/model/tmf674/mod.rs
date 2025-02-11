//! TMF674 Geographic Sites
//! 

use leptos::prelude::*;
use leptos_router::components::{Route,ParentRoute,Outlet};
use leptos_router::{path,MatchNestedRoutes};

use crate::model::common::list::GenericListWithAdd;

// use tmflib::HasId;
#[cfg(feature = "tmf674_v4")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;
#[cfg(feature = "tmf674_v5")]
use tmflib::tmf674::geographic_site_v5::GeographicSite;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component]
pub fn GeographicSiteHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="geographicSite">"Site"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}

#[component]
pub fn GeographicSiteList() -> impl IntoView {
    let site1 = GeographicSite::new("Branch123".to_string());
    let site2 = GeographicSite::new("Head Office".to_string());
    let sites = vec![site1,site2];
    view! {
        <div class="list">
            <GenericListWithAdd items=sites />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn GeographicSiteDetail() -> impl IntoView {
    view! {
        <p>"Site Details"</p>
    }
}

#[component(transparent)]
pub fn GeographicSiteRoutes() -> impl MatchNestedRoutes + Clone {
    // let site_path = GeographicSite::get_class();
    view! {
<<<<<<< HEAD
        <ParentRoute path=path!("/tmf-api/geographicSiteManagement/v4") view=GeographicSiteHome>
            <ParentRoute path=path!("geographicSite") view=GeographicSiteList >
                <Route path=path!(":id") view=GeographicSiteDetail />
                <Route path=path!("") view=NoOptionView />
            </ParentRoute>
            <Route path=path!("") view=NoOptionView />
=======
        <ParentRoute path="/tmf-api/geographicSiteManagement/v4" view=GeographicSiteHome>
            <ParentRoute path=site_path view=GeographicSiteList >
                <Route path=":id" view=GeographicSiteDetail />
                <Route path="" view=NoOptionView />
            </ParentRoute>
            <Route path="" view=NoOptionView />
>>>>>>> master
        </ParentRoute>
    }
    .into_inner()
}