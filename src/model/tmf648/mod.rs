/// Quote Module
/// 

use leptos::*;
use leptos_router::*;

use tmflib::HasId;
use tmflib::tmf648::quote::Quote;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component]
pub fn QuoteHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li>Quote</li>
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
    view! {
        <div class="list">
            <table>
                <tr><th>"Quote Id"</th></tr>
                {quotes.into_iter()
                    .map(|q| {
                        view! { <tr><td><a href={ q.get_href()}>{ q.id } </a></td></tr> }
                    }).collect_view()
                }
            </table>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn QuoteDetail() -> impl IntoView {
    view! {
        <p>"Quote Detail View"</p>
    }
}

#[component(transparent)]
pub fn QuoteRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/tmf648/v4" view=QuoteHome>
            <Route path="quote" view=QuoteList >
                <Route path=":id" view=QuoteDetail />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="" view=NoOptionView />
        </Route>
    }
}