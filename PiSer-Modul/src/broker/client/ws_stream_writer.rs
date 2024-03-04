use std::io::Write;
use std::net::TcpStream;
use mqtt_packet_3_5::{ ConnectPacket, MqttPacket,};

#[derive(Debug)]
pub struct WSWriter {
    cli_id:String,
}

impl WSWriter {

    pub fn new(id:String)->Self{Self{cli_id:id}}
    pub fn write_stream(&mut self,connect_packet: ConnectPacket, packet:MqttPacket)->bool{
        println!("[writer: {:?}] received packet to write: {:?}",self.cli_id,packet);

        let encoded_packet = packet.encode(connect_packet.protocol_version).expect("[writer] failed to encode ConAck");

        //TODO: Write packet to WS-Client
    }

    pub async fn handle_connect(&mut self,connect_packet: ConnectPacket) ->bool{
        //let mut client = client.lock().await;
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
        //TODO: Write packet
    }

    //tcp_stream_reader


}