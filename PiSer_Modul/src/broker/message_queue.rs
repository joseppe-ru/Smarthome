use std::collections::{HashMap, VecDeque};
use std::sync::{Arc};
use tokio::sync::Mutex;
use mqtt_packet_3_5::{MqttPacket, PublishPacket, SubackPacket, SubscribePacket};
use serde::{Deserialize, Serialize};

use crate::broker::client::Client;

pub type MQ = Arc<Mutex<MessageQueue>>;

#[derive(Debug)]
pub struct WorkerJob {
    pub job_id: u32,
    pub packet: MqttPacket,
    pub subscribers: Vec<Client>,
    pub sender: Client,
}

#[derive(Debug, Default)]
pub struct MessageQueue{
    topic_subscription:HashMap<String,Vec<Client>>,
    jobs:VecDeque<WorkerJob>,
    pub job_counter:u32,
}

impl MessageQueue{
    pub fn new()->Self{Self::default()}

    //fn terminate(self) {println!("Shutdown process");}

    //fn handle_link_death(&self) {println!("Link trapped");}

    pub fn add_job(&mut self, job:WorkerJob){
        self.job_counter += 1;
        self.jobs.push_back(job);
    }

    pub fn subscribe(&mut self, packet: SubscribePacket, sender: Arc<Mutex<Client>>) -> bool {
        let mut granted = vec![];
        // add the client reference to each topic
        for subscription in packet.subscriptions.into_iter() {
            let subscribers = self
                .topic_subscriptions
                .entry(subscription.topic)
                .or_default();
            subscribers.push(sender.clone());

            // we will need to send a SUBACK to the subscriber
            // in which we will let the subscriber know which QoS level
            // was granted for each topic
            // NOTE: for now we will send QoS level 0 because that's the only
            // level the MQTT broker supports for now
            granted.push(mqtt_packet_3_5::Granted::QoS0);
        }

        // create a new job for sending the SUBACK
        let job_id = self.get_job_id();
        self.jobs.push_back(WorkerJob {
            job_id,
            // use helper method for creating a basic MQTTv3 SUBACK packet
            packet: MqttPacket::Suback(SubackPacket::new_v3(packet.message_id, granted)),
            subscribers: vec![],
            sender,
        });
        true // let the Client know we registered him
    }
    fn publish(&mut self, packet: PublishPacket, sender: Client) -> bool {false}


    pub async fn get_next_job(&mut self) -> Option<WorkerJob> {
        //self.jobs.pop_front();
        match self.jobs.pop_front() {
            Some(job)=>{self.job_counter-=1; Some(job)},
            None=>None
        }

    }

    fn get_job_id(&mut self) -> u32 {1}
}