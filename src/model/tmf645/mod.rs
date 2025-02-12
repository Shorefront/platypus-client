//! TMF645 Service Qualification Module
//! This module defines the Service Qualification Model

use leptos::prelude::*;
use leptos_router::components::{Route,ParentRoute,Outlet};
use leptos_router::{path,MatchNestedRoutes};

use crate::model::common::list::DescListWithAdd;

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
    let sites = vec![qual1];
    view! {
        <div class="list">
            <DescListWithAdd items=sites />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn CheckQualificaitonDetail() -> impl IntoView {
    view! {
        <p>"Check Qualification Details"</p>
    }
}

#[component(transparent)]
pub fn ServiceQualificationRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/tmf-api/serviceQualification/v4") view=ServiceQualificationHome>
            <ParentRoute path=path!("checkServiceQualification") view=CheckQualificationList >
                <Route path=path!(":id") view=CheckQualificaitonDetail />
                <Route path=path!("") view=NoOptionView />
            </ParentRoute>
            <Route path=path!("") view=NoOptionView />
        </ParentRoute>
    }
    .into_inner()
}