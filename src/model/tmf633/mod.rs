//! Service Catalogue Module
//! 

use leptos::prelude::*;
use leptos_router::components::{Route,ParentRoute};

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
pub fn ServiceCatalogRoutes() -> impl IntoRender {
    view! {
        <ParentRoute path="/tmf-api/serviceCatalogManagement/v4" view=ServiceCatalogHome>
            <ParentRoute path="catalog" view=ServiceCatalogList >
                <Route path=":id" view=ServiceCatalogView />
                <Route path="add" view=ServiceCatalogForm />
                <Route path="" view=InvalidOptionView />
            </ParentRoute>
            <ParentRoute path="category" view=ServiceCategoryList >
                <Route path=":id" view=ServiceCategoryView />
                <Route path="add" view=ServiceCategoryForm />
                <Route path="" view=InvalidOptionView />
            </ParentRoute>
            <ParentRoute path="candidate" view=ServiceCandidateList >
                <Route path=":id" view=ServiceCandidateView />
                <Route path="add" view=ServiceCandidateForm />
                <Route path="" view=InvalidOptionView />
            </ParentRoute>
            <ParentRoute path="specification" view=ServiceSpecificationList >
                <Route path=":id" view=ServiceSpecificationView />
                <Route path="add" view=ServiceSpecificationForm />
                <Route path="" view=InvalidOptionView />
            </ParentRoute>
        <Route path="" view=NoOptionView />
        </ParentRoute>
    }
}

