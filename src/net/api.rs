use ntex::web;
use ntex::web::types::Json;
use crate::prelude::Config;
use serde::Serialize;


pub fn scope_api(path:&str) -> web::Scope<web::DefaultError> 
{
    let x: web::Scope<web::DefaultError> = web::scope(path)
        .service(hello)
        .service(port_forwards)
        .service(local_ip_address);
    x
}

#[web::get("/")]
async fn hello() -> impl web::Responder {
    let config = Config::get().read().await;
    Json((config.app_name.clone(), config.version.clone(), config.name.clone()))
}

#[web::get("/port")]
async fn port_forwards() -> impl web::Responder {
    let config = Config::get().read().await;
    Json(config.port_forwards.clone())
}

#[web::get("/localip")]
async fn local_ip_address() -> impl web::Responder {
    let ipaddr = super::get_local_ip();
    Json(ipaddr)
}

