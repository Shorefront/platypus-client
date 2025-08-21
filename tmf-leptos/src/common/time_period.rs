//! TimePeriod module

use tmflib::HasValidity;

use leptos::prelude::*;

#[component]
pub fn TimePeriod<T : HasValidity,'a>(item : &'a T) -> impl IntoView {
    let start = item.get_validity_start();
    let finish = item.get_validity_end();
    view!{
        <fieldset>
            <legend>Time Period</legend>
            <label for="name">"Start:"</label>
            <input type="text" id="start" name="start" value={start}/><br />
            <label for="name">"Finish:"</label>
            <input type="text" id="finish" name="finish" value={finish}/><br />
        </fieldset>
    }
}