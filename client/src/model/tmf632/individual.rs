//! Individual Model

use leptos::prelude::*;
use leptos_router::components::*;

#[cfg(feature = "V4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "V5")]
use tmflib::tmf632::individual_v5::Individual;

use tmflib::HasName;

// use crate::GenericTable;
use crate::model::common::list::GenericListWithAdd;

#[component]
pub fn IndividualTable() -> impl IntoView {
    let ind1 = Individual::new("Ryan");
    let ind2 = Individual::new("John");
    let ind3 = Individual::new("Fred");
    let individuals = vec![ind1, ind2, ind3];
    view! {
        <div class="list">
            <GenericListWithAdd items=individuals/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn IndividualView() -> impl IntoView {
    let name = "Placeholder".to_string();
   view! {
        <table>
            <tr>
                <td>"Name:"</td>
                <td>{name}</td>
            </tr>
        </table>
    }
}

#[component]
pub fn IndividualAdd() -> impl IntoView {
    let new_item = Individual::new("New Individual");
    view! {
        <p>"Add new Individual"</p>
        <form>
            <label for="name">"Name:"</label>
            <input type="text" id="name" name="name" value={new_item.get_name()}/><br />
            <button type="submit">"Submit"</button>
        </form>
        <Outlet />
    }
}    
