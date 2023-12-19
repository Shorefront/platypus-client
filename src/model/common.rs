//! Common Table
//! 
//! 
use leptos::*;
use tmflib::{HasId,HasName};

#[component]
fn TableRow<T : HasId + HasName>(item : T) -> impl IntoView {
    view! {
        <tr><td><a href={item.get_href()}>{ item.get_name() }</a></td></tr>
    }
}

#[component]
pub fn GenericTable<T : HasId + HasName>(items : Vec<T>) -> impl IntoView {
    view! {
        <div class="gentable">
            <table>
                <tr>
                    <th>{ T::get_class()}</th>
                </tr>
         
                {items.into_iter()
                    .map(|c| {
                        TableRow(TableRowProps{ item: c})
                    }).collect_view()
                }
            </table>
        </div>
    }
}

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <div class="banner">Platypus</div>
    }
}

#[component]
pub fn Menu() -> impl IntoView {
    view!{
        <ul class="menu">
            <li><a href="/">"Home"</a></li>
            <li>
                <a href="/tmf-api/productCatalogManagement/v4">"TMF620"</a>
            </li>
            <li>
                <a href="/tmf-api/tmf629/v4/customer">"TMF629"</a>
            </li>
            <li>
                <a href="/tmf-api/tmf632/v4">"TMF632"</a>
            </li>
            <li>
                <a href="/tmf-api/tmf648/v4/quote">"TMF648"</a>
            </li>
        </ul>
    }
}