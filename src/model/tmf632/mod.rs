
pub mod individual;
pub mod organization;

use leptos_router::*;
use leptos::*;

use organization::{OrganizationList,OrganizationView};
use individual::{IndividualTable,IndividualView};

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component(transparent)]
pub fn PartyRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/partyManagement/v4" view=PartyHome>
            <Route path="organization" view=OrganizationList >
                <Route path=":id" view=OrganizationView />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="individual" view=IndividualTable >
                <Route path=":id" view=IndividualView />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="" view=NoOptionView />
        </Route>
    }
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