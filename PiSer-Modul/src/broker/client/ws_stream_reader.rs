use core::panic;
use std::{sync::Arc};
use std::io::{BufReader, Cursor};
use futures::stream::SplitStream;
use futures::StreamExt;
use mqtt_packet_3_5::{ConnectPacket, MqttPacket, PacketDecoder};
use tokio::{
    time::{sleep,Duration},
    sync::Mutex
};
use crate::broker::{message_queue::MessageQueue,client::MQTTClient};
use crate::broker::client::{KindOfClient, MQTTWsClient};

#[derive(Debug)]
pub struct WsReader{
    client:Arc<Mutex<MQTTWsClient>>,
    queue:Arc<Mutex<MessageQueue>>,
}

impl WsReader {
    pub fn new (client:Arc<Mutex<MQTTWsClient>>, queue:Arc<Mutex<MessageQueue>>) ->Self{Self{client,queue}}

    //das verbindung mqtt packet herrausfinden
    pub async fn get_connect_packet(ws_rx: Arc<Mutex<SplitStream<warp::ws::WebSocket>>>)->Result<ConnectPacket,&'static str>{
        println!("[reader  ]analysieren des Connect-Paketes");
        let mut ws_rx_lock =ws_rx.lock().await;
        let recv = ws_rx_lock.next().await.expect("[ws-reader] failed to get nex message").unwrap();

        if !recv.clone().is_binary(){panic!("neinnnnn")}

        let buf = BufReader::new(Cursor::new(recv.into_bytes()));

        let mut packet_decoder = PacketDecoder::from_bufreader(buf);

        match packet_decoder.decode_packet(3){
            Ok(MqttPacket::Connect(connect)) => {
                println!("[reader  ] Connect Packet gefunden (id {:?}, protokoll_v: {:?}, name: {:?})",connect.client_id.clone(),connect.protocol_version.clone(),connect.user_name.clone());
                return Ok(connect)
            },
            Ok(packet) => {
                panic!("[reader  ] Client sent incorrect packet as initial packet {packet:?}");
            },
            Err(e) => {
                panic!("[reader  ] Malformed packet received from client! Error details: {e}");
            },
        }
    }

    pub async fn handle_connect(&mut self){
        let mut client = self.client.lock().await;
        client.connection_handler().await;
    }

    pub async fn message_handler(&mut self){

        let client_lock = self.client.lock().await;

        println!("[reader {:?}] Read tcp stream...", client_lock.connect_packet.client_id);
        let mut rx_lock = client_lock.ws_rx.lock().await;
        let recv = rx_lock.next().await.expect("[ws-reader] failed to get nex message").unwrap();

        if !recv.clone().is_binary(){return}
        let buf = BufReader::new(Cursor::new(recv.into_bytes()));
        let mut packet_decoder = PacketDecoder::from_bufreader(buf);

        let mqtt_version=client_lock.connect_packet.protocol_version;
        drop(rx_lock);//rx Mutex dropped here
        drop(client_lock);//client Mutex dropped here

        let _ = sleep(Duration::from_millis(10)).await;

        while packet_decoder.has_more(){
            println!("[reader  ] has_more");
            match packet_decoder.decode_packet(mqtt_version){
                Ok(packet)=> match packet {
                    MqttPacket::Connect(_)=>{
                        println!("[reader  ] (Connect) nicht schon wieder");
                    },
                    MqttPacket::Pingreq=>{
                        println!("[reader  ] received Pingreq!")
                    },
                    MqttPacket::Publish(publ)=>{
                        println!("[reader  ] received Publish!");
                        let mut message_queue=self.queue.lock().await;
                        let client_clone = Arc::clone(&self.client);
                        message_queue.publish(publ,KindOfClient::WsKind(client_clone));
                    },
                    MqttPacket::Subscribe(sub)=>{
                        println!("[reader  ] received Subscribe!");
                        let mut message_queue=self.queue.lock().await;
                        let client_clone = Arc::clone(&self.client);
                        message_queue.subscribe(sub,KindOfClient::WsKind(client_clone));
                    },//MessageQueue Mutex dropped here
                    fxxked_up_packet => {eprintln!("unbekannter Packet-Typ: {:?}",fxxked_up_packet);}
                },
                Err(err)=>{eprintln!("[reader  ] Fehler beim auswerten eines Paketes: {:?}",err)}
            }
        }
        println!("[reader  ] process stopped")
    }
}