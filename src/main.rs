use leptos::*;
use leptos_struct_table::*;

use tmflib::tmf629::customer::Customer;

#[component]
fn App() -> impl IntoView {

    let c1 = Customer::new(String::From("A Customer"));

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
