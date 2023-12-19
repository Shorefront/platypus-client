use leptos::*;
use leptos_router::*;

mod model;

use model::tmf620::CatalogRoutes;
use model::tmf629::CustomerRoutes;
use model::tmf632::PartyRoutes;
use model::tmf648::QuoteRoutes;
use model::common::{Banner,Menu,GenericTable};

#[warn(missing_docs)]

#[component]
fn Home() -> impl IntoView {
    view!{
        <nav>
            <ul>
                ""
            </ul>
        </nav>
        <div>
        <h2>Platypus - TMF Data Management</h2>
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
                <Banner />
            </nav>
            <nav>
                <Menu />
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <CatalogRoutes />
                    <CustomerRoutes />
                    <PartyRoutes />
                    <QuoteRoutes />
                    <Route path="*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(Platypus)
}
