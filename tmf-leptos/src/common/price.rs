//! Form for rendering a price, which is a money value with an optional unit of measure.

use leptos::prelude::*;
use tmflib::common::money::Money;

#[component]
pub fn Price(label : String,read : ReadSignal<Money>,write: WriteSignal<Money>) -> impl IntoView {
    view!{
        <label for="price">{ label }</label>
        <input 
            type="text" 
            id="price" 
            value={read.get().amount.to_string()}
            on:input=move |ev| {
                let value = event_target_value(&ev);
                if let Ok(parsed) = value.parse::<f32>() {
                    let mut money = read.get();
                    money.amount = parsed;
                    write.set(money);
                }
            }
        />
        <input 
            type="text" 
            id="price_uom" 
            value={read.get().unit_of_measure.clone().unwrap_or_default()}
            on:input=move |ev| {
                let value = event_target_value(&ev);
                let mut money = read.get();
                money.unit_of_measure = if value.is_empty() { None } else { Some(value) };
                write.set(money);
            }
        />
    }
}   