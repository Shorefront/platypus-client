//! TimePeriod module

use tmflib::HasValidity;

use leptos::*;

#[component]
pub fn time_period<T : HasValidity>(item : T) -> impl IntoView {
    view!{
        <label for="start">Start</label>
        <input id="start" value=item.get_validity_start()/>
        <label>"End"</label>
        <input id="end" value=item.get_validity_end()/>    
    }
}