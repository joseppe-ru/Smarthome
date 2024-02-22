use std::collections::{HashMap, VecDeque};
use mqtt_packet_3_5::{MqttPacket, PublishPacket, SubackPacket, SubscribePacket};
use serde::{Deserialize, Serialize};

use crate::broker::client::Client;


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
    fn init()->Result<Self,()>{Ok(Self::default())}

    fn terminate(self) {
        println!("Shutdown process");
    }

    fn handle_link_death(&self) {
        println!("Link trapped");
    }

    fn subscribe(&mut self, packet: SubscribePacket, sender: Client) -> bool {false}
    fn publish(&mut self, packet: PublishPacket, sender: Client) -> bool {false}

    fn get_next_job(&mut self) -> Option<WorkerJob> {
        self.jobs.pop_front()
    }

    fn get_job_id(&mut self) -> u32 {1}
}