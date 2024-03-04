mod input_control;
mod http_server;
mod broker;

use std::io::{stdout, BufWriter};

//use warp::Filter;
use ferris_says::say;
use tokio::{
    task::JoinHandle,
    sync::oneshot,
    time::{Duration,sleep}
};

use local_ip_address::list_afinet_netifas;



async fn flatten<T>(handle: JoinHandle<Result<T, &'static str>>) -> Result<T, &'static str> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(_err) => Err("handling failed"),
    }
}

#[tokio::main]
async fn main() {
    //Greetings
    {
        let stdout = stdout();
        let writer = BufWriter::new(stdout.lock());

        say("Hello, Munke",12,writer).unwrap();

        let network_interfaces = list_afinet_netifas().unwrap();
        for (name, ip) in network_interfaces.iter() {
            if name == "wlp0s20f3"{
                if ip.is_ipv4(){
                    println!("HTTP Server hosted at https://{:?}:9231/", ip);
                }
            }
        }
        println!("HTTP Server hosted at https://localhost:9231/");
    }

    loop {
        //Signal-Kanal für shutdown
        let (shut_channel_sender, shut_channel_receiver) = oneshot::channel::<()>();

        let http_server = tokio::spawn(http_server::http_server_setup(shut_channel_receiver));
        //let http_server = tokio::spawn(pace_holder("http_server"));
        let input = tokio::spawn(input_control::system_input(shut_channel_sender));
        //let input = tokio::spawn(pace_holder("input"));
        let broker = tokio::spawn(broker::broker_setup());
        //let broker = tokio::spawn(pace_holder("broker"));


        let processing_res = tokio::try_join!(
            flatten(http_server),
            flatten(input),
            flatten(broker)
        );

        match processing_res {
            Ok((server_res, input_res,broker_res)) => {
                println!("Rückgabe = (Server: {:?}); (Input: {:?}); (Broker: {:?})", server_res, input_res, broker_res);
                continue;
            }
            Err(e) => {
                println!("Fehler in Tokio! err: {e}");
                return;
            }
        }

    }
}


async fn pace_holder(_name:&str)->Result<(), &'static str>{
    loop {
        println!("[main    ] Modul {_name} nicht aktiv!");
        let _ = sleep(Duration::from_millis(10000)).await;
    }
}
