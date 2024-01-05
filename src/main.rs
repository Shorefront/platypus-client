use leptos::*;
use leptos_router::*;

mod model;

// Routes
use model::tmf620::CatalogRoutes;

// Common
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
                    <CatalogRoutes />
                    <Route path="/" view=Home/>
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(Platypus)
}
