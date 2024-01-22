//! Customer components


use leptos::*;
use leptos_router::*;

use crate::model::common::table::GenericTable;

use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an option"</p>
    }
}

#[component(transparent)]
pub fn CustomerRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/tmf629/v4" view=CustomerHome>
            <Route path="customer" view=CustomerList >
                <Route path=":id" view=CustomerView />
                <Route path="" view=NoOptionView />
            </Route>
            <Route path="" view=NoOptionView />
        </Route>
    }
}

#[component]
pub fn CustomerHome() -> impl IntoView {
    view! {
        <nav>
            <ul class="menu">
                <li><a href="customer">"Customer"</a></li>
            </ul>
        </nav>

        <Outlet />
    }
}

#[component]
pub fn CustomerView() -> impl IntoView {
    view! {
        <h2>"Customer Details"</h2>
        <table>
        </table>
    }
}

#[component]
pub fn CustomerList() -> impl IntoView {
    let org1 = Organization::new("Customer One".to_string());
    let cust1 = Customer::new(org1);
    let org2 = Organization::new("Customer Two".to_string());
    let cust2 = Customer::new(org2);
    let custs = vec![cust1,cust2];
    view! {
        <div class="list">
            <GenericTable items=custs/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}