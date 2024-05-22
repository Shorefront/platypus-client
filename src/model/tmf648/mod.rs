/// Quote Module
/// 

use leptos::*;
use leptos_router::*;

use tmflib::{HasId, HasName};
use tmflib::tmf648::quote::Quote;

use crate::model::common::form::{BasicClass,NamedClass,Validity,SingleRow};

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an quote"</p>
    }
}

#[component(transparent)]
pub fn QuoteRoutes() -> impl IntoView {
    let mod_path = Quote::get_mod_path();
    let quote_path = Quote::get_class();
    let mod_path = Quote::get_mod_path();
    view! {
        <Route path=mod_path view=QuoteHome>
            <Route path=quote_path view=QuoteList >
                <Route path=":id" view=QuoteDetail />
                <Route path="add" view=QuoteAdd />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="" view=NoOptionView />
        </Route>
    }
}

#[component]
pub fn QuoteHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="quote">"Quote"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}

#[component]
pub fn QuoteList() -> impl IntoView {
    let mut quote1 = Quote::new();
    quote1.id = Some("Quote123".to_string());
    let quotes = vec![quote1];
    let add_href = format!("{}/add",Quote::get_class_href());
    view! {
        <div class="list">
        <table>
            {quotes.into_iter()
                .map(|_c| {
                    
                }).collect_view()
            }
        </table>
        <a href=add_href>"Add"</a>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn QuoteDetail() -> impl IntoView {
    view! {
        <p>"Quote Details"</p>
    }
}

#[component]
pub fn BasicQuote(mut quote : Quote) -> impl IntoView {
    let desc = quote.description.clone();
    let cat = quote.category.clone();
    let (name,set_name) = create_signal(quote.get_name());
    let (desc,set_desc) = create_signal(quote.description());
    let (cat,set_cat) = create_signal("Cat".to_string());
    name.with(|n| quote.set_name(n));
    view! {
        <fieldset>
            <legend>"Quote Information"</legend>
            <NamedClass item=quote.clone() signal=set_name/>
            <fieldset>
                <legend>"Details"</legend>
                <SingleRow id="description".to_string() label="Description".to_string() signal=set_desc />
                <SingleRow id="category".to_string() label="Category".to_string() signal=set_cat />
            </fieldset>
        </fieldset>
    }
}

#[component]
pub fn QuoteAdd() -> impl IntoView {
    let quote =Quote::new();
    let (_name,_set_name) = create_signal("New Quote".to_string());
    view! {
        <p>"Add Quote"</p>
        <BasicQuote quote=quote.clone() />
        <Validity item=quote />
    }
}

