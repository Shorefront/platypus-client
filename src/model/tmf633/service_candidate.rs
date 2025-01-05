//! Service Candidate Module


use leptos::prelude::*;
use leptos_router::components::Outlet;

use tmflib::tmf633::service_candidate::ServiceCandidate;
use crate::model::common::list::GenericListWithAdd;

#[component]
pub fn ServiceCandidateList() -> impl IntoView {
    let candidate1 = ServiceCandidate::new("Candidate 1");
    let candidate2 = ServiceCandidate::new("Candidate 2");
    let items = vec![candidate1,candidate2];
    view! {
        <div class="list">
            <GenericListWithAdd items=items/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
    
}

#[component]
pub fn ServiceCandidateView() -> impl IntoView {
    view!{
        <p>"Candidate: "</p>
    }
}

#[component]
pub fn ServiceCandidateForm() -> impl IntoView {
    view!{

    }
}
