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
    job_counter:u32,
}

impl MessageQueue{
    pub fn new()->Self{Self::default()}

    //fn terminate(self) {println!("Shutdown process");}

    //fn handle_link_death(&self) {println!("Link trapped");}

    pub fn add_job(&mut self, job:WorkerJob){self.jobs.push_back(job)}

    fn subscribe(&mut self, packet: SubscribePacket, sender: Client) -> bool {false}
    fn publish(&mut self, packet: PublishPacket, sender: Client) -> bool {false}


    pub fn get_next_job(&mut self) -> Option<WorkerJob> {
        self.jobs.pop_front()
    }

    fn get_job_id(&mut self) -> u32 {1}
}