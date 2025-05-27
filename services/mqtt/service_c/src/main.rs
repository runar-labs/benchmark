//! Service C for MQTT benchmark
//
// Intention: Receives a Profile from Service A, generates a list of ServiceRecord (length controlled by LIST_SIZE env var), and returns them to Service A via MQTT.
//
// Architectural boundaries:
// - Communicates only via MQTT broker (no direct service-to-service communication)
// - All data types are defined in services/common/types.rs
// - List size is controlled by LIST_SIZE env var (default: 5)
// - Handles errors and logs all major steps
//
// This implementation demonstrates the intended inter-service workflow for benchmarking.

use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use serde_json;
use std::env;
use std::time::Duration;
use tokio::time::timeout;
use services_common::{Profile, ServiceRecord};
use rand::{Rng, thread_rng, distributions::Alphanumeric};
use chrono::{Utc, Duration as ChronoDuration};

const MQTT_BROKER: &str = "mqtt_broker";
const MQTT_PORT: u16 = 1883;
const TIMEOUT_MS: u64 = 5000;

fn get_list_size() -> usize {
    env::var("LIST_SIZE").ok().and_then(|v| v.parse().ok()).unwrap_or(5)
}

fn generate_services(profile: &Profile, count: usize) -> Vec<ServiceRecord> {
    let mut rng = thread_rng();
    let now = Utc::now();
    (0..count).map(|i| {
        let id: String = (0..8).map(|_| rng.sample(Alphanumeric) as char).collect();
        let service_name = format!("{}_svc_{}", profile.name, i);
        let date = (now - ChronoDuration::days(i as i64)).to_rfc3339();
        let description = format!("Service {} for {}", i, profile.email);
        ServiceRecord { id, service_name, date, description }
    }).collect()
}

#[tokio::main]
async fn main() {
    println!("[Service C] Starting and subscribing to requests");
    let mut mqttoptions = MqttOptions::new("service_c", MQTT_BROKER, MQTT_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 50);

    client.subscribe("service_c/request", QoS::AtLeastOnce).await.unwrap();
    let list_size = get_list_size();
    println!("[Service C] Using LIST_SIZE={}", list_size);

    loop {
        match timeout(Duration::from_millis(TIMEOUT_MS), eventloop.poll()).await {
            Ok(Ok(Event::Incoming(Packet::Publish(publish)))) => {
                let payload = publish.payload;
                match serde_json::from_slice::<Profile>(&payload) {
                    Ok(profile) => {
                        let services = generate_services(&profile, list_size);
                        let response = (profile.email.clone(), services);
                        let payload = serde_json::to_vec(&response).unwrap();
                        client.publish("service_a/response/c", QoS::AtLeastOnce, false, payload).await.unwrap();
                        println!("[Service C] Responded for profile {}", profile.email);
                    }
                    Err(e) => {
                        eprintln!("[Service C] Failed to deserialize profile: {}", e);
                    }
                }
            }
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                eprintln!("[Service C] Eventloop error: {}", e);
            }
            Err(_) => {
                eprintln!("[Service C] Timeout waiting for request");
            }
        }
    }
}
