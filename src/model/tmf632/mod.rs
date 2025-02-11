
pub mod individual;
pub mod organization;

use leptos_router::{components::{Outlet, ParentRoute, Route}, MatchNestedRoutes};
use leptos_router::path;
use leptos::prelude::*;

use organization::{OrganizationList,OrganizationView};
use individual::{IndividualTable,IndividualView};

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component(transparent)]
pub fn PartyRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/tmf-api/partyManagement/v4") view=PartyHome>
            <ParentRoute path=path!("organization") view=OrganizationList >
                <Route path=path!(":id") view=OrganizationView />
                <Route path=path!("") view=NoOptionView />
            </ParentRoute>
            <ParentRoute path=path!("individual") view=IndividualTable >
                <Route path=path!(":id") view=IndividualView />
                <Route path=path!("") view=NoOptionView />
            </ParentRoute>
            <Route path=path!("") view=NoOptionView />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn PartyHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/partyManagement/v4/organization">"Organisation"</a></li>
                <li><a href="/tmf-api/partyManagement/v4/individual">"Individual"</a></li>
            </ul>
        </nav>
        <Outlet />
    }
}