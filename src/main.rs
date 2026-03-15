mod prelude;
mod net;

use crate::{net::{get_local_ip, print_ips}, prelude::{Config, DatabaseManager, ProjectPath, TracingManager}};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    ProjectPath::get();
    TracingManager::init();
    let args = prelude::get_cli_args();
    let _config = Config::get().clone();

    let x = get_local_ip();
    print_ips(&x);
    
    if args.vps_port.is_some() {
        start_vps_mode().await?;
    } else {
        start_host_mode().await?;
    }


    Ok(())
}

async fn start_vps_mode() -> Result<()> {
    let _database = DatabaseManager::init();
    println!("VPS mode with simplified types not fully implemented yet");
    Ok(())
}

async fn start_host_mode() -> Result<()> {
    println!("Host mode with simplified types not fully implemented yet");
    Ok(())
}
