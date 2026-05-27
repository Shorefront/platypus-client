//! TimePeriod module

use tmflib::{HasValidity,TimePeriod};

use leptos::prelude::*;

use leptos::web_sys::*;

#[component]
pub fn TimePeriod<'a>(period : &'a mut TimePeriod) -> impl IntoView {
    let start_enabled : bool = false;
    let finish_enabled : bool = false;
    let preset : i32 = 0;
    let (start_read, start_write) = signal(period.start_date_time.clone());
    let (end_read, finish_write) = signal(period.end_date_time.clone());
    let (start_enabled_read, enabled_write) = signal(start_enabled);
    let (finish_enabled_read, finish_enabled_write) = signal(finish_enabled);
    let (preset_read, preset_write) = signal(preset);
    preset_read.with(|p| {
        let new_period = match p {
            // -2 => new_period = TimePeriod::period_never(),
            // 0 => new_period = TimePeriod::period_ongoing(),
            7 => TimePeriod::period_days(7),
            30 => TimePeriod::period_days(30),
            90 => TimePeriod::period_days(90),
            365 => TimePeriod::period_days(365),
            // -1 => new_period = TimePeriod::period_all_time(),
            _ => TimePeriod::default(),
        };
        println!("Preset changed to: {}. New period: {:?}", p, new_period);
        start_write.set(new_period.start_date_time.clone());
        finish_write.set(new_period.end_date_time.clone());
        // enabled_write.set(new_period.start_date_time.is_some());
        // finish_enabled_write.set(new_period.end_date_time.is_some());
    }); 
    view!{
        <fieldset>
            <legend>Time Period</legend>
            <label for="start">"Start"</label>
            <input 
                type="text" 
                id="start" 
                name="start" 
                value={start_read}
            />
            <input type="checkbox" checked={start_enabled_read}/>
            <br />
            <label for="finish">"Finish"</label>
            <input 
                type="text" 
                id="finish" 
                name="finish" 
                value={end_read}
            />
            <input type="checkbox" checked={finish_enabled_read}/>
            <br />
            <label for="presets">"Presets"</label>
            <select
                on:change=move |ev| {
                    println!("Preset selection changed: {:?}", event_target_value(&ev));
                    preset_write.set(event_target_value(&ev).parse::<i32>().unwrap_or(0));
                } 
                id="presets" name="presets">
                <option value="-2">"Never"</option>
                <option value="0">"Ongoing"</option>
                <option value="7">"7 Days"</option>
                <option value="30">"30 Days"</option>
                <option value="90">"90 Days"</option>
                <option value="365">"1 Year"</option>
                <option value="-1">"For all time"</option>
            </select>
        </fieldset>
    }
}