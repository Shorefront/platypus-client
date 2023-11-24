use leptos::*;


#[component]
pub fn CatalogTable() -> impl IntoView {
    view! {
        <p>"A Catalog Table"</p>
        <table>
        </table>
    }
}

#[component]
pub fn CatalogView(_id : String) -> impl IntoView {
    view! {
        <p>
        "Some catalogue view"
        </p>
    }
}