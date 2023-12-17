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
        <h2>{T::get_class().to_uppercase()} List</h2>
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
        <ul>
            <li><a href="/">"Home"</a></li>
            <li>
                "TMF620 Product Catalog"
                <ul>
                    <li><a href="/tmflib/productCatalogManagement/v4/category">"Categories"</a></li>
                    <li><a href="/tmflib/productCatalogManagement/v4/catalog">"Catalog"</a></li>
                    <li><a href="/tmflib/productCatalogManagement/v4/productOffering">"Product Offers"</a></li>
                    <li><a href="/tmflib/productCatalogManagement/v4/productSpecification">"Product Specification"</a></li>
                </ul>
            </li>
            <li>
            "TMF629"
                <ul>
                    <li><a href="/tmf-api/tmf629/v4/customer">"Customer"</a></li>
                </ul>
            </li>
            <li>
                "TMF632 Party"
                <ul>
                    <li><a href="/tmf-api/tmf632/v4/individual">"Individual"</a></li>    
                    <li><a href="/tmf-api/tmf632/v4/organization">"Organization"</a></li>
                </ul>
            </li>
        </ul>
    }
}