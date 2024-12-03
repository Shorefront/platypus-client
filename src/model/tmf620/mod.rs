//! TMF632 models
pub mod catalog;
pub mod category;
pub mod product_specification;
pub mod product_offering;
pub mod product_offering_price;

use leptos::prelude::*;
use leptos_router::*;
use components::{Outlet, Route, Router, ParentRoute};

use catalog::{CatalogList,CatalogDetails,CatalogAdd};
use category::{CategoryTable,CategoryView,CategoryAdd};
use product_offering::{ProductOfferingTable,ProductOfferingView};
use product_specification::{ProductSpecificationTable,ProductSpecificationView,ProductSpecificationAdd};
use product_offering_price::{ProductOfferingPriceList,ProductOfferingPriceDetail};

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component(transparent)]
pub fn CatalogRoutes() -> impl IntoView {
    view! {
        <ParentRoute path="/tmf-api/productCatalogManagement/v4" view=CatalogHome>
            <ParentRoute path="catalog" view=CatalogList >
                <Route path=":id" view=CatalogDetails /> 
                <Route path="add" view=CatalogAdd />
                <Route path="" view=NoOptionView />
            </ParentRoute>
            <ParentRoute path="category" view=CategoryTable >
                <Route path=":id" view=CategoryView />
                <Route path="add" view=CategoryAdd />
                <Route path="" view=NoOptionView />
            </ParentRoute>
            <ParentRoute path="productOffering" view=ProductOfferingTable >
                <Route path=":id" view=ProductOfferingView />
                <Route path="" view=NoOptionView />
            </ParentRoute>
            <ParentRoute path="productSpecification" view=ProductSpecificationTable >
                <Route path=":id" view=ProductSpecificationView />
                <Route path="add" view=ProductSpecificationAdd />
                <Route path="" view=NoOptionView />
            </ParentRoute>
            <ParentRoute path="productOfferingPrice" view=ProductOfferingPriceList >
                <Route path=":id" view=ProductOfferingPriceDetail />
                <Route path="" view=NoOptionView />
            </ParentRoute>
            <Route path="" view=NoOptionView />
        </ParentRoute>
    }
}

#[component]
pub fn CatalogHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/productCatalogManagement/v4/catalog">"Catalog"</a></li>
                <li><a href="/tmf-api/productCatalogManagement/v4/category">"Category"</a></li>
                <li><a href="/tmf-api/productCatalogManagement/v4/productOffering">"Offering"</a></li>
                <li><a href="/tmf-api/productCatalogManagement/v4/productSpecification">"Specification"</a></li>
                <li><a href="/tmf-api/productCatalogManagement/v4/productOfferingPrice">"Pricing"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}
