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
        <div style="float: left; ">
        <h2>{T::get_class().to_uppercase()}</h2>
            <table style="border: 1px solid;width: 30%;">
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
pub fn Menu() -> impl IntoView {
    view!{
        <ul class="menu">
            <li><a href="/">"Home"</a></li>
            <li>
                <a href="/tmf-api/productCatalogManagement/v4">"TMF620"</a>
            </li>
            <li>
                "TMF629"
            </li>
            <li>
                "TMF632"
            </li>
        </ul>
    }
}