use std::net::IpAddr;
use ntex::web;
use tokio::sync::RwLock;
use futures::future::try_join_all;
use crate::prelude::{CliArgs, Config};
use crate::net::hostapi::get_dyn_record;
use serde::{Deserialize, Serialize};
use tokio::time::{interval, Duration};
use reqwest::Client;

pub mod hostapi;
pub mod serverapi;

#[derive(Deserialize,Serialize)]
pub struct RecordRequest {
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
    let registration_config = config.registration.clone();
    drop(config);
    
    if let Some((vps_addr, vps_port)) = registration_config {
        tokio::spawn(async move {
            let config = Config::get();
            let client = Client::new();
            let mut interval = interval(Duration::from_secs(600));
            loop {
                let cfg = config.read().await;
                let name = cfg.name.clone();
                let ips = get_local_ip();
                drop(cfg);
                
                let record = RecordRequest {
                    name,
                    ips,
                };
                
                let vps_url = format!("http://{}:{}/api/record", vps_addr, vps_port);
                
                if let Err(e) = client
                    .post(&vps_url)
                    .json(&record)
                    .send()
                    .await
                {
                    tracing::error!("Error registering to VPS: {}", e);
                }
                
                interval.tick().await;
            }
        });
    }
    
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
    
    for forward in config.dyn_port_forwards.iter() {
        let dyn_record = get_dyn_record();
        let target_name = forward.input.0.clone();
        let remote_port = forward.input.1;
        let listen_addr = format!("0.0.0.0:{}", forward.output)
            .parse::<std::net::SocketAddr>()
            .unwrap();
        
        tokio::spawn(async move {
            if let Ok(listener) = tokio::net::TcpListener::bind(listen_addr).await {
                loop {
                    if let Ok((mut client, _)) = listener.accept().await {
                        let dyn_record = dyn_record.clone();
                        let target_name = target_name.clone();
                        
                        tokio::spawn(async move {
                            let record = dyn_record.read().await;
                            let target_ip = record.get(&target_name).cloned();
                            drop(record);
                            
                            if let Some(ip) = target_ip {
                                let target_addr = format!("{}:{}", ip, remote_port)
                                    .parse::<std::net::SocketAddr>()
                                    .unwrap();
                                
                                if let Ok(mut remote) = tokio::net::TcpStream::connect(target_addr).await {
                                    let (mut ri, mut wi) = client.split();
                                    let (mut rr, mut wr) = remote.split();
                
                                    let _ = tokio::try_join!(
                                        tokio::io::copy(&mut ri, &mut wr),
                                        tokio::io::copy(&mut rr, &mut wi)
                                    );
                                }
                            } else {
                                tracing::error!("Device '{}' not found in DYN_RECORD", target_name);
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
    .run().await?;
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
