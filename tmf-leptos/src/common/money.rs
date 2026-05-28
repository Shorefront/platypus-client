//! Rendor Money type

use leptos::prelude::*;
use tmflib::common::money::Money;

#[component]
pub fn money(label : String,read : ReadSignal<Money>,write: WriteSignal<Money>) -> impl IntoView {
    let m = Money::default();
    let (value_read,value_write) = signal(m);
    view!{
        <label for="money">{ label }</label>
        <input 
            type="text" 
            id="money" 
            value={value_read}
            on:input=move |ev| {
                let value = event_target_value(&ev);
                if let Ok(parsed) = value.parse::<f32>() {
                    value_write.set(parsed);
                }
            }
        />
    }
}