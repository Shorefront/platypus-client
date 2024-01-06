//! Category Views

use leptos::*;
use leptos_router::Outlet;
use crate::model::common::GenericTable;
use tmflib::tmf620::category::Category;

#[component]
pub fn CategoryTable() -> impl IntoView {
    let cat1 = Category::new("Root".to_string());
    let categories = vec![cat1];
    view! {
        <div class="list">
            <GenericTable items=categories/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
    
}

#[component]
pub fn CategoryView() -> impl IntoView {
    view!{
        <div>
            <h2>"Category Details"</h2>
            <svg>
                <g class="catnode">
                    <rect x="10" y="10" width="64" height="24" style="fill: grey; stroke: black; opacity: 0.5;"/>
                    <text x="12" y="24">Root</text>
                </g>
            </svg>
        </div>
    }
}