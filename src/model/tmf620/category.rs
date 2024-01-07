//! Category Views

use leptos::*;
use leptos_router::*;

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
pub fn CategoryNode(cat : Category, position: u16) -> impl IntoView {
    let y1 = 5+(30*position);
    view! {
        <g class="catnode">
            <rect x="10" y={ y1 }  width="64" height="24" style="fill: grey; stroke: black; opacity: 0.5;"/>
            <text x="12" y={ 24+(position*30) }>{cat.name}</text>
        </g>    
    }
}

fn get_cat_by_id(id : String) -> Option<Category> {
    // Try to find a category node with the given id by making
    // TMF API calls via the back end.
    let cat1 = Category::new("A Root".to_string());
    Some(cat1)
}

#[component]
pub fn CategoryView() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    
    let cat1 = get_cat_by_id(id()).unwrap();
    let cat2 = Category::new("A Child".to_string());
    let cat3 = Category::new("B Child".to_string());
    
    //cat.id = Some(cat_id);
    view!{
        <div>
            <h2>"Category Details : " { cat1.name.as_ref().unwrap() } </h2>
            <svg>
                <CategoryNode cat=cat1 position=0/>
                <CategoryNode cat=cat2 position=1/>
                <CategoryNode cat=cat3 position=2/>
            </svg>
        </div>
    }
}