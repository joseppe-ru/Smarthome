use std::sync::{Arc};
use std::time::Duration;
use mqtt_packet_3_5::{MqttPacket,PublishPacket};
use tokio::{sync::Mutex,time::sleep};
use std::net::TcpListener;

mod client;
mod message_queue;
mod worker;

pub async fn broker_setup()->Result<(),&'static str>{
    println!("Broker wird gestartet");

    //message queue starten (mehr so ne referenz auf die Messagequeue)
    let queue = Arc::new(Mutex::new(message_queue::MessageQueue::new()));
    //worker starten (Consumer der Queue)
    let queue_clone=Arc::clone(&queue);
    let worker = tokio::spawn(async move { worker::worker_process(queue_clone).await });


    let listener = TcpListener::bind("0.0.0.0:1884")?;
    while let Ok((stream,_)) = listener.accept(){
        println!("Neuer MQTT_Client connected: {:?}",stream.peer_addr());
        let queue_clone=Arc::clone(&queue);
        let client = tokio::spawn(async move {client::Client::start_new_client(stream,queue_clone).await});
    }

    //TODO: tokio::join_handle

    Ok(())
}

async fn prod(mut mq:message_queue::MQ){
    loop{
        println!("[producer 1]");
        let pub_pack=MqttPacket::Publish(PublishPacket{dup:true,qos:1,retain:true,topic:String::new(),message_id:None,payload:vec![1],properties:None}) ;
        let jobber = message_queue::WorkerJob{job_id:12,packet:pub_pack,subscribers:vec![],sender:client::Client{}};
        {
            let mut mq = mq.lock().await;
            mq.add_job(jobber);
        }
        sleep(Duration::from_millis(1900)).await;
    }
}