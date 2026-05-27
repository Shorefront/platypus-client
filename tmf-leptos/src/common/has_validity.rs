//! Forms for HasValidity trait
//! 

use leptos::prelude::*;
use tmflib::{HasId,HasValidity};

#[component]
pub fn Validity<T: HasId + HasValidity>(item: T) -> impl IntoView {
    let valid_start = item.get_validity_start();
    let valid_end = item.get_validity_end();
    view! {
        <fieldset>
            <legend>Validity</legend>
            <tr>
                <td><label for="start_date_time">Start</label></td>
                <td><input id="start_date_time" type="datetime-local" value=valid_start /></td>
            </tr>
            <tr>
                <td><label for="end_date_time">End</label></td>
                <td><input id="end_date_time" type="datetime-local" value=valid_end/></td>
            </tr>
        </fieldset>
    }
}