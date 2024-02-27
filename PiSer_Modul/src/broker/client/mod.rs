use std::net::TcpStream;
use std::sync::Arc;
use mqtt_packet_3_5::ConnackPacket;
use tokio::sync::Mutex;
use crate::broker::message_queue::MessageQueue;

mod tcp_write;
mod tcp_read;

#[derive(Debug)]
pub struct Client{}
impl Client{
    pub fn new ()->Self{Self{}}

    pub async fn start_new_client(tcp_stream:TcpStream, mq:Arc<Mutex<MessageQueue>>)->Self{

        let connect_packet = Client::read_connect_packet(tcp_stream.clone());
        let message_queue: ProcessRef<MessageQueue> = ProcessRef::lookup(&MessageQueue::default())
            .expect("Failed to lookup MessageQueue process");

        // initialize writer process
        let writer = TcpStreamWriter::link()
            .start((tcp_stream.clone(), connect_packet.clone()))
            .expect("should have started writer");

        // send CONNACK back to client
        writer.write_packet(MqttPacket::Connack(ConnackPacket {
            // 0 means success in Mqtt V3, V5 uses reason_code
            return_code: Some(0),
            // hardcode values for now
            reason_code: None,
            session_present: false,
            properties: None,
        }));

        let Client = Self {

        };
        Client
    }

    fn read_connect_packet(stream:TcpStream)-> ConnackPacket{

    }
}