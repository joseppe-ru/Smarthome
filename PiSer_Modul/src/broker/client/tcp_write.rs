use std::sync::{Arc};
use tokio::{sync::Mutex};
use mqtt_packet_3_5::{ConnectPacket, MqttPacket};

pub type SW = Arc<Mutex<TcpStreamWriter>>;

#[derive(Debug)]
pub struct TcpStreamWriter{
    tcp_stream:std::net::TcpStream,
    connect_packet:ConnectPacket,
}

impl TcpStreamWriter{
    pub fn new(stream:std::net::TcpStream,conn_pack:ConnectPacket)-> Self{
        Self{
            tcp_stream:stream,
            connect_packet:conn_pack,
        }
    }

    pub async fn write_packet(&mut self,packet:MqttPacket)->bool{
        println!("[writer {}] received packet to write {:?}",self.connect_packet.client_id,packet);
        true
    }


}