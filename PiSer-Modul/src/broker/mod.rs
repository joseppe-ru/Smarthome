use std::{
    sync::Arc,
    time::Duration,
};

use futures::{AsyncReadExt, StreamExt, TryStreamExt};
use futures::stream::{SplitSink, SplitStream};
use tokio::{
    sync::Mutex,
    time::sleep,
    io::AsyncBufReadExt,
    net::TcpListener,
};
use warp;

use crate::broker::{
    client::{
        mqtt_client::MQTTClient,
        tcp_stream_writer::TcpWriter,
        tcp_stream_reader::TcpReader,
        mqtt_ws_client::MqttWsClient,
        KindOfClient,
    },
    message_queue::MessageQueue,
};
use warp::{Filter};


mod client;
mod message_queue;
mod worker;



pub async fn broker_setup()->Result<(),&'static str>{
    println!("[broker] Broker wird gestartet");

    //message queue starten (mehr so ne referenz auf die Messagequeue)
    let queue = Arc::new(Mutex::new(message_queue::MessageQueue::new()));
    //worker starten (Consumer der Queue)
    let queue_clone_worker=Arc::clone(&queue);
    let queue_clone_client=Arc::clone(&queue);
    let queue_clone_ws_client=Arc::clone(&queue);
    tokio::spawn(async move { worker::worker_process(queue_clone_worker).await });

    println!("[broker] MQTT-Broker at Port: 1885");
    println!("[broker] MQTT-Broker for Websockets at Port: 1886");

    let mqtt_listener = TcpListener::bind("0.0.0.0:1885").await.expect("failed to bind address");
    tokio::spawn(
        async move {
            while let Ok((stream, _)) = mqtt_listener.accept().await {
                println!("[broker] MQTT_Client connected: {:?}", stream.peer_addr());
                let queue_clone = Arc::clone(&queue_clone_client);
                let s = tokio::spawn(handle_mqtt_connect(stream, queue_clone));
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

    Ok(())
}

async fn handle_ws_mqtt_message(message:warp::ws::Message,
                                rx:Arc<Mutex<SplitStream<warp::ws::WebSocket>>>,
                                tx:Arc<Mutex<SplitSink<warp::ws::WebSocket,warp::ws::Message>>>,
                                queue:Arc<Mutex<MessageQueue>>)->Result<(),&'static str>{

    //Connect packet herraus nehmen
    let conn_pack=TcpReader::get_ws_connect_packet(message).expect("Failed to parse ws-message: ");

    //neuer writer
    let writer = TcpWriter::new(conn_pack.client_id.clone());

    //neuer client
    let ws_client = Arc::new(Mutex::new(MqttWsClient::start_new_client(rx,tx,Arc::clone(&queue),conn_pack,writer)));

    //neuer Reader
    let mut reader = TcpReader::new(KindOfClient::MqttWsClient(ws_client), Arc::clone(&queue));

    reader.handle_connect().await;

    reader.message_handler().await;

    Ok(())

}

async fn handle_ws_connect(websocket:warp::ws::WebSocket, queue:Arc<Mutex<MessageQueue>>) ->Result<(),&'static str>{
    //websocket upgrade mit warp
    //websocket chat server erstellen und dann in worker und jobs ... von MQTT-Broker integrieren
    let (tx,rx)=websocket.split();
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    let rx_clone=Arc::clone(&rx);

    //nachricht aus websocket stream lesen
    let mut rx = rx.lock().await;
    while let Some(body) = rx.next().await {
        let message = match body {
            Ok(msg) => msg,
            Err(e) => {
                println!("[broker] error reading websocket received message: {e}");
                break;
            }
        };
        return handle_ws_mqtt_message(message,Arc::clone(&rx_clone),Arc::clone(&tx),Arc::clone(&queue)).await;
    }

Ok(())
}

async fn handle_mqtt_connect(tokio_stream:tokio::net::TcpStream, queue:Arc<Mutex<MessageQueue>>) ->Result<(),&'static str>{

        //tokio TcpStream in std TcpStream umwandeln
    let std_stream=tokio_stream.into_std().expect("[broker  ] failed to convert tokio to std");
    //let std_stream=tokio_stream;

    //connectpacket raus lesen
    let conn_pack = TcpReader::get_mqtt_connect_packet(
        std_stream.try_clone().unwrap()
    ).expect("[broker] Fehler beim finden des Connect Paketes: ");

    //neuer Tcp-Handler -> hat Funktionen: zum sperren des Cients; senden und empfagnen über Client.stream
    let tcp_writer = TcpWriter::new(conn_pack.client_id.clone());

    //neuer Client-Handler
    let client_handler = Arc::new(Mutex::new(MQTTClient::start_new_client(std_stream, Arc::clone(&queue), conn_pack, tcp_writer)));

    //neuer Reader
    let mut tcp_reader = TcpReader::new(
        KindOfClient::MqttClient(client_handler)
        ,Arc::clone(&queue)
    );

    //alles andere lesen
    tcp_reader.handle_connect().await;

    //reader prozess starten
    tcp_reader.message_handler().await;
    Ok(())
}

async fn pace_holder(_name:&str)->Result<(), &'static str>{
    loop{
        println!("[broker] Modul '{_name}' nicht aktiv!");
        sleep(Duration::from_millis(1000)).await;
    }
}