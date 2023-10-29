//! Category Views

use leptos::*;
use crate::model::common::GenericTable;
use tmflib::tmf620::category::Category;

#[component]
pub fn CategoryTable() -> impl IntoView {
    let cat1 = Category::new("Root".to_string());
    let categories = vec![cat1];
    view! {
        <GenericTable items=categories/>
    }
    
}

#[component]
pub fn CategoryView() -> impl IntoView {
    view!{
        <div>
            <h2>"Category Details"</h2>
            <table>
            </table>
        </div>
    }
}