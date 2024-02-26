mod client;
mod message_queue;
mod worker;

pub async fn broker_setup()->Result<(),&'static str>{
    println!("Broker wird gestartet");

    //message queue starten
    let m_queue = message_queue::MessageQueue::start().unwrap();
    //worker starten
    let worker = tokio::spawn(worker::worker_process(m_queue));
    //listener starten & Client_new_client rein binden

    Ok(())
}

