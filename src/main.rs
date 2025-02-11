use leptos_router::components::{Router,Route, Routes};
use leptos_router::path;
use leptos::prelude::*;
use leptos::mount::mount_to_body;

mod model;

// Routes
use model::tmf620::CatalogRoutes;
use model::tmf629::CustomerRoutes;
use model::tmf632::PartyRoutes;
#[cfg(feature = "tmf633")]
use model::tmf633::ServiceCatalogRoutes;
use model::tmf648::QuoteRoutes;
use model::tmf674::GeographicSiteRoutes;
#[cfg(feature = "tmf7xx")]
use model::tmf7xx::CostModelRoutes;
// Common
use model::common::Banner;
// use model::common::table::GenericTable;
use model::common::menu::Menu;

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
            <p class="error">"Cannot find that page"</p>
    }
}

#[component]
fn Platypus() -> impl IntoView {
    // provide_context(ExampleContext(0));

    view!{
        <Router set_is_routing>
            <nav>
                <Banner />
            </nav>
            <nav>
                <Menu />
            </nav>
            <main>
                <Routes fallback=|| "This page could not be found">
                    <CatalogRoutes />
                    <CustomerRoutes />
                    <PartyRoutes />
                    <ServiceCatalogRoutes />
                    <QuoteRoutes />
                    <GeographicSiteRoutes />
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/*any") view=NotFound />
                    <Route path=path!("") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(Platypus)
}
