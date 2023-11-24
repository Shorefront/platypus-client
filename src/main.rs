use leptos::*;
use leptos_router::*;

use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;

mod model;

use model::tmf620::catalog::CatalogTable;
use model::tmf620::category::CategoryTable;
use model::tmf620::product_specification::ProductSpecificationTable;
use model::tmf620::product_offering::ProductOfferingTable;
use model::common::{Menu,GenericTable};
use model::tmf632::individual::IndividualTable;
use model::tmf632::organization::{OrganizationTable,OrganizationView};

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
                    <Route path="/tmflib/productCatalogManagement/catalog" view=CatalogTable/>
                    <Route path="/tmflib/productCatalogManagement/category" view=CategoryTable/>
                    <Route path="/tmflib/productCatalogManagement/productSpecification" view=ProductSpecificationTable/>
                    <Route path="/tmflib/productCatalogManagement/productOffering" view=ProductOfferingTable/>
                    <Route path="/tmflib/tmf629/customer" view=CustomerTable/>
                    <Route path="/tmflib/tmf632/individual" view=IndividualTable/>
                    <Route path="/tmflib/tmf632/organization" view=OrganizationTable>
                        <Route path=":id" view=OrganizationView/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(App1)
}
