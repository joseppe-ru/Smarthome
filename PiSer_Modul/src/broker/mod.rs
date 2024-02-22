mod client;
mod message_queue;
mod worker;

pub async fn broker_setup()->Result<(),&'static str>{
    println!("Broker wird gestartet");

    //message queue starten

    //worker starten

    //listener starten & Client_new_client rein binden

    Ok(())
}

