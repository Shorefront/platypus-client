//! Forms for HasValidity trait
//! 

use leptos::{html::Time, prelude::*};
use tmflib::{HasId,HasValidity,TimePeriod};
use super::time_period::TimePeriod;

#[component]
pub fn HasValidity<'a>(period: &'a mut TimePeriod) -> impl IntoView {

    // let mut period = TimePeriod::period_days(0);

    view! {
        <fieldset>
            <legend>"Validity"</legend>
           <TimePeriod period=period />
        </fieldset>
    }
}