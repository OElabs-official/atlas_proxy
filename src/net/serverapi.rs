use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::OnceLock;

use ntex::web;
use ntex::web::types::Json;
use tokio::sync::RwLock;
use crate::prelude::Config;


use super::RecordRequest;
static RECORD: OnceLock<RwLock<HashMap<String,Vec<IpAddr>>>> = OnceLock::new();

fn get_record() -> &'static RwLock<HashMap<String,Vec<IpAddr>>> {
    RECORD.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn scope_serverapi(path:&str) -> web::Scope<web::DefaultError> 
{
    let x: web::Scope<web::DefaultError> = web::scope(path)
        .service(ip_list)
        .service(record_ip);
    x
}


#[web::get("/list")]
async fn ip_list() -> impl web::Responder {
    let record = get_record().read().await;
    Json(record.clone())
}

#[web::post("/record")]
async fn record_ip(Json(payload): Json<RecordRequest>) -> impl web::Responder {
    let mut record = get_record().write().await;
    record.insert(payload.name, payload.ips);
    web::HttpResponse::Ok().finish()
}
