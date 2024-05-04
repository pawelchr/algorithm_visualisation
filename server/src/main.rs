#![warn(clippy::all, clippy::pedantic)]
#[macro_use] extern crate rocket;

mod routes;
mod sorting;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().attach(routes::stage())
}
