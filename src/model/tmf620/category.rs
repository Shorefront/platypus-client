//! Category Views

use leptos::*;
use leptos_router::*;

use crate::model::common::GenericTable;
use tmflib::{tmf620::category::Category, HasId};
use reqwest_wasm::Client;

async fn get_cat() -> Vec<Category> {
    // This is where we create an API call back into back end
    
    
    let cat1 = Category::new("Root".to_string());
    let href = cat1.get_href();
    let client = Client::new();
    let res = client.get(href).send().await;
    match res {
        Ok(r) => {
            // Parse result into Vec<Category>
            let body = r.text().await;
            match body {
                Ok(b) => {
                    let cat_list : Vec<Category> = serde_json::from_str(b.as_ref()).unwrap();
                    cat_list
                },
                Err(_e) => {
                    vec![]
                },
            }
        },
        Err(e) => {
            // output error, return empty
            println!("Could not fetch categories: {}",e);
            vec![]
        }
    }
}

#[component]
pub fn CategoryTable() -> impl IntoView {


    let load_cat_list = create_resource(|| (), |_| async move {
        get_cat().await
    });
    let cat_list = load_cat_list.get();
    let categories = match cat_list {
        Some(c) => c,
        None => vec![],
    };

    view! {
        <div class="list">
            <GenericTable items=categories/>
            <a href="/tmf-api/productCatalogManagement/v4/category/add">"Add New"</a>
        </div> 
        <div class="detail">
            <Outlet />
        </div>
    }
    
}

#[component]
pub fn CategorySelection(signal : WriteSignal<String>) -> impl IntoView {
    // TODO: Get a list of categories to generate the selection
    view! {
        <label for="parent">"Parent Node"</label>
        <select id="parent" on:change=move |ev| {
            signal.set(event_target_value(&ev));
        }>   
            <option value="rootId">"Root"</option>
            <option value="childa">"Child A"</option>
            <option value="childb">"Child B"</option>
        </select>
    }
}

#[component]
pub fn CategoryAdd() -> impl IntoView {
    let (name, set_name) = create_signal("New Category".to_string());
    let (parent,set_parent) = create_signal("Root".to_string());
    view! {
        <div class="form">
        <h2>"Add Category"</h2>
        <label for="name">Name </label>
        <input id="name" type="text"
            on:input=move |ev| {
                // Since we are not using nightly we have to call .set() on the WriteSignal.
                // Nightly allows us to treat WriteSignal as a function, e.g. set_name()
                set_name.set(event_target_value(&ev));            
            }
            prop:value=name
        />
        <CategorySelection signal=set_parent/>
        <p>"Will create new category called: " {name} " with parent: " { parent }</p>
        </div>
    }
}

#[component]
pub fn CategoryNode(cat : Category, position: u16) -> impl IntoView {
    let y1 = 5+(30*position);
    let stroke = match cat.root() {
        true => "blue".to_string(),
        false => "black".to_string(),
    };
    let style = format!("fill: grey; stroke: {}; opacity: 0.5;",stroke);
    view! {
        <g class="catnode">
            <rect x="10" y={ y1 }  width="64" height="24" rx="5" style=style/>
            <text x="12" y={ 24+(position*30) }>{cat.name}</text>
        </g>
        <g class="catdetail">
            <rect x="80" y={ y1 } width="256" height="24" />
            <text x="82" y={ 24+(position*30) }>{cat.description}</text>
        </g>
    }
}

#[component]
pub fn CategoryView() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    
    // let cat1 = |_| {
    //     spawn_local(async {
    //         get_cat_by_id(id()).await;
    //     });
    // };
    // let cat_id = id();
    // let mut output = String::new();
    // let get_cat = move |cat_id,cat_output : &mut String| {
    //     spawn_local(async {
    //         match get_cat_by_id(cat_id).await {
    //             Ok(_) => "one".to_string(),
    //             Err(_) => "two".to_string(),            
    //         };
    //         cat_output = "Test".to_string();
    //     });
    // };
    // let cat1 = get_cat(cat_id,&mut output);
    let cat1 = Category::new("Root".to_string());
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