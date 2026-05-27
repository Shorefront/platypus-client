//! Forms for HasValidity trait
//! 

use leptos::prelude::*;
use tmflib::TimePeriod;
use super::time_period::TimePeriod;

#[component]
pub fn HasValidity<'a>(
        period: &'a mut TimePeriod,
        dirty : WriteSignal<bool>
    ) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"Validity"</legend>
           <TimePeriod period=period dirty=dirty />
        </fieldset>
    }
}