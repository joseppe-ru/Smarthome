use std::io::{Write, Read};
use std::ops::Deref;
use std::sync::Arc;
use futures::AsyncWriteExt;
use mqtt_packet_3_5::{ConnackPacket, ConnectPacket, MqttPacket, PacketDecoder};
use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex};
use crate::broker::message_queue::{MQ,MessageQueue};

mod tcp_write;
mod tcp_read;

#[derive(Debug)]
pub struct Client{
    pub connect_packet: ConnectPacket,
    tcp_stream: std::net::TcpStream,
    message_queue: MQ,
    pub writer: tcp_write::SW,
}
impl Client{
    pub async fn start_new_client(tcp_stream:tokio::net::TcpStream, mq:Arc<Mutex<MessageQueue>>)->Self{
        println!("[client] Neuer MQTT-Client wird verbunden");
        let tcp_stream=tcp_stream.into_std().expect("failed to convert tokio to std");
        //let tcp_clone = Arc::clone(&tcp_stream);
        let connect_packet = Client::read_connect_packet(tcp_stream.try_clone().expect("failed to clone tsc_stream")).unwrap();
        let message_queue = mq;

        // initialize writer
        let mut writer =Arc::new(Mutex::new(
            tcp_write::TcpStreamWriter::new(
                tcp_stream.try_clone().expect("failed to clone tsc_stream"),
                connect_packet.clone(),
            )
        ));


        {        // send CONNACK back to client
            let mut writer = writer.lock().await;
            writer.write_packet(MqttPacket::Connack(ConnackPacket {
                // 0 means success in Mqtt V3, V5 uses reason_code
                return_code: Some(0),
                // hardcode values for now
                reason_code: None,
                session_present: false,
                properties: None,
            })).await;
        }


        let cloned_queue=Arc::clone(&message_queue);
        let cloned_packet=connect_packet.clone();
        let cloned_tcp = tcp_stream.try_clone().expect("failed to clone tsc_stream");
        let cloned_writer = Arc::clone(&writer);

        let client = Self {
            connect_packet:cloned_packet,
            message_queue:cloned_queue,
            tcp_stream:cloned_tcp,
            writer:cloned_writer,
        };

        println!("[client] reader-Prozess wird gestartet");
        //spawn reader-Process
        //let client_clone = client.clone();
        let _ =tokio::spawn(async move{ tcp_read::read_tcp_stream(client).await});

        Self {
            connect_packet,
            message_queue,
            tcp_stream,
            writer,
        }
    }

    fn read_connect_packet(stream: std::net::TcpStream) -> Result<ConnectPacket,&'static str> {

        println!("analysieren des Connect-Paketes (stream: {:?}",stream);

        let mut packet_decoder:PacketDecoder<std::net::TcpStream> = PacketDecoder::from_stream(stream);

            match packet_decoder.decode_packet(3){
                Ok(MqttPacket::Connect(connect)) => {
                    println!("Connect Packet gefunden");
                    return Ok(connect)
                },
                Ok(packet) => {
                    panic!("Client sent incorrect packet as initial packet {packet:?}");
                },
                Err(e) => {
                    panic!("Malformed packet received from client! Error details: {e}");
                },
            }
    }
}