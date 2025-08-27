//! Category Views

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use log::{error, info};

use crate::model::common::form::NamedClass;
use crate::model::common::form::SingleRow;
use crate::model::common::list::GenericListWithAdd;
use reqwest_wasm::Client;
use tmflib::{tmf620::category::Category, HasId, HasName};
use tmf_leptos::common::related_party::RelatedPartyList;
use tmf_leptos::common::time_period::TimePeriod;

const DEFAULT_HOST: &str = "http://localhost:8000";

async fn get_cat() -> Vec<Category> {
    // This is where we create an API call back into back end

    let href = format!("{}{}", DEFAULT_HOST, Category::get_class_href());
    let client = Client::new();
    let res = client.get(href).send().await;
    match res {
        Ok(r) => {
            // Parse result into Vec<Category>
            let body = r.text().await;
            match body {
                Ok(b) => {
                    let cat_list: Vec<Category> = serde_json::from_str(b.as_ref()).unwrap();
                    info!("Found {} categories", cat_list.len());
                    cat_list
                }
                Err(e) => {
                    error!("Could not parse JSON: {}", e);
                    vec![]
                }
            }
        }
        Err(e) => {
            // output error, return empty
            error!("Could not fetch categories: {}", e);
            vec![]
        }
    }
}

#[component]
pub fn CategoryTable() -> impl IntoView {
    let cat1 = Category::new("Component");
    let cat2 = Category::new("Product");

    let cat_list = vec![cat1, cat2];

    let _add_href = format!("{}/add", Category::get_class_href());

    view! {
        <div class="list">
            <GenericListWithAdd items=cat_list />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn CategorySelection(signal: WriteSignal<String>) -> impl IntoView {
    // TODO: Get a list of categories to generate the selection
    view! {
        <fieldset>
            <legend>Heirarchy</legend>
            <label for="parent">"Parent Node"</label>
            <select id="parent" on:change=move |ev| {
                signal.set(event_target_value(&ev));
            }>
                <option value="rootId">"Root"</option>
                <option value="childa">"Child A"</option>
                <option value="childb">"Child B"</option>
            </select>
        </fieldset>
    }
}

#[component]
pub fn CategoryAdd() -> impl IntoView {
    let (name, set_name) = signal("New Category".to_string());
    let (parent, set_parent) = signal("Root".to_string());
    let (get_desc, set_desc) = signal("Description".to_string());
    let (version, set_version) = signal("1".to_string());
    let mut new_cat = Category::new(name.get());
    name.with(|n| new_cat.set_name(n));
    view! {
        <form>
            <NamedClass item=&new_cat signal=set_name />
            <fieldset>
                <legend>Details</legend>
                <SingleRow id="description".to_string() label="Description".to_string() read=get_desc write=set_desc />
                <SingleRow id="version".to_string() label="Version".to_string() read=version write=set_version />
            </fieldset>
            <TimePeriod item=&new_cat />
            <CategorySelection signal=set_parent/>
        </form>
        <p>"Will create new category [" {version} "]:  "{ name } " with parent: "{ parent }" and description "{ get_desc }</p>
    }
}

#[component]
pub fn CategoryNode(cat: Category, position: u16) -> impl IntoView {
    let y1 = 5 + (30 * position);
    let stroke = match cat.root() {
        true => "blue".to_string(),
        false => "black".to_string(),
    };
    let style = format!("fill: grey; stroke: {}; opacity: 0.5;", stroke);
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
    let _id = move || params.with(|params| params.get("id").clone().unwrap_or_default());


    let cat1 = Category::new("Root Category".to_string());
    let cat2 = Category::new("Component".to_string());
    let cat3 = Category::new("Product".to_string());

    //cat.id = Some(cat_id);
    view! {
        <div>
            <svg>
                <CategoryNode cat=cat1 position=0/>
                <CategoryNode cat=cat2 position=1/>
                <CategoryNode cat=cat3 position=2/>
            </svg>
        </div>
    }
}
