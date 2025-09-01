//! TMF645 Service Qualification Module
//! This module defines the Service Qualification Model

use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route};
use leptos_router::{path, MatchNestedRoutes};

use crate::model::common::list::DescList;

use tmflib::tmf645::check_service_qualification::CheckServiceQualification;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component]
pub fn InvalidOptionView() -> impl IntoView {
    view! {
        <p class="error">"Invalid Option"</p>
    }
}

#[component]
pub fn ServiceQualificationHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="/tmf-api/serviceQualificationManagement/v4/checkServiceQualification">"Check"</a></li>
                <li><a href="/tmf-api/serviceQualificationManagement/v4/queryServiceQualification">"Query"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}

#[component]
pub fn CheckQualificationList() -> impl IntoView {
    let qual1 = CheckServiceQualification::new("Qual1");
    let qual2 = CheckServiceQualification::new("Qual2");

    let quals = vec![qual1, qual2];
    view! {
        <div class="list">
            <DescList items=quals />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn CheckQualificationDetail() -> impl IntoView {
    view! {
        <p>"Check Qualification Details"</p>
    }
}

#[component(transparent)]
pub fn ServiceQualificationRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/tmf-api/serviceQualificationManagement/v4") view=ServiceQualificationHome>
            <ParentRoute path=path!("checkServiceQualification") view=CheckQualificationList >
                <Route path=path!(":id") view=CheckQualificationDetail />
                <Route path=path!("") view=NoOptionView />
            </ParentRoute>
            <Route path=path!("") view=NoOptionView />
        </ParentRoute>
    }
    .into_inner()
}
