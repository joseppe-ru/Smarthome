use self::{
    tcp_stream_reader::read_tcp_stream,
    tcp_stream_writer::{TcpStreamWriter, TcpStreamWriterRequests},
};
use crate::message_queue::MessageQueue;
use lunatic::{ap::ProcessRef, net::TcpStream, spawn_link, AbstractProcess};
use mqtt_packet_3_5::{ConnackPacket, ConnectPacket, MqttPacket, PacketDecoder};
use serde::{Deserialize, Serialize};
use std::io::Read;
pub mod tcp_stream_reader;
pub mod tcp_stream_writer;

use std::io::Write;

#[derive(Debug, Serialize, Deserialize, Clone)]
// this struct will store some relevant client data
// as well as references to the reader and writer processes
pub struct Client {
    pub connect_packet: ConnectPacket,
    tcp_stream: TcpStream,
    message_queue: ProcessRef<MessageQueue>,
    pub(crate) writer: ProcessRef<TcpStreamWriter>,
}

impl Client {
    // note that both the reader and writer are linked, which means that if either
    // of the processes dies, all three will die. We will handle session disconnects
    // as well as "sticky" sessions in a future article
    pub fn start_new_client(tcp_stream: TcpStream) -> Self {
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

        let client = Self {
            connect_packet,
            writer,
            tcp_stream,
            message_queue,
        };
        let cloned = client.clone();

        // now we spawn the reader process
        let _ = spawn_link!(|cloned| read_tcp_stream(cloned));
        client
    }

    // this function will either deliver the packet or crash
    // because there's not much we can do if the client delivered an
    // because there's not much we can do if the client delivered an
    // because there's not much we can do if the client ddelivered an
    // invalid packet
    fn read_connect_packet(mut stream: TcpStream) -> ConnectPacket {

        //TCP Stream in Bytes
        let mut buf = [0;1024];
        stream.read(&mut buf).expect("failed to read stream");

        println!("Stream Data: {:?}",buf);

        stream.write_all(&buf).expect("failed to write buffer back");
        //was davon ist websocket und was ist mqtt?
        //die bytes der mqtt message an decoder übergeben

        let mut packet_decoder = PacketDecoder::from_stream(stream);

        while packet_decoder.has_more(){
            match packet_decoder.decode_packet(5){
                Ok(MqttPacket::Connect(connect)) => {println!("Connect Packet gefunden"); return connect },
                Ok(packet) => { panic!("Client sent incorrect packet as initial packet {packet:?}") },
                Err(e) => { panic!("Malformed packet received from client! Error details: {e}") },
            }
        }
        panic!("halt stop")
        /*
        match packet_decoder.decode_packet(3) {
            Ok(MqttPacket::Connect(connect)) => connect,
            Ok(packet) => panic!("Client sent incorrect packet as initial packet {packet:?}"),
            Err(e) => panic!("Malformed packet received from client! Error details: {e}"),
        }
        */
    }
}
