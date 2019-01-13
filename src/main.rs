#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod routing;
mod helpers;
mod models;
mod entities;
mod schema;

use diesel::SqliteConnection;

#[database("fruit_store")]
pub struct DbConnection(SqliteConnection);

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
        .attach(DbConnection::fairing())
        .mount("/", routes)
        .launch();
}
