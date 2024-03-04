use std::{
    sync::Arc,
    time::Duration,
};

use futures::{AsyncReadExt, StreamExt, TryStreamExt};
use futures::stream::{SplitSink, SplitStream};
use mqtt_packet_3_5::{ConnectPacket, MqttPacket, PacketDecoder};
use tokio::{
    sync::Mutex,
    time::sleep,
    io::AsyncBufReadExt,
    net::TcpListener,
};
use warp;

use crate::broker::{
    client::{
        MQTTClient,
        tcp_stream_writer::TcpWriter,
        tcp_stream_reader::TcpReader,
        mqtt_ws_client::MqttWsClient,
        KindOfClient,
        ws_stream_reader::WSReader,
        ws_stream_writer::WSWriter,
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
    //let mqtt_ws_listener = TcpListener::bind("0.0.0.0:1884").await.expect("[broker  ] failed to bind ws-address");
    tokio::spawn(
        async move {
            while let Ok((stream, _)) = mqtt_listener.accept().await {
                println!("[broker  ] Neuer MQTT_Client connected: {:?}", stream.peer_addr());
                let queue_clone = Arc::clone(&queue_clone_client);
                let s = tokio::spawn(handle_mqtt_connect(stream, queue_clone));
            }
        }
    );

    //tcp-listener für Websocket mit warp:

    let mqtt_route=warp::any()
        .and(warp::ws())
        .map(|ws:warp::ws::Ws| {
            println!("[broker  ] MQTT-Server für Websocket");
            ws.on_upgrade(move|websocket |async move {
                let queue_clone_ws_client = Arc::clone(&queue_clone_ws_client);
                handle_ws_connect(websocket,Arc::clone(&queue_clone_ws_client)).await.expect("[broker  ] Failed to handle new Websocket MQTT");
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

    if message.is_binary(){
        //TODO: connect packet erstellen
        let buf = std::io::BufReader::new(message);
        let mut packet_decoder = PacketDecoder::from_bufreader(buf);
        match packet_decoder.decode_packet(3){
            Ok(MqttPacket::Connect(connect)) => {
                println!("[broker  ] Connect Packet gefunden (id {:?}, protokoll_v: {:?}, name: {:?})",connect.client_id.clone(),connect.protocol_version.clone(),connect.user_name.clone());
                //TODO: die richtigen Klassen erstellen
                //Paket Writer
                let ws_writer= WSWriter::new(connect.client_id.clone());
                //WSClient
                let ws_client = Arc::new(Mutex::new(KindOfClient::MqttWsClient(MqttWsClient::start_new_client(rx,tx,queue,connect,ws_writer))));
                //Paket Reader
                let mut ws_reader = WSReader::new(ws_client, Arc::clone(&queue));
                //richtiges Packet
                ws_reader.handle_connect().await;

                ws_reader.message_handler().await;
                Ok(())
                //return handle_connect_comb(connect).await;
            },
            Ok(packet) => {
                panic!("[broker  ] Client sent incorrect packet as initial packet {packet:?}");
            },
            Err(e) => {
                panic!("[broker  ] Malformed packet received from client! Error details: {e}");
            },
        }
    }else{
        println!("[broker  ] WS: not a MQTT-Message (not binary)");
    }
}

async fn handle_ws_connect(websocket:warp::ws::WebSocket, queue:Arc<Mutex<MessageQueue>>) ->Result<(),&'static str>{
    //websocket upgrade mit warp
    //websocket chat server erstellen und dann in worker und jobs ... von MQTT-Broker integrieren
    let (tx,rx)=websocket.split();
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));
    //TODO: Mutex für rx und tx anlegen
    //TODO: Client Struktur mit stream oder websocket ausstatten können
    let rx_clone=Arc::clone(&rx);
    let mut rx = rx.lock().await;
    while let Some(body) = rx.next().await {
        let message = match body {
            Ok(msg) => msg,
            Err(e) => {
                println!("error reading websocket received message: {e}");
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
    let conn_pack = TcpReader::get_connect_packet(
        std_stream.try_clone().unwrap()
    ).expect("[broker  ] Fehler beim finden des Connect Paketes");


   // return handle_connect_comb(conn_pack).await;

    //neuer Tcp-Handler -> hat Funktionen: zum sperren des Cients; senden und empfagnen über Client.stream
    let tcp_writer = TcpWriter::new(conn_pack.client_id.clone());

    //neuer Client-Handler
    let client_handler = Arc::new(Mutex::new(MQTTClient::start_new_client(std_stream, Arc::clone(&queue), conn_pack, tcp_writer)));

    //neuer Reader
    let mut tcp_reader = TcpReader::new(Arc::clone(&client_handler),Arc::clone(&queue));

    //alles andere lesen
    tcp_reader.handle_connect().await;

    //reader prozess starten
    tcp_reader.message_handler().await;
    Ok(())

}

async fn handle_connect_comb(connect_packet: ConnectPacket)->Result<(),&'static str>{

}


async fn pace_holder(_name:&str)->Result<(), &'static str>{
    loop{
        println!("[main  ] Modul '{_name}' nicht aktiv!");
        sleep(Duration::from_millis(1000)).await;
    }
}