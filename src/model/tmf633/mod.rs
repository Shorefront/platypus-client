//! Service Catalogue Module
//! 

use leptos::prelude::*;
use leptos_router::components::{Outlet,Route,ParentRoute};
use leptos_router::{MatchNestedRoutes,path};

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

#[component]
pub fn ServiceCatalogHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/serviceCatalogManagement/v4/catalog">"Catalog"</a></li>
                <li><a href="/tmf-api/serviceCatalogManagement/v4/category">"Category"</a></li>
                <li><a href="/tmf-api/serviceCatalogManagement/v4/serviceCandidate">"Candidate"</a></li>
                <li><a href="/tmf-api/serviceCatalogManagement/v4/specification">"Specification"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}


#[component(transparent)]
pub fn ServiceCatalogRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/tmf-api/serviceCatalogManagement/v4") view=ServiceCatalogHome>
            <ParentRoute path=path!("catalog") view=ServiceCatalogList >
                <Route path=path!(":id") view=ServiceCatalogView />
                <Route path=path!("add") view=ServiceCatalogForm />
                <Route path=path!("") view=InvalidOptionView />
            </ParentRoute>
            <ParentRoute path=path!("category") view=ServiceCategoryList >
                <Route path=path!(":id") view=ServiceCategoryView />
                <Route path=path!("add") view=ServiceCategoryForm />
                <Route path=path!("") view=InvalidOptionView />
            </ParentRoute>
            <ParentRoute path=path!("serviceCandidate") view=ServiceCandidateList >
                <Route path=path!(":id") view=ServiceCandidateView />
                <Route path=path!("add") view=ServiceCandidateForm />
                <Route path=path!("") view=InvalidOptionView />
            </ParentRoute>
            <ParentRoute path=path!("specification") view=ServiceSpecificationList >
                <Route path=path!(":id") view=ServiceSpecificationView />
                <Route path=path!("add") view=ServiceSpecificationForm />
                <Route path=path!("") view=InvalidOptionView />
            </ParentRoute>
        <Route path=path!("") view=NoOptionView />
        </ParentRoute>
    }
    .into_inner()
}

