use std::any::Any;
use std::sync::Arc;
use mqtt_packet_3_5::MqttPacket;
use tokio::{time::{sleep, Duration}};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::client;
use crate::broker::client::KindOfClient;
use crate::broker::message_queue::MessageQueue;

pub async fn worker_process(mq:Arc<Mutex<MessageQueue>>){
    loop{
        //println!("[worker] Zugriff auf Queue");
        let mut mq = mq.lock().await;

        match mq.get_next_job().await {
            Some(job) => {
                println!("[worker] Element aus Queue: {:?}",job.job_id);
                match job.packet{
                    publish @ MqttPacket::Publish(_)=>{
                        for subscriber in job.subscribers.into_iter(){
                            //subscriber.writer.write_packet(publish.clone());
                            println!("[worker] Publishing '{:?}' to sub '{:?}'",publish.type_id(),subscriber);
                            match job.client.clone(){
                                KindOfClient::MqttClient(mqtt)=>{
                                    let mut mqtt_lock = mqtt.lock().await;
                                    mqtt_lock.write(publish.clone()).await;
                                },
                                KindOfClient::MqttWsClient(ws)=>{
                                    let mut ws_lock = ws.lock().await;
                                    ws_lock.write(publish.clone()).await;
                                }
                            }
                        }
                    }
                    sub_ack @ MqttPacket::Suback(_)=>{
                        println!("[worker] Preparing Sub-Acknowledge{:?}", sub_ack.type_id());
                        match job.client.clone(){
                            KindOfClient::MqttClient(mqtt)=>{
                                let mut mqtt_lock = mqtt.lock().await;
                                mqtt_lock.write(sub_ack.clone()).await;
                            },
                            KindOfClient::MqttWsClient(ws)=>{
                                let mut ws_lock = ws.lock().await;
                                ws_lock.write(sub_ack.clone()).await;
                            }
                        }
                    }
                    packet => eprintln!("[worker] kein worker-job für packet: {:?}",packet.type_id())
                }
                drop(mq);  //Mutex fallen lassen:
            }

            None => {
                //println!("[worker] Queue ist leer...");
                drop(mq);  //Mutex fallen lassen:
                sleep(Duration::from_millis(100)).await; }
        }

    }
}