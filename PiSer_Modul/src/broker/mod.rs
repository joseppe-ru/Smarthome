use std::sync::{Arc};
use mqtt_packet_3_5::{ConnackPacket, MqttPacket};
use tokio::{sync::Mutex,net::TcpListener};
use tokio::net::TcpStream;
use crate::broker::client::Client;
use crate::broker::message_queue::MessageQueue;
use crate::broker::client::tcp_stream_handler::TCPHandler;
mod client;
mod message_queue;
mod worker;


pub async fn broker_setup()->Result<(),&'static str>{
    println!("Broker wird gestartet");

    //message queue starten (mehr so ne referenz auf die Messagequeue)
    let queue = Arc::new(Mutex::new(message_queue::MessageQueue::new()));
    //worker starten (Consumer der Queue)
    let queue_clone=Arc::clone(&queue);
    let worker = tokio::spawn(async move { worker::worker_process(queue_clone).await });

    println!("Listener wird gestartet");

    let listener = TcpListener::bind("0.0.0.0:1884").await.expect("failed to bind address");
    while let Ok((stream,_)) = listener.accept().await{
        println!("Neuer MQTT_Client connected: {:?}",stream.peer_addr());
        let queue_clone=Arc::clone(&queue);
        let client = tokio::spawn(async move {handle_connect(stream,queue_clone).await});
    }

    Ok(())
}

async fn handle_connect(tokio_stream:TcpStream,queue:Arc<Mutex<MessageQueue>>){

    //stream umwandeln
    let std_stream=tokio_stream //tokio TcpStream in std TcpStream umwandeln
        .into_std()
        .expect("[client] failed to convert tokio to std");

    //connectpacket raus lesen
    let ConnPack=TCPHandler::get_connect_packet(std_stream.try_clone().unwrap()).expect("[conn] Fehler beim finden des Connect Paketes");

    //neuer Tcp-Handler -> hat Funktionen: zum sperren des Cients; senden und empfagnen über Client.stream
    let mut tcp_handler = TCPHandler::new(ConnPack.client_id.clone());

    //neuer Client-Handler
    let client_handler = Arc::new(Mutex::new(Client::start_new_client(std_stream,queue,ConnPack)));

    //connect packet handlen
    let cli_clone=Arc::clone(&client_handler);
    TCPHandler::handle_connect(cli_clone).await;

    //reader prozess starten
    let _ = tokio::spawn(async move{ tcp_handler.message_handler(client_handler).await});
}
