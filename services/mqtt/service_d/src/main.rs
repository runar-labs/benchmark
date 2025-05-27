//! Service D for MQTT benchmark
//
// Intention: Receives AggregatedData from Service A, duplicates each record, and sends two records to Service E.
//
// Architectural boundaries:
// - Communicates only via MQTT broker (no direct service-to-service communication)
// - All data types are defined in services/common/types.rs
// - Handles errors and logs all major steps
//
// This implementation demonstrates the intended inter-service workflow for benchmarking.

use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use serde_json;
use std::time::Duration;
use tokio::time::timeout;
use services_common::AggregatedData;

const MQTT_BROKER: &str = "mqtt_broker";
const MQTT_PORT: u16 = 1883;
const TIMEOUT_MS: u64 = 5000;

#[tokio::main]
async fn main() {
    println!("[Service D] Starting and subscribing to requests");
    let mut mqttoptions = MqttOptions::new("service_d", MQTT_BROKER, MQTT_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 50);

    client.subscribe("service_d/request", QoS::AtLeastOnce).await.unwrap();

    loop {
        match timeout(Duration::from_millis(TIMEOUT_MS), eventloop.poll()).await {
            Ok(Ok(Event::Incoming(Packet::Publish(publish)))) => {
                let payload = publish.payload;
                match serde_json::from_slice::<AggregatedData>(&payload) {
                    Ok(data) => {
                        for i in 0..2 {
                            let payload = serde_json::to_vec(&data).unwrap();
                            client.publish("service_e/request", QoS::AtLeastOnce, false, payload).await.unwrap();
                            println!("[Service D] Duplicated and sent record {} for profile {}", i+1, data.profile.email);
                        }
                    }
                    Err(e) => {
                        eprintln!("[Service D] Failed to deserialize AggregatedData: {}", e);
                    }
                }
            }
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                eprintln!("[Service D] Eventloop error: {}", e);
            }
            Err(_) => {
                eprintln!("[Service D] Timeout waiting for request");
            }
        }
    }
}
