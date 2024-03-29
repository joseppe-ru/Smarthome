use std::any::Any;
use std::io::Write;
use std::net::TcpStream;
use mqtt_packet_3_5::{ ConnectPacket, MqttPacket,};

#[derive(Debug)]
pub struct TcpWriter {
    cli_id:String,
}

impl TcpWriter {

    pub fn new(id:String)->Self{Self{cli_id:id}}
    pub fn write_mqtt_stream(&mut self, mut std_stream:std::net::TcpStream, connect_packet: ConnectPacket, packet:MqttPacket) ->bool{
        let log_id = self.cli_id.clone();
        let conn_id = connect_packet.client_id.clone();
        println!("[writer: {log_id}] received packet for {conn_id} to write: {:?}",packet.type_id());

        let encoded_packet = packet.encode(3).expect("[writer] failed to encode ConAck");
        println!("[writer] stream:{:?}",std_stream.try_clone().unwrap());
        match std_stream.write_all(&encoded_packet){
            Ok(_)=> { println!("[writer  {log_id}] successfully wrote Packet");true },
            Err(e)=> { eprintln!("[writer  {log_id}] Fehler beim senden von ConAck (err: {:?}",e);false }
        }
    }

    pub async fn handle_mqtt_connect(&mut self, connect_packet: ConnectPacket, mut tcp_stream: TcpStream) ->bool{
        //let mut client = client.lock().await;
        let mqtt_version=connect_packet.protocol_version;
        let log_id = connect_packet.client_id;
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
            .expect("[writer] failed to encode Acknowledged (wrong protokoll_version");

        println!("[writer {log_id}] transmitting connect packet");
       // let encoded_packet=connack.encode(5).expect("[conn-ack] encoding failed");
        match tcp_stream.write_all(&encoded_packet){
            Ok(_)=>true,
            Err(e)=>{eprintln!("[writer  {log_id}] failed to write connect packet (e: {:?}",e); false}
        }
    }
}