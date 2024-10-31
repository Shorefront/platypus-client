//! Parent menu Module

use leptos::*;

#[cfg(feature = "tmf7xx")]
use crate::model::tmf7xx::CostModelMenu;

#[component]
pub fn Menu() -> impl IntoView {
    view!{
        <ul class="menu">
            <li><a href="/">"Home"</a></li>
            <li>
                <a href="/tmf-api/productCatalogManagement/v4">"TMF620"</a>
            </li>
            <li>
                <a href="/tmf-api/customerManagement/v4/customer">"TMF629"</a>
            </li>
            <li>
                <a href="/tmf-api/partyManagement/v4">"TMF632"</a>
            </li>
            <li>
                <a href="/tmf-api/serviceCatalogManagement/v4">"TMF633"</a>
            </li>
            <li>
                <a href="/tmf-api/quoteManagement/v4/quote">"TMF648"</a>
            </li>
            <li>
                <a href="/tmf-api/geographicSiteManagement/v4/geographicSite">"TMF674"</a>
            </li>
        </ul>
    }
}