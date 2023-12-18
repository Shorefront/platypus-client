//! TMF632 models
pub mod catalog;
pub mod category;
pub mod product_specification;
pub mod product_offering;

use leptos::*;
use leptos_router::*;

use catalog::{CatalogTable,CatalogView};
use category::CategoryTable;

#[component(transparent)]
pub fn CatalogRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/productCatalogManagement/v4/" view=CatalogHome>
            <Route path="catalog" view=CatalogTable >
                <Route path=":id" view=CatalogView />
            </Route>
            <Route path="category" view=CategoryTable />
            <Route path="" view=|| {
                view! {
                    <p>"Please select something"</p>
                }
            } />
        </Route>
    }
}

#[component]
pub fn CatalogHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/productCatalogManagement/v4/catalog">"Catalog"</a></li>
                <li><a href="/tmf-api/productCatalogManagement/v4/category">"Category"</a></li>
                <li>Offering</li>
                <li>Specification</li>
                <li>Pricing</li>
            </ul>
        </nav>

        <Outlet />
    }
}
