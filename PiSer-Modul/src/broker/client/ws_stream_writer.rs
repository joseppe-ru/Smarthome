use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use futures::SinkExt;
use futures::stream::SplitSink;
use mqtt_packet_3_5::{ ConnectPacket, MqttPacket,};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct WsWriter {
    cli_id:String,
}

impl WsWriter {

    pub fn new(id:String)->Self{Self{cli_id:id}}
    pub async fn write_stream(&mut self,mut ws_tx:Arc<Mutex<SplitSink<warp::ws::WebSocket,warp::ws::Message>>>,connect_packet: ConnectPacket, packet:MqttPacket)->bool{
        println!("[writer: {:?}] received packet to write: {:?}",self.cli_id,packet);

        let encoded_packet = packet.encode(connect_packet.protocol_version).expect("[writer] failed to encode ConAck");

        let mut ws_tx_lock = ws_tx.lock().await;
        ws_tx_lock.send(warp::ws::Message::binary(encoded_packet)).await.expect("[ws-writer] cannot send conn_ack");
        true
    }

    pub async fn handle_connect(&mut self,connect_packet: ConnectPacket, mut ws_tx:Arc<Mutex<SplitSink<warp::ws::WebSocket,warp::ws::Message>>>) ->bool{

        let mqtt_version=connect_packet.protocol_version;

        //TODO: auch mal in betracht ziehen:
        // let mut granted = vec![];
        // granted.push(mqtt_packet_3_5::Granted::QoS0)
        // MqttPacket::Suback(SubackPacket::new_v3(packet.message_id, granted))

        let packet=MqttPacket::Connack(mqtt_packet_3_5::ConnackPacket {
            // 0 means success in Mqtt V3, V5 uses reason_code
            return_code: Some(0),
            // hardcode values for now
            reason_code: None,
            session_present: false,
            properties:   None
        });

        //Protokoll Version 5 zu encoden funktioniert irgendwie nicht??!!!
        let encoded_packet: Vec<u8>= packet
            .encode(mqtt_version)
            .expect("[writer  ] failed to encode Acknowledged (wrong protokoll_version");

        // let encoded_packet=connack.encode(5).expect("[conn-ack] encoding failed");

        let mut ws_tx_lock = ws_tx.lock().await;
        ws_tx_lock.send(warp::ws::Message::binary(encoded_packet)).await.expect("[ws-writer] cannot send conn_ack");
        true
    }

    //tcp_stream_reader


}