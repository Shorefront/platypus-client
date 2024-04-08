//! TMF674 Geographic Sites
//! 

use leptos::*;
use leptos_router::*;

use tmflib::HasId;
use tmflib::tmf674::geographic_site::GeographicSite;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component]
pub fn GeographicSiteHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="geographicSite">"Site"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}

#[component]
pub fn GeographicSiteList() -> impl IntoView {
    let site1 = GeographicSite::new("Branch123".to_string());
    let site2 = GeographicSite::new("Head Office".to_string());
    let sites = vec![site1,site2];
    view! {
        <div class="list">
            <table>
                <tr><th>"Site Id"</th></tr>
                {sites.into_iter()
                    .map(|gs| {
                        view! { <tr><td><a href={ gs.get_href()}> { gs.name } </a></td></tr> }
                    }).collect_view()
                }
            </table>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn GeographicSiteDetail() -> impl IntoView {
    view! {
        <p>"Site Details"</p>
    }
}

#[component(transparent)]
pub fn GeographicSiteRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/geographicSiteManagement/v4" view=GeographicSiteHome>
            <Route path="geographicSite" view=GeographicSiteList >
                <Route path=":id" view=GeographicSiteDetail />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="" view=NoOptionView />
        </Route>
    }
}