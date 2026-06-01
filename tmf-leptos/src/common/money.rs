//! Rendor Money type

use leptos::prelude::*;
use tmflib::common::money::Money;

#[component]
pub fn money(label : String,read : ReadSignal<Money>,write: WriteSignal<Money>) -> impl IntoView {
    let value : f32 = read.get().value.try_into().unwrap_or_default();
    let (value_read,value_write) = signal(value);
    value_read.with(|v| {
        // Value has been updated, push upstream
        let mut m = read.get().clone();
        let new_value : i32 = (*v * 100.0) as i32;
        m.value = new_value.into();
        write.set(m);
    });
    view!{
        <label for="money">{ label }</label>
        <input 
            type="text" 
            id="money"
            value={value_read}        
            on:input=move |ev| {
                let new_value = event_target_value(&ev);
                if let Ok(parsed) = new_value.parse::<f32>() {
                    value_write.set(parsed);
                }
            }
        />
    }
}