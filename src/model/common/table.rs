//! Common Table
//! 
//! 
use leptos::*;
use tmflib::{HasId,HasName};

#[component]
pub fn TableRow<T : HasId + HasName>(item : T) -> impl IntoView {
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
                    <th>{ T::get_class()}s</th>
                </tr>
         
                {
                    items.into_iter()
                    .map(|c| {
                        TableRow(TableRowProps{ item: c})
                    }).collect_view()
                },  
            </table>
        </div>
    }
}

