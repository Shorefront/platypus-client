//! TMF674 Geographic Sites
//!

use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route};
use leptos_router::{path, MatchNestedRoutes};

use crate::model::common::list::GenericListWithAdd;

// use tmflib::HasId;
#[cfg(feature = "V4")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;
#[cfg(feature = "V5")]
use tmflib::tmf674::geographic_site_v5::GeographicSite;
use tmflib::HasName;

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
    let sites = vec![site1, site2];
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

#[component]
pub fn GeographicSiteAdd() -> impl IntoView {
    let new_item = GeographicSite::new("New Site".to_string());
    view! {
        <form>
            <fieldset>
                <legend>"New Geographic Site"</legend>
                <label for="name">"Name"</label>
                <input type="text" id="name" name="name" value={new_item.get_name()} /><br />
                <label for="description">"Description"</label>
                <input type="text" id="description" name="description" /><br />
            </fieldset>
            <button type="submit">"Submit"</button>
        </form>
        <Outlet />
    }
}

#[component(transparent)]
pub fn GeographicSiteRoutes() -> impl MatchNestedRoutes + Clone {
    // let site_path = GeographicSite::get_class();
    view! {
        <ParentRoute path=path!("/tmf-api/geographicSiteManagement/v4") view=GeographicSiteHome>
            <ParentRoute path=path!("geographicSite") view=GeographicSiteList >
                <Route path=path!("add") view=GeographicSiteAdd />
                <Route path=path!(":id") view=GeographicSiteDetail />
                <Route path=path!("") view=NoOptionView />
            </ParentRoute>
            <Route path=path!("") view=NoOptionView />
        </ParentRoute>
    }
    .into_inner()
}
