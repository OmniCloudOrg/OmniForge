//-----------------------------------------------------------------------------
// OmniForge - A Rust-based Free and open-source application deployment and
// lifecycle management platform built for the modern era or cloud native apps.
// 
// Authors: Tristan J. Poland, Chance Green, SafeShows
//-----------------------------------------------------------------------------

use rocket::launch;
use rocket::routes;

pub mod api;
mod autoscalar;
mod image_builder;
pub mod interfaces;

#[launch]
pub async fn start_server() -> _ {
    let port = 3030;
    rocket::build()
        .configure(rocket::Config {
            port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .mount("/", routes![api::build,api::deploy_permissions])
}