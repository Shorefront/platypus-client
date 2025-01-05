//! TMF632 models
pub mod catalog;
pub mod category;
pub mod product_specification;
pub mod product_offering;
pub mod product_offering_price;

use leptos::prelude::*;
use leptos_router::components::{Route,ParentRoute,Outlet};
use leptos_router::{path, MatchNestedRoutes};

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
pub fn CatalogRoutes() -> impl MatchNestedRoutes + Clone {
    view!{
        <ParentRoute path=path!("/tmf-api/productCatalogManagement/v4") view=CatalogHome>
            <ParentRoute path=path!("catalog") view=CatalogList >
                <Route path=path!(":id") view=CatalogDetails /> 
                <Route path=path!("add") view=CatalogAdd />
                <Route path=path!("") view=NoOptionView />
                <Route path=path!("/") view=NoOptionView />
            </ParentRoute>
            <ParentRoute path=path!("category") view=CategoryTable >
                <Route path=path!(":id") view=CategoryView />
                <Route path=path!("add") view=CategoryAdd />
                <Route path=path!("") view=NoOptionView />
                <Route path=path!("/") view=NoOptionView />
            </ParentRoute>
            <ParentRoute path=path!("productOffering") view=ProductOfferingTable >
                <Route path=path!(":id") view=ProductOfferingView />
                <Route path=path!("") view=NoOptionView />
                <Route path=path!("/") view=NoOptionView />
            </ParentRoute>
            <ParentRoute path=path!("productSpecification") view=ProductSpecificationTable >
                <Route path=path!(":id") view=ProductSpecificationView />
                <Route path=path!("add") view=ProductSpecificationAdd />
                <Route path=path!("") view=NoOptionView />
                <Route path=path!("/") view=NoOptionView />
            </ParentRoute>
            <ParentRoute path=path!("productOfferingPrice") view=ProductOfferingPriceList >
                <Route path=path!(":id") view=ProductOfferingPriceDetail />
                <Route path=path!("") view=NoOptionView />
                <Route path=path!("/") view=NoOptionView />
            </ParentRoute>
            <Route path=path!("") view=NoOptionView />
            <Route path=path!("/") view=NoOptionView />
        </ParentRoute>
    }
    .into_inner()
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
