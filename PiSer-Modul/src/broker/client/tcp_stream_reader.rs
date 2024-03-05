use std::{sync::Arc};
use std::io::{BufReader, Cursor};
use futures::StreamExt;
use mqtt_packet_3_5::{ConnectPacket, MqttPacket, PacketDecoder};
use tokio::{
    time::{sleep,Duration},
    sync::Mutex
};
use tokio::net::TcpStream;
use tokio::sync::MutexGuard;
use warp::ws::Message;
use crate::broker::{message_queue::MessageQueue,client::MQTTClient};
use crate::broker::client::KindOfClient;
use crate::broker::client::mqtt_ws_client::MqttWsClient;

#[derive(Debug)]
pub struct TcpReader{
    client:KindOfClient,
    queue:Arc<Mutex<MessageQueue>>,
}

impl TcpReader {
    pub fn new (client:KindOfClient, queue:Arc<Mutex<MessageQueue>>) ->Self{Self{client,queue}}

    //das verbindung mqtt packet herrausfinden
    pub fn get_mqtt_connect_packet(tcp_stream:std::net::TcpStream) ->Result<ConnectPacket,&'static str>{
        let mut packet_decoder:PacketDecoder<std::net::TcpStream> = PacketDecoder::from_stream(tcp_stream.try_clone().unwrap());

        match packet_decoder.decode_packet(3){
            Ok(MqttPacket::Connect(connect)) => {
                println!("[reader] Connect Packet gefunden (id {:?}, protokoll_v: {:?}, name: {:?})",connect.client_id.clone(),connect.protocol_version.clone(),connect.user_name.clone());
                return Ok(connect)
            },
            Ok(packet) => {
                panic!("[reader] Client sent incorrect packet as initial packet {packet:?}");
            },
            Err(e) => {
                panic!("[reader] Malformed packet received from client! Error details: {e}");
            },
        }
    }

    pub fn get_ws_connect_packet(message:warp::ws::Message)->Result<ConnectPacket,&'static str>{

        if message.is_binary(){
            let buf_reader = BufReader::new(Cursor::new(message.into_bytes()));
            let mut packet_decoder=PacketDecoder::from_bufreader(buf_reader);

            match packet_decoder.decode_packet(3){
                Ok(MqttPacket::Connect(connect)) => {
                    println!("[reader] Connect Packet gefunden (id {:?}, protokoll_v: {:?}, name: {:?})",connect.client_id.clone(),connect.protocol_version.clone(),connect.user_name.clone());
                    return Ok(connect)
                },
                Ok(packet) => {
                    panic!("[reader] Client sent incorrect packet as initial packet {packet:?}");
                },
                Err(e) => {
                    panic!("[reader] Malformed packet received from client! Error details: {e}");
                },
            }

        }else {
            Err("[reader] ws-message was not a mqtt packet (not binary)")
        }
    }

    pub async fn handle_connect(&mut self){
        match self.client.clone(){
            KindOfClient::MqttClient(mut mqtt) => {
                let mut mqtt_lock = mqtt.lock().await;
                mqtt_lock.connection_handler().await;
            },
            KindOfClient::MqttWsClient(mut ws) => {
                let mut ws_lock = ws.lock().await;
                ws_lock.connection_handler().await
            },
        };
    }

    async fn mqtt_packet_decoder(&mut self,client:Arc<Mutex<MQTTClient>>)->bool{
        let client_lock = client.lock().await;
        let mut packet_decoder =  PacketDecoder::from_stream(client_lock.tcp_stream.try_clone().unwrap());
        let log_id = client_lock.connect_packet.client_id.clone();
        drop(client_lock);

        let _ = sleep(Duration::from_millis(10)).await;

        while packet_decoder.has_more(){
            println!("[reader {log_id}] has_more_mqtt");
            match packet_decoder.decode_packet(3){
                Ok(packet)=> match packet {
                    MqttPacket::Connect(_)=>{
                        println!("[reader {log_id}] (Connect) nicht schon wieder");
                    },
                    MqttPacket::Pingreq=>{
                        println!("[reader {log_id}] received Pingreq!")
                    },
                    MqttPacket::Publish(publ)=>{
                        println!("[reader {log_id}] received Publish!");
                        let mut message_queue=self.queue.lock().await;
                        //let client_clone = Arc::clone(&self.client);
                        message_queue.publish(publ,self.client.clone());
                    },
                    MqttPacket::Subscribe(sub)=>{
                        println!("[reader {log_id}] received Subscribe!");
                        let mut message_queue=self.queue.lock().await;
                        //let client_clone = Arc::clone(&self.client);
                        message_queue.subscribe(sub,self.client.clone());
                    },//MessageQueue Mutex dropped here
                    packet => {eprintln!("[reader {log_id}] unbekannter Packet-Typ: {:?}",packet);}
                },
                Err(err)=>{eprintln!("[reader {log_id}] Fehler beim auswerten eines Paketes: {:?}",err)}
            }
        }
        println!("[reader {log_id}] process stopped");
        false
    }

    async fn ws_packet_decoder(&mut self,client:Arc<Mutex<MqttWsClient>>)->bool {
        let client_lock=client.lock().await;
        let mut rx_lock = client_lock.ws_rx.lock().await;

        let message = rx_lock.next().await.unwrap().unwrap();
        let buf_reader = BufReader::new(Cursor::new(message.into_bytes()));
        let mut packet_decoder = PacketDecoder::from_bufreader(buf_reader);
        let log_id = client_lock.connect_packet.client_id.clone();
        drop(rx_lock);
        drop(client_lock);

        let _ = sleep(Duration::from_millis(10)).await;

        while packet_decoder.has_more(){
            println!("[reader {log_id}] has_more_ws");
            match packet_decoder.decode_packet(3){
                Ok(packet)=> match packet {
                    MqttPacket::Connect(_)=>{
                        println!("[reader {log_id}] (Connect) nicht schon wieder");
                    },
                    MqttPacket::Pingreq=>{
                        println!("[reader {log_id}] received Pingreq!")
                    },
                    MqttPacket::Publish(publ)=>{
                        println!("[reader {log_id}] received Publish!");
                        let mut message_queue=self.queue.lock().await;
                        //let client_clone = Arc::clone(&self.client);
                        message_queue.publish(publ,self.client.clone());
                    },
                    MqttPacket::Subscribe(sub)=>{
                        println!("[reader {log_id}] received Subscribe!");
                        let mut message_queue=self.queue.lock().await;
                        //let client_clone = Arc::clone(&self.client);
                        message_queue.subscribe(sub,self.client.clone());
                    },//MessageQueue Mutex dropped here
                    fxxked_up_packet => {eprintln!("[reader {log_id}]unbekannter Packet-Typ: {:?}",fxxked_up_packet);}
                },
                Err(err)=>{eprintln!("[reader {log_id}] Fehler beim auswerten eines Paketes: {:?}",err)}
            }
        }
        println!("[reader {log_id}] process stopped");
        false
    }

    pub async fn message_handler(&mut self){
        //let client_lock = self.client.lock().await;
        match self.client.clone() {
            KindOfClient::MqttClient(mqtt) => self.mqtt_packet_decoder(mqtt).await,
            KindOfClient::MqttWsClient(ws) => self.ws_packet_decoder(ws).await,
        };
    }
}