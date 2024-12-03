//! Service Catalogue Module
//! 

use components::{Outlet, ParentRoute, Route};
use leptos::prelude::*;
use leptos_router::*;

pub mod service_catalog;
pub mod service_category;
pub mod service_candidate;
pub mod service_specification;

use service_catalog::*;
use service_category::*;
use service_candidate::*;
use service_specification::*;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component]
pub fn InvalidOptionView() -> impl IntoView {
    view! {
        <p class="error">"Invalid Option"</p>
    }
}


#[component(transparent)]
pub fn ServiceCatalogRoutes() -> impl IntoView {
    view! {
        <ParentRoute path="/tmf-api/serviceCatalogManagement/v4" view=ServiceCatalogHome>
            <Route path="" view=NoOptionView />
        </ParentRoute>
    }
}

