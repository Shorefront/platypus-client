//! Service Catalogue Module
//! 

use leptos::*;
use leptos_router::*;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}


#[component(transparent)]
pub fn ServiceCatalogRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/serviceCatalogManagement/v4" view=ServiceCatalogHome>
            <Route path="" view=NoOptionView />
        </Route>
    }
}

#[component]
pub fn ServiceCatalogHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/serviceCatalogManagement/v4/catalog">"Service Catalog"</a></li>
                <li><a href="/tmf-api/serviceCatalogManagement/v4/category">"Service Category"</a></li>
                <li><a href="/tmf-api/serviceCatalogManagement/v4/candidate">"Service Candidate"</a></li>
                <li><a href="/tmf-api/serviceCatalogManagement/v4/specification">"Service Specification"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}