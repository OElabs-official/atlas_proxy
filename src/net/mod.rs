use std::net::{IpAddr , Ipv4Addr, Ipv6Addr};
use ntex::web;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

mod api;


#[ntex::main]
async fn ntex_server
(

) -> std::io::Result<()> 
{
    web::HttpServer::new
    (async || 
    {
        web::App::new()
            .state(RwLock::new(0)) // task i0  > struct default = 0; use inner rwlock
            /*
                .middleware
                (
                    Cors::new()
                    // .allowed_origin("*") 
                    .allowed_methods(vec!["GET", "POST","OPTIONS"])
                    .allowed_headers(vec!
                        [
                            http::header::AUTHORIZATION, 
                            http::header::ACCEPT,
                            http::header::CONTENT_TYPE,
                            http::header::AUTHORIZATION,
                            
                        ])
                    // .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .send_wildcard()
                    .finish()
                )  
                .service
                (
                    web::resource("/proxy")
                    .route(web::route().to(hello))
                )

             */


            .service(hello)            
            // .default_service(web::route().to(testinglibs::flutter_web::dyn_service))


    })
    .bind(("0.0.0.0", 13600))?
    .run()
    .await
}

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}


/// 获取本地 IP 地址
pub fn get_local_ip() -> Vec<IpAddr> {
    let mut output = vec![];
    if let Ok(nets) = local_ip_address::list_afinet_netifas() {
        for (_name, ip) in nets {
            if !ip.is_loopback() {
                output.push(ip);
            }
        }
    }
    output
}

/// 打印 IP 地址列表
pub fn print_ips(input: &[IpAddr]) {
    for ip in input {
        println!("{}", ip);
    }
}
