use std::io::{Write, Read};
use std::ops::Deref;
use std::sync::Arc;
use futures::{AsyncBufReadExt, AsyncWriteExt};
use mqtt_packet_3_5::{ConnectPacket};
use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex};
use crate::broker::message_queue::{MQ,MessageQueue};

pub mod tcp_stream_handler;

#[derive(Debug)]
pub struct Client{
    pub connect_packet: ConnectPacket,
    tcp_stream: std::net::TcpStream,
    message_queue: MQ,
}

impl Client{
    pub fn start_new_client(std_stream:std::net::TcpStream, mq:Arc<Mutex<MessageQueue>>, conn_pack:ConnectPacket)->Self{
        println!("[client: {:?}] wird erstellt",conn_pack.client_id);
        //neuen Client initialisieren > mit Mutex sperren um Fehlerhaft Zugriffe zu vermeiden
            let client = Self {
                connect_packet:conn_pack,
                tcp_stream:std_stream,
                message_queue:mq,
            };
        client
    }
}

