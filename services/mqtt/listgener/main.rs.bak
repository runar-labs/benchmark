//! ListGener for MQTT benchmark
//
// Intention: Subscribes to all relevant MQTT topics, logs all messages received, but does not process or forward them.
//
// Architectural boundaries:
// - Communicates only via MQTT broker (no direct service-to-service communication)
// - Does not mutate, respond, or forward any messages
// - Logs all received messages with topic and size
//
// This implementation demonstrates a passive observer service for benchmarking.

use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use std::time::Duration;
use tokio::time::timeout;

const MQTT_BROKER: &str = "mqtt_broker";
const MQTT_PORT: u16 = 1883;
const TIMEOUT_MS: u64 = 5000;

const TOPICS: &[&str] = &[
    "service_b/request",
    "service_c/request",
    "service_a/response/b",
    "service_a/response/c",
    "service_d/request",
    "service_e/request",
];

#[tokio::main]
async fn main() {
    println!("[ListGener] Starting and subscribing to all topics");
    let mut mqttoptions = MqttOptions::new("listgener", MQTT_BROKER, MQTT_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 50);

    for &topic in TOPICS.iter() {
        client.subscribe(topic, QoS::AtLeastOnce).await.unwrap();
        println!("[ListGener] Subscribed to {}", topic);
    }

    loop {
        match timeout(Duration::from_millis(TIMEOUT_MS), eventloop.poll()).await {
            Ok(Ok(Event::Incoming(Packet::Publish(publish)))) => {
                println!("[ListGener] Received on topic {} ({} bytes)", publish.topic, publish.payload.len());
            }
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                eprintln!("[ListGener] Eventloop error: {}", e);
            }
            Err(_) => {
                eprintln!("[ListGener] Timeout waiting for message");
            }
        }
    }
}
