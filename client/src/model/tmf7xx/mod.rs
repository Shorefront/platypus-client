//! TMF7XX Cost Management Module

use leptos::prelude::*;
use leptos_router::*;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component]
pub fn CostModelMenu() -> impl IntoView {
    view! {
        <li>
            <a href="/tmf-api/tmf7xx/v5/costModel">"TMF7XX"</a>
        </li>
    }
}

#[component(transparent)]
pub fn CostModelRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
    <Route path="/tmf-api/tmf7xx/v5" view=CostHome>
        <Route path="costModel" view=CostModelList >
            <Route path=":id" view=CostModelDetail />
            <Route path="" view=NoOptionView />
        </Route>
        <Route path="" view=NoOptionView />
    </Route>
    }
}

#[component]
pub fn CostHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/costModel/v5/costModel">"CostModel"</a></li>
            </ul>
        </nav>
    }
}

#[component]
pub fn CostModelList() -> impl IntoView {
    view! {}
}

#[component]
pub fn CostModelDetail() -> impl IntoView {
    view! {}
}
