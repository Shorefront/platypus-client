//! Form for rendering a price, which is a money value with an optional unit of measure.

use leptos::prelude::*;
use super::money::Money;
use super::SingleRow;

use tmflib::common::price::Price;

#[component]
pub fn Price(label : String,read : ReadSignal<Price>,write: WriteSignal<Price>) -> impl IntoView {
    let duty = read.get().duty_free_amount;
    let tax = read.get().tax_included_amount;
    let tax_rate = read.get().tax_rate.to_string();
    let (duty_read,duty_write) = signal(duty);
    let (tax_read,tax_write) = signal(tax);
    let (rate_read,rate_write) = signal(tax_rate);
    rate_read.with(|r| {
        // Take updated rate and push upstream
        let mut new_price = Price::default();
        let rate : f32 = r.parse().unwrap_or_default();
        new_price.tax_rate = rate;
        write.set(new_price);
    });
    duty_read.with(|d| {
        // Update Tax based on Duty * Tax
        
    });
    view!{
        <fieldset>
            <legend>{label}</legend>
            <SingleRow id="rate" label="Tax Rate" read=rate_read write=rate_write />
            <Money label="Duty".into() read=duty_read write=duty_write />
            <Money label="Tax".into() read=tax_read write=tax_write />
        </fieldset>
    }
} 