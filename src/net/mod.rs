use std::net::IpAddr;
use ntex::web;
use tokio::sync::RwLock;
use futures::future::try_join_all;
use crate::prelude::{CliArgs, Config};
use serde::{Deserialize, Serialize};

mod hostapi;
mod serverapi;

#[derive(Deserialize,Serialize)]
struct RecordRequest {
    name: String,
    ips: Vec<IpAddr>,
}

#[ntex::main]
pub async fn ntex_server
(

) -> std::io::Result<()> 
{

    let atlas_proxy_server = web::HttpServer::new
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
  
            .service(hostapi::scope_hostapi("api"))        
            // .default_service(web::route().to(testinglibs::flutter_web::dyn_service))


    })
    .bind(("0.0.0.0", 1025))?
    .run();
    let mut tasks = vec![atlas_proxy_server];

    let config = Config::get().read().await;
    for forward in config.static_port_forwards.iter() {
        let target_addr = format!("{}:{}", forward.input.0, forward.input.1)
            .parse::<std::net::SocketAddr>()
            .unwrap();
        let listen_addr = format!("0.0.0.0:{}", forward.output)
            .parse::<std::net::SocketAddr>()
            .unwrap();
        
        let target = target_addr;
        tokio::spawn(async move {
            if let Ok(listener) = tokio::net::TcpListener::bind(listen_addr).await {
                loop {
                    if let Ok((mut client, _)) = listener.accept().await {
                        let target = target;
                        tokio::spawn(async move {
                            if let Ok(mut remote) = tokio::net::TcpStream::connect(target).await {
                                let (mut ri, mut wi) = client.split();
                                let (mut rr, mut wr) = remote.split();
                                
                                let _ = tokio::try_join!(
                                    tokio::io::copy(&mut ri, &mut wr),
                                    tokio::io::copy(&mut rr, &mut wi)
                                );
                            }
                        });
                    }
                }
            }
        });
    }
    
    try_join_all(tasks).await?;
    Ok(())
    // .await
}
  
#[ntex::main]
pub async fn ntex_server_vps
(

) -> std::io::Result<()> 
{
    let arg = CliArgs::get();
    let port = arg.vps_port.unwrap_or(1025);
    web::HttpServer::new
    (async || 
    {
        web::App::new()
            .service(serverapi::scope_serverapi("api"))      
    })
    .bind(("0.0.0.0", port))?
    .run().await;
    Ok(())
    // 
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
