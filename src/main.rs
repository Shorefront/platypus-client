use leptos::*;

use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;

#[component]
fn App() -> impl IntoView {

    let org = Organization::new("My Org".to_owned());
    let c1 = Customer::new(org);

    let customers = create_rw_signal(vec![c1]);

    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>"Click this: "
        {count}
        </button>
    }
}

fn main() {
    mount_to_body(|| view! {<App/>})
}
