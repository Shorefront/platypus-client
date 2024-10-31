//! Customer components


use leptos::*;
use leptos_router::*;
use tmflib::{HasId, HasName};

use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;
use crate::model::common::form::NamedClass;

use tmflib::tmf629::customer::Customer;
#[cfg(feature = "tmf632_v4")]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(feature = "tmf632_v5")]
use tmflib::tmf632::organization_v5::Organization;

#[component]
pub fn NoOptionView() -> impl IntoView {
    view! {
        <p>"Please select an customer"</p>
    }
}

#[component]
pub fn BasicCustomer(mut customer : Customer) -> impl IntoView {
    let (name,set_name) = create_signal("New Category".to_string());
    name.with(|n| customer.set_name(n));
    view! {
        <fieldset>
            <legend>Customer Information</legend>
            <NamedClass item=customer signal=set_name/>
        </fieldset>
    }
}

#[component(transparent)]
pub fn CustomerRoutes() -> impl IntoView {
    view! {
        <Route path="/tmf-api/customerManagement/v4" view=CustomerHome>
            <Route path="customer" view=CustomerList >
                <Route path=":id" view=CustomerView />
                <Route path="add" view=CustomerAdd />
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
            <GenericListWithAdd items=custs/>
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn CustomerAdd() -> impl IntoView {
    let customer = Customer::default();
    view! {
        <p>"Add Customer"</p>
        <BasicCustomer customer=customer.clone() />
    }
}