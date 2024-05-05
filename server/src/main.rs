#![warn(clippy::all, clippy::pedantic)]
#[macro_use] extern crate rocket;

mod routes;
mod sorting;
mod cors;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().attach(cors::CORS).attach(routes::stage())
}
