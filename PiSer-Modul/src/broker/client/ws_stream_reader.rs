use std::{sync::Arc};
use futures::StreamExt;
use mqtt_packet_3_5::{ConnectPacket, MqttPacket, PacketDecoder};
use tokio::{
    time::{sleep,Duration},
    sync::Mutex
};
use crate::broker::{message_queue::MessageQueue, client::MQTTClient, client::MqttWsClient, handle_ws_mqtt_message};
use crate::broker::client::KindOfClient;

#[derive(Debug)]
pub struct WSReader {
    client:Arc<Mutex<KindOfClient>>,
    queue:Arc<Mutex<MessageQueue>>,
}

impl WSReader {
    pub fn new (client:Arc<Mutex<KindOfClient>>, queue:Arc<Mutex<MessageQueue>>) ->Self{Self{client,queue}}

    pub async fn handle_connect(&mut self){
        let mut client = self.client.lock().await;
        match client{
            KindOfClient::MqttWsClient(mut client)=>{client.connection_handler().await;},
            _=>{}
            }

    }

    pub async fn message_handler(&mut self) {
        let client_lock = self.client.lock().await;
        //println!("[reader {:?}] Read tcp stream...", client_lock.connect_packet.client_id);
        let rx = match client_lock {
            KindOfClient::MqttWsClient(ws_client) => {
                Ok(ws_client.ws_rx)
            }
            _ => { Err("[wsreader] no rx today") }
        }.unwrap();

        let mut rx = rx.lock().await;
        while let Some(body) = rx.next().await {
            let message = match body {
                Ok(msg) => msg,
                Err(e) => {
                    println!("error reading websocket received message: {e}");
                    break;
                }
            };

            //let mut packet_decoder=PacketDecoder::from_stream(client_lock.tcp_stream.try_clone().unwrap());
            let buf = std::io::BufReader::new(message);
            let mut packet_decoder = PacketDecoder::from_bufreader(buf);

            //let _ = sleep(Duration::from_millis(10)).await;

            while packet_decoder.has_more() {
                println!("[reader  ] has_more");
                match packet_decoder.decode_packet(3) {
                    Ok(packet) => match packet {
                        MqttPacket::Connect(_) => {
                            println!("[reader  ] (Connect) nicht schon wieder");
                        },
                        MqttPacket::Pingreq => {
                            println!("[reader  ] received Pingreq!")
                        },
                        MqttPacket::Publish(publ) => {
                            println!("[reader  ] received Publish!");
                            let mut message_queue = self.queue.lock().await;
                            let client_clone = Arc::clone(&self.client);
                            message_queue.publish(publ, client_clone);
                        },
                        MqttPacket::Subscribe(sub) => {
                            println!("[reader  ] received Subscribe!");
                            let mut message_queue = self.queue.lock().await;
                            let client_clone = Arc::clone(&self.client);
                            message_queue.subscribe(sub, client_clone);
                        },//MessageQueue Mutex dropped here
                        fxxked_up_packet => { eprintln!("unbekannter Packet-Typ: {:?}", fxxked_up_packet); }
                    },
                    Err(err) => { eprintln!("[reader  ] Fehler beim auswerten eines Paketes: {:?}", err) }
                }
            }
            println!("[reader  ] process stopped")
        }
    }
}