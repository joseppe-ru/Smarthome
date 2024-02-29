use std::io::Write;
use std::sync::{Arc, MutexGuard};
use mqtt_packet_3_5::{ConnackPacket, ConnackProperties, ConnectPacket, MqttPacket, PacketDecoder, UserProperties};
use tokio::sync::Mutex;
use crate::broker::client::Client;

pub struct TCPHandler{
    cli_id:String,
}

impl TCPHandler{

    pub fn new(id:String)->Self{println!("neuer tcp-handler für {} (id)",id.clone());Self{cli_id:id}}
    pub fn write_stream(&mut self,mut std_stream:std::net::TcpStream,connect_packet: ConnectPacket, packet:MqttPacket)->bool{
        println!("[writer: {:?}] received packet to write: {:?}",connect_packet.client_id,packet);

        let encoded_packet = packet.encode(connect_packet.protocol_version).expect("[writer] failed to encode ConAck");

        match std_stream.write_all(&encoded_packet){
            Ok(_)=> { println!("[tcp-writer] successfully wrote Packet");true },
            Err(e)=> { eprintln!("[tcp-writer] Fehler beim senden von ConAck (err: {:?}",e);false }
        }
    }

    //das verbindung mqtt packet herrausfinden
    pub fn get_connect_packet(tcp_stream:std::net::TcpStream)->Result<ConnectPacket,&'static str>{
        println!("analysieren des Connect-Paketes");
        let mut packet_decoder:PacketDecoder<std::net::TcpStream> = PacketDecoder::from_stream(tcp_stream.try_clone().unwrap());

        match packet_decoder.decode_packet(3){
            Ok(MqttPacket::Connect(connect)) => {
                println!("[tcp_handler] Connect Packet gefunden (id {:?}, protokoll_v: {:?}, name: {:?})",connect.client_id.clone(),connect.protocol_version.clone(),connect.user_name.clone());
                return Ok(connect)
            },
            Ok(packet) => {
                panic!("[tcp_handler] Client sent incorrect packet as initial packet {packet:?}");
            },
            Err(e) => {
                panic!("Malformed packet received from client! Error details: {e}");
            },
        }
    }

    pub async fn handle_connect(client:Arc<Mutex<Client>>)->bool{
        let mut client = client.lock().await;
        let mqtt_version=client.connect_packet.protocol_version;
        println!("[handle connect] client id: {:?}, protokoll: {:?}",
                 client.connect_packet.client_id,
                 mqtt_version, );

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
            .expect("[conn-ack] failed to encode Acknowledged (wrong protokoll_version");

       // let encoded_packet=connack.encode(5).expect("[conn-ack] encoding failed");
        match client.tcp_stream.write_all(&encoded_packet){
            Ok(_)=>true,
            Err(e)=>{eprintln!("[conn-ack] failed to write (e: {:?}",e); false}
        }
    }

    //tcp_stream_reader
    pub async fn message_handler(&mut self,client:Arc<Mutex<Client>>){
        let mut client = client.lock().await;

        println!("[tcp-reader: {:?}] Read tcp stream...", client.connect_packet.client_id);
        let mut packet_decoder=PacketDecoder::from_stream(client.tcp_stream.try_clone().unwrap());
        let mqtt_version=client.connect_packet.protocol_version;

        while packet_decoder.has_more(){
            println!("[tcp-reader] has_more");
            match packet_decoder.decode_packet(mqtt_version){
                Ok(packet)=> match packet {
                    MqttPacket::Connect(_)=>{
                        println!("[tcp-reader] (Connect) nicht schon wieder");
                    },
                    MqttPacket::Pingreq=>{println!("[tcp-reader] received Pingreq!")},
                    MqttPacket::Publish(_)=>{println!("[tcp-reader] received Publish!")},
                    MqttPacket::Subscribe(sub)=>{
                        println!("[tcp-reader] received Subscribe!");
                        let client_clone = Arc::clone(&client);
                        let mut message_queue=client.message_queue.lock().await;
                        message_queue.subscribe(sub,client_clone);
                    },
                    _ => {eprintln!("unbekannter Packet-Typ");}
                },
                Err(_err)=>{eprintln!("[tcp-reader] Fehler beim auswerten eines Paketes: {:?}",_err)}
            }
        }
        println!("[tcp-reader] process stopped")
    }

}