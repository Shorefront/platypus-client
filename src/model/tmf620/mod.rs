//! TMF632 models
pub mod catalog;
pub mod category;
pub mod product_specification;
pub mod product_offering;
pub mod product_offering_price;

use leptos::*;
use leptos_router::*;

use catalog::{CatalogList,CatalogDetails};
use category::{CategoryTable,CategoryView};
use product_offering::{ProductOfferingTable,ProductOfferingView};
use product_specification::{ProductSpecificationTable,ProductSpecificationView};
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
        <Route path="/tmf-api/productCatalogManagement/v4" view=CatalogHome>
            <Route path="catalog" view=CatalogList >
                <Route path=":id" view=CatalogDetails /> 
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="category" view=CategoryTable >
                <Route path=":id" view=CategoryView />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="productOffering" view=ProductOfferingTable >
                <Route path=":id" view=ProductOfferingView />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="productSpecification" view=ProductSpecificationTable >
                <Route path=":id" view=ProductSpecificationView />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="productOfferingPrice" view=ProductOfferingPriceList >
                <Route path=":id" view=ProductOfferingPriceDetail />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="" view=NoOptionView />
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
                <li><a href="/tmf-api/productCatalogManagement/v4/productOffering">"Offering"</a></li>
                <li><a href="/tmf-api/productCatalogManagement/v4/productSpecification">"Specification"</a></li>
                <li><a href="/tmf-api/productCatalogManagement/v4/productOfferingPrice">"Pricing"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}
