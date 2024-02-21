use std::future::Future;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

extern crate pi_ser;
const ADDR:SocketAddr = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 9231);
#[tokio::main]
async fn main(){
    loop{
        //oneshot channel:
        let (shut_channel_sender, shut_channel_receiver) = oneshot::channel::<()>();
        let server = pi_ser::http_server::server::HttpWebServer::new(ADDR);
        server.http_server_setup(shut_channel_receiver).await.expect("nüscht");
    }
}

