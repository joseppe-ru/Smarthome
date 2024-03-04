use std::sync::Arc;
use futures::stream::{SplitSink, SplitStream};
use mqtt_packet_3_5::{ConnectPacket, MqttPacket};
use tokio::sync::Mutex;
use crate::broker::client::tcp_stream_writer::TcpWriter;
use crate::broker::client::ws_stream_writer::WSWriter;
use crate::broker::message_queue::MessageQueue;

#[derive(Debug)]
pub struct MqttWsClient {
    pub connect_packet: ConnectPacket,
    pub message_queue: Arc<Mutex<MessageQueue>>,
    ws_writer: WSWriter,
    pub ws_rx:Arc<Mutex<SplitStream<warp::ws::WebSocket>>>,
    ws_tx:Arc<Mutex<SplitSink<warp::ws::WebSocket,warp::ws::Message>>>,
}

impl MqttWsClient {
    pub fn start_new_client(    ws_rx:Arc<Mutex<SplitStream<warp::ws::WebSocket>>>,
                                ws_tx:Arc<Mutex<SplitSink<warp::ws::WebSocket,warp::ws::Message>>>,
                                mq:Arc<Mutex<MessageQueue>>,
                                conn_pack:ConnectPacket,
                                ws_handler:WSWriter) ->Self{

        println!("[client: {:?}] wird erstellt",conn_pack.client_id);
        //neuen Client initialisieren > mit Mutex sperren um Fehlerhaft Zugriffe zu vermeiden
        let client = Self {
            connect_packet:conn_pack,
            ws_rx:ws_rx,
            ws_tx:ws_tx,
            message_queue:mq,
            ws_writer: ws_handler,
        };
        client
    }

    pub async fn connection_handler(&mut self){
        //TODO: write a connect packet
    }

    pub async fn write(&mut self,packet:MqttPacket){
        //TODO: write any paket
    }
}