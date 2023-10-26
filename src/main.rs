use leptos::*;
use leptos_router::*;

use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;
use tmflib::tmf632::individual::Individual;

mod model;

use model::tmf620::catalog::{CatalogTable,CatalogView};
use model::common::{Menu,GenericTable};

#[component]
fn CustomerRow(customer : Customer) -> impl IntoView {
    view! {
        <tr><td><a href={customer.href}>{ customer.id}</a></td><td>{ customer.name }</td></tr>
    }
}

#[component]
fn CategoryTable() -> impl IntoView {
    let cat1 = Category::new("Root".to_string());
    let categories = vec![cat1];
    view! {
        <GenericTable items=categories/>
    }
    
}

#[component]
fn IndividualTable() -> impl IntoView {
    let ind1 = Individual::new("Ryan");
    let ind2 = Individual::new("John");
    let ind3 = Individual::new("Fred");
    let individuals = vec![ind1,ind2,ind3];
    view! {
            <GenericTable items=individuals/>
    }
}

#[component]
fn CustomerTable() -> impl IntoView {
    let org = Organization::new("Another Org".to_owned());
    let c1 = Customer::new(org);
    let org2 = Organization::new("Mangus Org".to_owned());
    let c2 = Customer::new(org2);
    let customers = vec![c1,c2];
    view! {
        <GenericTable items=customers/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view!{
        <div>
        <h2>TMF Data Management</h2>
        <p>This is an experiemental TMF data management platform written in Rust.</p>
        </div>
    }
}

#[component]
fn App1() -> impl IntoView {

    view!{
        <Router>
            <h1>"Platypus - TMF Management"</h1>
            <nav>
                {Menu()}
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/tmflib/tmf620/catalog" view=CatalogTable/>
                    <Route path="/tmflib/tmf620/category" view=CategoryTable/>
                    <Route path="/tmflib/tmf629/customer" view=CustomerTable/>
                    <Route path="/tmflib/tmf632/individual" view=IndividualTable/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(App1)
}
