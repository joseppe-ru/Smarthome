use mqtt_packet_3_5::{MqttPacket, PacketDecoder};
use crate::broker::client::Client;

pub async fn read_tcp_stream(client: Client){
    println!("[reader: {}] Read tcp stream...", client.connect_packet.client_id);
    let mut packet_decoder=PacketDecoder::from_stream(client.tcp_stream);
    let mqtt_version=client.connect_packet.protocol_version;

    while packet_decoder.has_more(){
        match packet_decoder.decode_packet(mqtt_version){
            Ok(packet)=> match packet {
                MqttPacket::Pingreq=>{println!("[reader] received Pingreq!")},
                MqttPacket::Publish(_)=>{println!("[reader] received Publish!")},
                MqttPacket::Subscribe(_)=>{println!("[reader] received Subscribe!")},
                //Err(_)=>{eprintln!("Fehler beim auswerten eines Paketes")}
                _ => {}
            },
            Err(_err)=>{eprintln!("Fehler beim auswerten eines Paketes: {:?}",_err)}
        }
    }
    println!("[reader] process stopped")
}