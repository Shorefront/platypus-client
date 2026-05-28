//! Form for rendering a price, which is a money value with an optional unit of measure.

use leptos::prelude::*;
use super::money::Money;

use tmflib::common::price::Price;

#[component]
pub fn Price(label : String,read : ReadSignal<Price>,write: WriteSignal<Price>) -> impl IntoView {
    let duty = read.get().duty_free_amount;
    let tax = read.get().tax_included_amount;
    view!{
        <Money label=label read=read write=write />
    }
}   