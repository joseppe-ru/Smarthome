use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;
use mqtt_packet_3_5::{ConnectPacket, MqttPacket, PacketDecoder};
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::broker::client::Client;
use crate::broker::message_queue::MQ;

#[derive(Debug)]
pub struct TcpReader{
    client:Arc<Mutex<Client>>,
    queue:MQ,
}

impl TcpReader {
    pub fn new (client:Arc<Mutex<Client>>,queue:MQ) ->Self{Self{client,queue}}

    //das verbindung mqtt packet herrausfinden
    pub async fn get_connect_packet(tcp_stream:Arc<Mutex<std::net::TcpStream>>)->Result<ConnectPacket,&'static str>{
        let tcp_stream = tcp_stream.lock().await;
        println!("[reader  ]analysieren des Connect-Paketes");
        let mut packet_decoder:PacketDecoder<std::net::TcpStream> = PacketDecoder::from_stream(tcp_stream.try_clone().unwrap());


            match packet_decoder.decode_packet(3) {
                Ok(MqttPacket::Connect(connect)) => {
                    println!("[reader  ] Connect Packet gefunden (id {:?}, protokoll_v: {:?}, name: {:?})", connect.client_id.clone(), connect.protocol_version.clone(), connect.user_name.clone());
                    return Ok(connect)
                },
                Ok(_packet) => {
                    panic!("[reader  ] Client sent incorrect packet as initial packet {_packet:?}");
                },
                Err(e) => {
                    panic!("[reader  ] Malformed packet received from client! Error details: {e}");
                },
            }

    }

    pub async fn handle_connect(&mut self){
        let mut client = self.client.lock().await;
        client.connection_handler().await;
    }

    pub async fn message_handler(&mut self){
        let client_lock = self.client.lock().await;

        //show_stream(client_lock.tcp_stream.try_clone().expect("[reader  ] failed to clone stream"));
        //let mqtt_version=client_lock.connect_packet.protocol_version;

        let tcp_stream=client_lock.tcp_stream.lock().await;

        println!("[reader {:?}] Read tcp stream... {:?}", client_lock.connect_packet.client_id,tcp_stream.try_clone().unwrap());
        let mut packet_decoder=PacketDecoder::from_stream(tcp_stream.try_clone().unwrap());
        //drop(client_lock);//client Mutex dropped here
        sleep(Duration::from_millis(10)).await;
        while packet_decoder.has_more(){
            println!("[reader  ] has_more");
            match packet_decoder.decode_packet(3){
                Ok(packet)=> match packet {
                    MqttPacket::Connect(_)=>{
                        println!("[reader  ] (Connect) nicht schon wieder");
                    },
                    MqttPacket::Pingreq=>{
                        println!("[reader  ] received Pingreq!")
                    },
                    MqttPacket::Publish(publ)=>{
                        println!("[reader  ] received Publish!");
                        let mut message_queue=self.queue.lock().await;
                        let client_clone = Arc::clone(&self.client);
                        message_queue.publish(publ,client_clone);
                    },
                    MqttPacket::Subscribe(sub)=>{
                        println!("[reader  ] received Subscribe!");
                        let mut message_queue=self.queue.lock().await;
                        let client_clone = Arc::clone(&self.client);
                        message_queue.subscribe(sub,client_clone);
                    },//MessageQueue Mutex dropped here
                    fxxked_up_packet => {eprintln!("unbekannter Packet-Typ: {:?}",fxxked_up_packet);}
                },
                Err(err)=>{eprintln!("[reader  ] Fehler beim auswerten eines Paketes: {:?}",err)}
            }
        }
        println!("[reader  ] process stopped")
    }
}

fn show_stream(mut stream:std::net::TcpStream){
    let mut buf:[u8;1024] = [0;1024];
    let bytes = stream.read(&mut buf).expect("[reader  ] failed to read stream");
    println!("[reader  ] counting bytes...{}",bytes);
    println!("[reader  ] Stream_data: {:X?}",buf);
    println!("[reader  ] [End of Stream]");
}