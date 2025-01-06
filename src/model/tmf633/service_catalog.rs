//! Service Catalog Module
//! 
//! 
//! 

use leptos::prelude::*;
use leptos_router::components::Outlet;
use crate::model::common::list::GenericListWithAdd;
use tmflib::tmf633::service_specification::ServiceSpecification;

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

#[component]
pub fn ServiceCatalogList() -> impl IntoView {
    let spec1 = ServiceSpecification::new("Spec1");
    let spec2 = ServiceSpecification::new("Spec2");
    let items = vec![spec1,spec2];
    view! {
        <div class="list">
            <GenericListWithAdd items=items/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn ServiceCatalogView() -> impl IntoView {
    view! {<p class="error">"Not yet implemented!"</p>}
}

pub fn ServiceCatalogForm() -> impl IntoView {
    view! {<p class="error">"Not yet implemented!"</p>}
}