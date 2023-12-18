
pub mod individual;
pub mod organization;

use leptos_router::*;
use leptos::*;

use organization::{OrganizationTable,OrganizationView};
use individual::{IndividualTable,IndividualView};

#[component(transparent)]
pub fn PartyRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/tmf632/v4" view=PartyHome>
            <Route path="organisation" view=OrganizationTable >
                <Route path=":id" view=OrganizationView />
            </Route>
            <Route path="individual" view=IndividualTable >
                <Route path=":id" view=IndividualView />
            </Route>
            <Route path="" view=|| {
                view! {
                    <p>"Please select something"</p>
                }
            } />
        </Route>
    }
}

#[component]
pub fn PartyHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/tmf632/v4/organization">"Organisation"</a></li>
                <li><a href="/tmf-api/tmf632/v4/individual">"Individual"</a></li>
            </ul>
        </nav>
        <div>
        <Outlet />
        </div>
    }
}