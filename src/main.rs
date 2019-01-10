#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

mod routing;
mod models;
mod helpers;
mod entities;

fn main() {
    let routes = routes![
        routing::api_routing::api,
        routing::fruit_routing::all,
        routing::fruit_routing::one,
        routing::fruit_routing::new,
        routing::fruit_routing::edit,
        routing::fruit_routing::delete
    ];

    rocket::ignite()
        .mount("/", routes)
        .launch();
}
