use std::collections::{HashMap, VecDeque};
use std::sync::{Arc};
use tokio::sync::Mutex;
use mqtt_packet_3_5::{MqttPacket, PublishPacket, SubackPacket, SubscribePacket};
use rand::Rng;

use crate::broker::client::{KindOfClient, MQTTClient};

#[derive(Debug)]
pub struct WorkerJob {
    pub job_id: u32,
    pub packet: MqttPacket,
    pub subscribers: Vec<KindOfClient>,
    pub client: KindOfClient,
}

#[derive(Debug, Default)]
pub struct MessageQueue{
    topic_subscription:HashMap<String,Vec<KindOfClient>>,
    jobs:VecDeque<WorkerJob>,
    pub job_counter:u32,
}

impl MessageQueue{
    pub fn new()->Self{Self::default()}

    pub fn add_job(&mut self, job:WorkerJob){
        self.job_counter += 1;
        self.jobs.push_back(job);
    }

    pub fn subscribe(&mut self, packet: SubscribePacket, client: KindOfClient) -> bool {
        println!("[queue   ] subscribe to topic");

        let mut granted = vec![];
        // add the client reference to each topic
        for subscription in packet.subscriptions.into_iter() {
            println!("[queue   ] pushed sub to topic {:?}",subscription.topic);
            let subscribers = self.topic_subscription.entry(subscription.topic).or_default();
            subscribers.push(match client{
                KindOfClient::WsKind(ref ws)=>KindOfClient::WsKind(Arc::clone(ws)),
                KindOfClient::MQTTKind(ref mq)=>KindOfClient::MQTTKind(Arc::clone(mq))
            },);

            // we will need to send a SUBACK to the subscriber
            // in which we will let the subscriber know which QoS level
            // was granted for each topic
            // NOTE: for now we will send QoS level 0 because that's the only
            // level the MQTT broker supports for now
            granted.push(mqtt_packet_3_5::Granted::QoS0);
        }

        // create a new job for sending the SUBACK
        let new_job = WorkerJob {
            job_id:self.get_job_id(),
            // use helper method for creating a basic MQTTv3 SUBACK packet
            packet: MqttPacket::Suback(SubackPacket::new_v3(packet.message_id, granted)),
            subscribers: vec![],
            client:match client{
                KindOfClient::WsKind(ref ws)=>KindOfClient::WsKind(Arc::clone(ws)),
                KindOfClient::MQTTKind(ref mq)=>KindOfClient::MQTTKind(Arc::clone(mq))
            },
        };

        self.add_job(new_job);
        println!("[queue   ] pushed new worker job");
        true // let the Client know we registered him
    }
    pub fn publish(&mut self, packet: PublishPacket, sender_client: KindOfClient) -> bool {
        //neuer Job zum Senden eines Paketes anlegen
        let clients_sub= self.topic_subscription.get(&packet.topic)
            .expect("[queue  ] keine Clients zum topic gefunden");

        let mut client_sub = Vec::new();
            for sub_clients in self.topic_subscription.get(&packet.topic).expect("[queue  ] keine Clients zum topic gefunden"){
                client_sub.push(match sub_clients {
                KindOfClient::WsKind(ws)=>KindOfClient::WsKind(Arc::clone(ws)),
                KindOfClient::MQTTKind(mq)=>KindOfClient::MQTTKind(Arc::clone(mq))
            })
        };

            let id = self.get_job_id().clone();
            self.add_job(WorkerJob{
                job_id:id,
                packet:MqttPacket::Publish(packet.clone()),
                subscribers:client_sub ,
                client:match sender_client{
                    KindOfClient::WsKind(ref ws)=>KindOfClient::WsKind(Arc::clone(ws)),
                    KindOfClient::MQTTKind(ref mq)=>KindOfClient::MQTTKind(Arc::clone(mq))
                },
            });

        true

    }

    pub async fn get_next_job(&mut self) -> Option<WorkerJob> {
        //self.jobs.pop_front();
        match self.jobs.pop_front() {
            Some(job)=>{self.job_counter-=1; Some(job)},
            None=>None
        }

    }

    fn get_job_id(&mut self) -> u32 {
        let mut rand_gen = rand::thread_rng();
        let randnum = rand_gen.gen::<u32>();
        println!("[queue  ] random JobID: {randnum}");
        randnum
    }
}