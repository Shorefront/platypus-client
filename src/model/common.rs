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
        <div>
        <h2>{T::get_class().to_uppercase()} Table</h2>
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
pub fn Menu() -> impl IntoView {
    view!{
        <ul>
            <li><a href="/">Home</a></li>
            <li>
                TMF620 Product Catalog
                <ul>
                    <li><a href="/tmflib/tmf620/category">Categories</a></li>
                    <li><a href="/tmflib/tmf620/catalog">Catalog</a></li>
                </ul>
            </li>
            <li>
            TMF629
                <ul>
                    <li><a href="/tmflib/tmf629/customer">"Customer"</a></li>
                </ul>
            </li>
            <li>
                TMF632 Party
                <ul>
                    <li><a href="/tmflib/tmf632/individual">"Individual"</a></li>    
                    <li><a href="/tmflib/tmf632/organization">"Organization"</a></li>
                </ul>
            </li>
        </ul>
    }
}