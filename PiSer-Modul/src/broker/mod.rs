use std::{sync::{Arc}};
use std::io::{Read, Write};
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
        tokio::spawn(async move { handle_connect(stream, queue_clone).await });
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

async fn handle_connect(mut tokio_stream:TcpStream,queue:Arc<Mutex<MessageQueue>>)->Result<(),&'static str>{

        //tokio TcpStream in std TcpStream umwandeln
    let mut std_stream=Arc::new(Mutex::new(tokio_stream.into_std().expect("[broker  ] failed to convert tokio to std")));
    //let std_stream=tokio_stream;

    //let stream_clone = Arc::clone(&std_stream);
    //show_stream(stream_clone).await;
    //show_stream(Arc::clone(&std_stream)).await;

    //connectpacket raus lesen
    let conn_pack = TcpReader::get_connect_packet(
        Arc::clone(&std_stream)
    ).await.expect("[broker  ] Fehler beim finden des Connect Paketes");

        //neuer Tcp-Handler -> hat Funktionen: zum sperren des Cients; senden und empfagnen über Client.stream
    let tcp_writer = TcpWriter::new(conn_pack.client_id.clone());

        //neuer Client-Handler
    let client_handler = Arc::new(Mutex::new(Client::start_new_client(Arc::clone(&std_stream), Arc::clone(&queue), conn_pack, tcp_writer)));

        //neuer Reader
    let mut tcp_reader = TcpReader::new(Arc::clone(&client_handler),Arc::clone(&queue));


    tcp_reader.handle_connect().await;

        //alles andere lesen

        //reader prozess starten
    //let _ = tokio::spawn(async move{ tcp_reader.message_handler().await});
    tcp_reader.message_handler().await;
    Ok(())
}

async fn pace_holder(_name:&str)->Result<(), &'static str>{
    println!("[broker    ] Modul {_name} nicht aktiv!");
    sleep(Duration::from_millis(1000)).await;
    Ok(())
}

async fn show_stream(mut stream:Arc<Mutex<std::net::TcpStream>>){
    let mut stream = stream.lock().await;
    //let mut clone = stream.try_clone().unwrap();
    let mut buf:[u8;1024] = [0;1024];
    let bytes = stream.read(&mut buf).expect("[reader  ] failed to read stream");
    //stream.write_all(&buf).expect("failed to write buffer back");
    println!("[broker  ] counting bytes...{}",bytes);
    println!("[broker  ] Stream_data: {:X?}",buf);
    println!("[broker  ] [End of Stream]");
    stream.write_all(&buf).expect("failed to write buffer back");
}