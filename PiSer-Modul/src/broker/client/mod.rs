use std::sync::Arc;
use tokio::sync::Mutex;
use crate::broker::client::mqtt_client::MQTTClient;
use crate::broker::client::mqtt_ws_client::MqttWsClient;

pub mod tcp_stream_writer;
pub(crate) mod tcp_stream_reader;
pub mod mqtt_client;
pub mod mqtt_ws_client;

//Enum für verschiedenartige Clients
#[derive(Debug,Clone)]
pub enum KindOfClient {
    MqttClient(Arc<Mutex<MQTTClient>>),
    MqttWsClient(Arc<Mutex<MqttWsClient>>),
}