#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routing;

fn main() {
    let routes = routes![routing::api_routing::api];

    rocket::ignite()
        .mount("/", routes)
        .launch();
}
