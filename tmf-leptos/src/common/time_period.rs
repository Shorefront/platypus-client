//! TimePeriod module

use tmflib::HasValidity;

use leptos::prelude::*;

#[component]
pub fn TimePeriod<T : HasValidity,'a>(item : &'a mut T) -> impl IntoView {
    let start = item.get_validity_start();
    let finish = item.get_validity_end();
    let start_enabled = start.is_some();
    let finish_enabled = finish.is_some();
    view!{
        <fieldset>
            <legend>Valid For</legend>
            <label for="start">"Start"</label>
            <input 
                type="text" 
                id="start" 
                name="start" 
                prop:value={start}
            />
            <input type="checkbox" checked={start_enabled}/>
            <br />
            <label for="finish">"Finish"</label>
            <input 
                type="text" 
                id="finish" 
                name="finish" 
                value={finish}
            />
            <input type="checkbox" checked={finish_enabled}/>
            <br />
            <label for="presets">"Presets"</label>
            <select id="presets" name="presets">
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