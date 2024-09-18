#[allow(unused)]
#[macro_use] extern crate rocket;

mod routes;
mod container;
mod schema;
mod models;
mod repositories;

use std::str::FromStr;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use routes::*;

#[launch]
 fn rocket() -> _ {
    let routes = routes![
        index_page,
        logout_page,
        logout_component,
        login_page,
        login_component,
        login_processor,
        dashboard_page,
        dashboard_component
    ];

    rocket::build()
        .mount("/", routes)
        .mount("/public", FileServer::from("./public"))
        .attach(Template::fairing ())
}
