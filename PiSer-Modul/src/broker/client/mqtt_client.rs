use std::{
    sync::Arc
};
use mqtt_packet_3_5::{ConnectPacket, MqttPacket};
use tokio::{sync::Mutex};
use crate::broker::{
    message_queue::{MessageQueue},
    TcpWriter
};

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
