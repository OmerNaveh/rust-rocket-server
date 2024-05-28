#[macro_use] extern crate rocket;


mod models;
mod routes;
mod controllers;
mod middlewares;

use routes::{router1,router2,router3,safe_route};


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/router1", routes![router1::get1, router1::get2, router1::post, router1::get_by_id, router1::create, router1::get_by_query_id])
    .mount("/router2", routes![router2::get1, router2::get2, router2::post])
    .mount("/router3", routes![router3::get1, router3::get2, router3::post])
    .mount("/safe-route", routes![safe_route::get_secret])
}
