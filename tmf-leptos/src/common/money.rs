//! Rendor Money type

use leptos::prelude::*;

use tmflib::common::money::Money;

#[component]
pub fn money(label : String,read : ReadSignal<f32>,write: WriteSignal<f32>) -> impl IntoView {
    view!{
        <label for="money">{ label }</label>
        <input 
            type="text" 
            id="money" 
            value={read.get()}
            on:input=move |ev| {
                let value = event_target_value(&ev);
                if let Ok(parsed) = value.parse::<f32>() {
                    write.set(parsed);
                }
            }
        />
    }
}