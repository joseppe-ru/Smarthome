use std::{
    sync::Arc,
    time::Duration,
};

use futures::{AsyncReadExt, StreamExt, TryStreamExt};
use tokio::{
    sync::Mutex,
    time::sleep,
    io::AsyncBufReadExt,
    net::TcpListener,
};

use crate::broker::{
    client::{
        MQTTClient,
        MQTTWsClient,
        tcp_stream_writer::TcpWriter,
        tcp_stream_reader::TcpReader,
        ws_stream_writer::WsWriter,
        ws_stream_reader::WsReader,
    },
    message_queue::MessageQueue,
};

use warp::{Filter};

mod client;
mod message_queue;
mod worker;


pub async fn broker_setup()->Result<(),&'static str>{
    println!("[broker  ] Broker wird gestartet");

    //message queue starten (mehr so ne referenz auf die Messagequeue)
    let queue = Arc::new(Mutex::new(message_queue::MessageQueue::new()));
    //worker starten (Consumer der Queue)
    let queue_clone_worker=Arc::clone(&queue);
    let queue_clone_client=Arc::clone(&queue);
    let queue_clone_ws_client=Arc::clone(&queue);
    tokio::spawn(async move { worker::worker_process(queue_clone_worker).await });

    println!("[broker] Listener wird gestartet");

    let mqtt_listener = TcpListener::bind("0.0.0.0:1885").await.expect("failed to bind address");

    tokio::spawn(
        async move {
            while let Ok((stream, _)) = mqtt_listener.accept().await {
                println!("[broker  ] Neuer MQTT_Client connected: {:?}", stream.peer_addr());
                let queue_clone = Arc::clone(&queue_clone_client);
                let s = tokio::spawn(handle_connect(stream, queue_clone));
            }
        }
    );

    //tcp-listener für Websocket mit warp:
    let mqtt_route=warp::any()
        .and(warp::ws())
        .map(move |ws:warp::ws::Ws|{
            println!("[broker] MQTT-Server für Websocket");
            let queue_clone_ws_client = Arc::clone(&queue_clone_ws_client);
            ws.on_upgrade(move|websocket |async move {
                handle_ws_connect(websocket, queue_clone_ws_client.clone()).await.expect("[broker] Failed to handle new Websocket MQTT: ");
            })
        });
    warp::serve(mqtt_route)
        //.bind_with_graceful_shutdown(([0,0,0,0],1886),async { shut_ws_mqtt_rx.await.ok(); });
        .bind(([0, 0, 0, 0], 1886)).await;


    Ok(())//Ok(pace_holder("broker").await.unwrap())
}

async fn handle_ws_connect(websocket:warp::ws::WebSocket,queue:Arc<Mutex<MessageQueue>>)->Result<(),&'static str>{

    let (tx,rx)=websocket.split();
    let ws_rx=Arc::new(Mutex::new(rx));
    let ws_tx=Arc::new(Mutex::new(tx));
    let conn_pack = WsReader::get_connect_packet(Arc::clone(&ws_rx)).await.expect("failed zo get connect_packet");

    let writer=WsWriter::new(conn_pack.client_id.clone());
    let client=Arc::new(Mutex::new(MQTTWsClient::start_new_client(writer,Arc::clone(&queue),conn_pack.clone(),Arc::clone(&ws_rx),Arc::clone(&ws_tx))));
    let mut reader = WsReader::new(client,queue);

    reader.handle_connect().await;
    reader.message_handler().await;


Ok(())
}

async fn handle_connect(tokio_stream:tokio::net::TcpStream,queue:Arc<Mutex<MessageQueue>>)->Result<(),&'static str>{

        //tokio TcpStream in std TcpStream umwandeln
    let std_stream=tokio_stream.into_std().expect("[broker  ] failed to convert tokio to std");
    //let std_stream=tokio_stream;

    //connectpacket raus lesen
    let conn_pack = TcpReader::get_connect_packet(
        std_stream.try_clone().unwrap()
    ).expect("[broker  ] Fehler beim finden des Connect Paketes");

        //neuer Tcp-Handler -> hat Funktionen: zum sperren des Cients; senden und empfagnen über Client.stream
    let tcp_writer = TcpWriter::new(conn_pack.client_id.clone());

        //neuer Client-Handler
    let client_handler = Arc::new(Mutex::new(MQTTClient::start_new_client(std_stream, Arc::clone(&queue), conn_pack, tcp_writer)));

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
    loop{
        println!("[main  ] Modul '{_name}' nicht aktiv!");
        sleep(Duration::from_millis(1000)).await;
    }
}