## Introduction

This project is a an example of [Rocket](https://rocket.rs/) web framework for a Web API.

The database engine used by the project is [SQLite](https://www.sqlite.org/).

### Getting Started

First, install [Diesel](http://diesel.rs/) with `cargo install diesel_cli --no-default-features --features sqlite` 
for database management.

To create and migrate the database:

1. Initialize database with `diesel setup`
2. Migrate database with `diesel migration run` and `diesel migration redo`

To start the Rocket server:

1. Install dependencies and build executables with `cargo build`
2. Start Rocket server with `target/debug/rocket_webapi`

Now you can visit http://localhost:3000/api from your browser.
