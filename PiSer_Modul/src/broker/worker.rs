use mqtt_packet_3_5::MqttPacket;
use tokio::time::{sleep, Duration};
use crate::broker::message_queue;

//TODO:
// Die Messagequeue muss hier irgendwie als Zeiger/Referenz übergeben werden
// sonst macht das lesen überhaupt keinen Sinn. Es steht nichts drin, da die
// Queue von niemanden befüllt werden kann
pub async fn worker_process(mut mq:message_queue::MQ){
    loop{
        let mut mq = mq.lock().await;
        match mq.get_next_job() {
            Some(job) => {
                println!("Element in Queue gefunden: {:?}",job);
                /*
                match job.packet{
                    publish @ MqttPacket::Publish(_)=>{
                        for subscriber in job.subscribers.into_iter(){
                            //subscriber.writer.write_packet(publish.clone());
                            println!("Publishing to all subscribers!! {:?},{:?}",subscriber,publish)
                        }
                    }
                }
                */
            }

            None => {
                println!("Queue ist leer...");
                sleep(Duration::from_millis(1000)).await; }
        }
    }
}