use crate::broker::client::mqtt_client::MQTTClient;
use crate::broker::client::mqtt_ws_client::MqttWsClient;

pub mod tcp_stream_writer;
pub(crate) mod tcp_stream_reader;
pub mod mqtt_client;
pub mod mqtt_ws_client;
pub mod ws_stream_writer;
pub mod ws_stream_reader;

//Enum für verschiedenartige Clients
pub enum KindOfClient {
    MqttClient(MQTTClient),
    MqttWsClient(MqttWsClient),
}