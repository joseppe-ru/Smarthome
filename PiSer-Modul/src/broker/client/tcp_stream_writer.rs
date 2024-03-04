use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use mqtt_packet_3_5::{ ConnectPacket, MqttPacket,};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct TcpWriter {
    cli_id:String,
}

impl TcpWriter {

    pub fn new(id:String)->Self{Self{cli_id:id}}
    pub async fn write_stream(&mut self,mut std_stream:Arc<Mutex<std::net::TcpStream>>,connect_packet: ConnectPacket, packet:MqttPacket)->bool{
        let mut std_stream = std_stream.lock().await;
        println!("[writer: {:?}] received packet to write: {:?}",connect_packet.client_id,packet);

        let encoded_packet = packet.encode(connect_packet.protocol_version).expect("[writer] failed to encode ConAck");

        match std_stream.write_all(&encoded_packet){
            Ok(_)=> { println!("[writer  ] successfully wrote Packet");true },
            Err(e)=> { eprintln!("[writer  ] Fehler beim senden von ConAck (err: {:?}",e);false }
        }
    }

    pub async fn handle_connect(&mut self,connect_packet: ConnectPacket, mut tcp_stream: Arc<Mutex<std::net::TcpStream>>) ->bool{
        //let mut client = client.lock().await;
        let mut tcp_stream = tcp_stream.lock().await;
        let mqtt_version=connect_packet.protocol_version;
        println!("[writer  ] client id: {:?}, protokoll: {:?}",
                 connect_packet.client_id,
                 mqtt_version, );


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
        match tcp_stream.write_all(&encoded_packet){
            Ok(_)=>true,
            Err(e)=>{eprintln!("[writer  ] failed to write (e: {:?}",e); false}
        }
    }

    //tcp_stream_reader


}