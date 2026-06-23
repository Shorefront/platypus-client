use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use actix_web::middleware::Logger;
use actix_web::middleware::Compress;
use actix_web::{web, App, HttpResponse, HttpServer,Responder};

mod model;

// Routes
#[cfg(feature = "tmf620")]
use model::tmf620::CatalogRoutes;
#[cfg(feature = "tmf629")]
use model::tmf629::CustomerRoutes;
#[cfg(feature = "tmf632")]
use model::tmf632::PartyRoutes;
#[cfg(feature = "tmf633")]
use model::tmf633::ServiceCatalogRoutes;
#[cfg(feature = "tmf645")]
use model::tmf645::ServiceQualificationRoutes;
#[cfg(feature = "tmf648")]
use model::tmf648::QuoteRoutes;
#[cfg(feature = "tmf674")]
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
    view! {
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
    view! {
            <p class="error">"Cannot find that page"</p>
    }
}

#[component]
fn Platypus() -> impl IntoView {
    // provide_context(ExampleContext(0));

    view! {
        <Router>
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
                    <ServiceQualificationRoutes />
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/*any") view=NotFound />
                    <Route path=path!("") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

async fn leptos_main() -> impl Responder {
    _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(Platypus);
    HttpResponse::Ok().body("Hello from the server!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(web::resource("/api/{tail:.*}").route(web::post().to(leptos_main)))
    })
    .run()
    .await
}
