use ntex::web;
use ntex::web::types::Json;
use tokio::sync::RwLock;
use crate::prelude::Config;
use crate::prelude::ProjectPath;
use super::{RecordRequest, dynapi};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::sync::OnceLock;
use toml;

static DYN_RECORD: OnceLock<RwLock<HashMap<String, IpAddr>>> = OnceLock::new();

fn get_dyn_record() -> &'static RwLock<HashMap<String, IpAddr>> {
    DYN_RECORD.get_or_init(|| RwLock::new(HashMap::new()))
}


pub fn scope_hostapi(path:&str) -> web::Scope<web::DefaultError> 
{
    let x: web::Scope<web::DefaultError> = web::scope(path)
        .service(hello)
        .service(port_forwards)
        .service(local_ip_address)
        .service(regist_to_vps)
        .service(set_vps)
        .service(set_dynip)
        .service(get_dynip)
        .service(list_dynip)
        .service(cfg);
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
    Json(config.static_port_forwards.clone())
}

#[web::get("/localip")]
async fn local_ip_address() -> impl web::Responder {
    let ipaddr = super::get_local_ip();
    Json(ipaddr)
}

#[web::get("/regist")]
async fn regist_to_vps() -> impl web::Responder {
    let config = Config::get().read().await;
    let ipaddr = super::get_local_ip();
    
    if let Some((vps_addr, vps_port)) = config.registration {
        let record = RecordRequest {
            name: config.name.clone(),
            ips: ipaddr,
        };
        
        let vps_url = format!("http://{}:{}/api/record", vps_addr, vps_port);
        
        match ntex::client::Client::new()
            .await
            .post(&vps_url)
            .send_json(&record)
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    web::HttpResponse::Ok().finish()
                } else {
                    web::HttpResponse::InternalServerError().finish()
                }
            }
            Err(_) => web::HttpResponse::InternalServerError().finish(),
        }
    } else {
        web::HttpResponse::BadRequest().body("No VPS registration configured")
    }
}

#[derive(Deserialize)]
struct SetVpsRequest {
    vps_addr: String,
    vps_port: u16,
}

#[web::post("/set_vps")]
async fn set_vps(Json(payload): Json<SetVpsRequest>) -> impl web::Responder {
    let config = Config::get().write().await;
    let vps_addr = payload.vps_addr.parse().unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    let mut config = config;
    config.registration = Some((vps_addr, payload.vps_port));
    
    let _ = Config::update().await;
    // if let Ok(toml_str) = toml::to_string_pretty(&*config) {
    //     let _ = fs::write(
    //         ProjectPath::get().proj_dir.join("config.toml"),
    //         toml_str
    //     );
    // }
    // 
    web::HttpResponse::Ok().finish()
}

#[web::get("/cfg")]
async fn cfg() -> impl web::Responder {
    let config = Config::get().read().await;
    Json(config.clone())
}




#[derive(Deserialize)]
pub struct SetDynIpRequest {
    name: String,
    ip: IpAddr,
}

#[web::post("/set_dynip")]
async fn set_dynip(Json(payload): Json<SetDynIpRequest>) -> impl web::Responder {
    let mut record = get_dyn_record().write().await;
    record.insert(payload.name, payload.ip);
    web::HttpResponse::Ok().finish()
}

#[web::get("/get_dynip/{name}")]
async fn get_dynip(name: String) -> impl web::Responder {
    let record = get_dyn_record().read().await;
    match record.get(&name) {
        Some(ip) => web::HttpResponse::Ok().json(&ip),
        None => web::HttpResponse::NotFound().finish(),
    }
}

#[web::get("/list_dynip")]
async fn list_dynip() -> impl web::Responder {
    let record = get_dyn_record().read().await;
    Json(record.clone())
}
