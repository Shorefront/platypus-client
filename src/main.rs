use leptos::*;
use leptos_router::*;

use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;

#[component]
fn CustomerRow(customer : Customer) -> impl IntoView {
    view! {
        <tr><td><a href={customer.href}>{ customer.id}</a></td><td>{ customer.name }</td></tr>
    }
}

#[component]
fn IndividualTable() -> impl IntoView {
    view! {
        <div>
        <h2>Individual Table</h2>
        <table>
            <tr>
                <th>Name</th>
            </tr>
        </table>
        </div>
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
        <div>
        <h2>Customer Table</h2>
            <table>
                <tr>
                    <th>Id</th><th>Customer</th>
                </tr>
         
                {customers.into_iter()
                    .map(|c| {
                        CustomerRow(CustomerRowProps{ customer: c})
                    }).collect_view()
                }
            </table>
        </div>
    }
}

#[component]
fn Menu() -> impl IntoView {
    view!{
        <ul>
            <li><a href="/">Home</a></li>
            <li><a href="/tmflib/tmf629/customer">"Party / Customer"</a></li>
            <li><a href="/tmflib/tmf632/individual">"Party / Individual"</a></li>
        </ul>
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
