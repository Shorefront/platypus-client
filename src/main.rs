use leptos::*;
use leptos_router::*;

use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;

mod model;

use model::tmf620::CatalogRoutes;
use model::common::{Menu,GenericTable};

#[warn(missing_docs)]

#[component]
fn CustomerRow(customer : Customer) -> impl IntoView {
    view! {
        <tr><td><a href={customer.href}>{ customer.id}</a></td><td>{ customer.name }</td></tr>
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
fn NotFound() -> impl IntoView {
    view!{
        <div>
            <p>"Cannot find that page"</p>
        </div>
    }
}

#[component]
fn Platypus() -> impl IntoView {

    view!{
        <Router>
            <nav>
                <Menu />
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <CatalogRoutes />
                    <Route path="*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(Platypus)
}
