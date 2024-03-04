use std::sync::Arc;
use mqtt_packet_3_5::MqttPacket;
use tokio::{time::{sleep, Duration}};
use tokio::sync::Mutex;
use crate::broker::message_queue::MessageQueue;

//TODO:
// Die Messagequeue muss hier irgendwie als Zeiger/Referenz übergeben werden
// sonst macht das lesen überhaupt keinen Sinn. Es steht nichts drin, da die
// Queue von niemanden befüllt werden kann
pub async fn worker_process(mq:Arc<Mutex<MessageQueue>>){
    loop{
        //println!("[worker] Zugriff auf Queue");
        let mut mq = mq.lock().await;

        match mq.get_next_job().await {
            Some(job) => {
                println!("[worker] Element aus Queue: {:?}",job);
                match job.packet{
                    publish @ MqttPacket::Publish(_)=>{
                        for subscriber in job.subscribers.into_iter(){
                            //subscriber.writer.write_packet(publish.clone());
                            println!("Publishing to all subscribers!! {:?},{:?}",subscriber,publish);
                            let mut client = subscriber.lock().await;
                            client.write(publish.clone()).await;
                        }
                    }
                    suback @ MqttPacket::Suback(_)=>{
                        let mut client = job.client.lock().await;
                        client.write(suback).await;
                    }
                    nüscht => eprintln!("[worker  ] kein worker-job für packet: {:?}",nüscht)
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