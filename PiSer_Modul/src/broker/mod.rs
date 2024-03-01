use std::{sync::{Arc}};
use tokio::net::{TcpStream ,TcpListener};
use std::time::Duration;
use tokio::{sync::Mutex};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use crate::broker::client::Client;
use crate::broker::message_queue::MessageQueue;
use crate::broker::client::tcp_stream_writer::TcpWriter;
use crate::broker::client::tcp_stream_reader::TcpReader;
mod client;
mod message_queue;
mod worker;


pub async fn broker_setup()->Result<(),&'static str>{
    println!("[broker  ] Broker wird gestartet");

    //message queue starten (mehr so ne referenz auf die Messagequeue)
    let queue = Arc::new(Mutex::new(message_queue::MessageQueue::new()));
    //worker starten (Consumer der Queue)
    let queue_clone=Arc::clone(&queue);
    let worker = tokio::spawn(async move { worker::worker_process(queue_clone).await });

    println!("[broker] Listener wird gestartet");

    let listener = TcpListener::bind("0.0.0.0:1885").await.expect("failed to bind address");
    /*
    while let Ok((stream,_)) = listener.accept().await{
        println!("[broker  ] Neuer MQTT_Client connected: {:?}",stream.peer_addr());
        let queue_clone=Arc::clone(&queue);
        tokio::spawn(handle_connect(stream,queue_clone));
    }
     */
    loop{
        let (stream,_)=listener.accept().await.unwrap();
        println!("[broker  ] Neuer MQTT_Client connected: {:?}",stream.peer_addr());
        let queue_clone=Arc::clone(&queue);
        tokio::spawn(async move { handle_connect(stream, queue_clone).await});
    }

    //return tokio::spawn(async move {pace_holder("broker").await});
    Ok(())
}

async fn handle_connect(tokio_stream:TcpStream,queue:Arc<Mutex<MessageQueue>>)->Result<(),&'static str>{

        //tokio TcpStream in std TcpStream umwandeln
    let std_stream=tokio_stream.into_std().expect("[broker  ] failed to convert tokio to std");
    //let std_stream=tokio_stream;

    //connectpacket raus lesen
    let ConnPack= TcpReader::get_connect_packet(
        std_stream.try_clone().unwrap()
    ).expect("[broker  ] Fehler beim finden des Connect Paketes");

        //neuer Tcp-Handler -> hat Funktionen: zum sperren des Cients; senden und empfagnen über Client.stream
    let tcp_writer = TcpWriter::new(ConnPack.client_id.clone());

        //neuer Client-Handler
    let client_handler = Arc::new(Mutex::new(Client::start_new_client(std_stream,Arc::clone(&queue),ConnPack,tcp_writer)));

        //neuer Reader
    let mut tcp_reader = TcpReader::new(Arc::clone(&client_handler),Arc::clone(&queue));

        //alles andere lesen
    tcp_reader.handle_connect().await;

        //reader prozess starten
    //let _ = tokio::spawn(async move{ tcp_reader.message_handler().await});
    tcp_reader.message_handler().await;
    Ok(())
}

async fn pace_holder(_name:&str)->Result<(), &'static str>{
    println!("[main    ] Modul {_name} nicht aktiv!");
    sleep(Duration::from_millis(1000)).await;
    Ok(())
}