//! Parent menu Module

use leptos::*;

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
                <a href="/tmf-api/tmf629/v4/customer">"TMF629"</a>
            </li>
            <li>
                <a href="/tmf-api/tmf632/v4">"TMF632"</a>
            </li>
            <li>
                <a href="/tmf-api/tmf648/v4/quote">"TMF648"</a>
            </li>
            <li>
                <a href="/tmf-api/tmf674/v4/site">"TMF674"</a>
            </li>
            <CostModelMenu />
        </ul>
    }
}