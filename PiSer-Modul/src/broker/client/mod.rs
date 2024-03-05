use std::{
    sync::Arc
};
use futures::stream::{SplitSink, SplitStream};
use mqtt_packet_3_5::{ConnectPacket, MqttPacket};
use tokio::{sync::Mutex};
use crate::broker::{
    message_queue::{MessageQueue},
    TcpWriter,
    client::ws_stream_writer::WsWriter,
};

pub mod tcp_stream_writer;
pub(crate) mod tcp_stream_reader;
pub mod ws_stream_writer;
mod ws_stream_reader;

#[derive(Debug)]
pub enum KindOfClient{
    WsKind(Arc<Mutex<MQTTWsClient>>),
    MQTTKind(Arc<Mutex<MQTTClient>>),
}

#[derive(Debug)]
pub struct MQTTClient {
    pub connect_packet: ConnectPacket,
    pub tcp_stream: std::net::TcpStream,
    pub message_queue: Arc<Mutex<MessageQueue>>,
    tcp_writer: TcpWriter,
}

impl MQTTClient {
    pub fn start_new_client(std_stream:std::net::TcpStream, mq:Arc<Mutex<MessageQueue>>, conn_pack:ConnectPacket, tcp_handler:TcpWriter) ->Self{
        println!("[client: {:?}] wird erstellt",conn_pack.client_id);
        //neuen Client initialisieren > mit Mutex sperren um Fehlerhaft Zugriffe zu vermeiden
            let client = Self {
                connect_packet:conn_pack,
                tcp_stream:std_stream,
                message_queue:mq,
                tcp_writer: tcp_handler,
            };
        client
    }

    pub async fn connection_handler(&mut self){
        self.tcp_writer.handle_connect(
            self.connect_packet.clone(),
            self.tcp_stream.try_clone().expect("[client] failed to clone tcp_stream")
        ).await;
    }

    pub async fn write(&mut self,packet:MqttPacket){
        self.tcp_writer.write_stream(self.tcp_stream.try_clone().unwrap(),self.connect_packet.clone(),packet);
    }
}

#[derive(Debug)]
struct MQTTWsClient{
    pub connect_packet: ConnectPacket,
    ws_writer: WsWriter,
    pub message_queue: Arc<Mutex<MessageQueue>>,
    pub ws_rx: Arc<Mutex<SplitStream<warp::ws::WebSocket>>>,
    pub ws_tx: Arc<Mutex<SplitSink<warp::ws::WebSocket,warp::ws::Message>>>,
}

impl MQTTWsClient {
    pub fn start_new_client(ws_writer: WsWriter,
                            message_queue: Arc<Mutex<MessageQueue>>,
                            connect_packet: ConnectPacket,
                            ws_rx: Arc<Mutex<SplitStream<warp::ws::WebSocket>>>,
                            ws_tx:Arc<Mutex<SplitSink<warp::ws::WebSocket,warp::ws::Message>>>) ->Self{

        println!("[client: {:?}] wird erstellt",connect_packet.client_id);
        //neuen Client initialisieren > mit Mutex sperren um Fehlerhaft Zugriffe zu vermeiden
        let client = Self {
            connect_packet,
            ws_writer,
            message_queue,
            ws_rx,
            ws_tx,
        };
        client
    }

    pub async fn connection_handler(&mut self){
        self.ws_writer.handle_connect(self.connect_packet.clone(),Arc::clone(&self.ws_tx)).await;
    }

    pub async fn write(&mut self,packet:MqttPacket){
        self.ws_writer.write_stream(Arc::clone(&self.ws_tx),self.connect_packet.clone() ,packet).await;
    }
}