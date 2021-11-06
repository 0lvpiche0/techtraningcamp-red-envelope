use std::collections::HashMap;
use crate::utils;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::ClientConfig;
use rdkafka::consumer::Consumer;


pub fn new_stream_consumer(config: &HashMap<String, String>, topics: &[String]) -> StreamConsumer {
    let mut client = ClientConfig::new();
    for (key, val) in config {
        client.set(key, val);
    }
    let stream_consumer: StreamConsumer = client.create().expect("Consumer creation failed");
    let topics: Vec<&str> = topics.iter().map(|s| s.as_str()).collect();
    stream_consumer
        .subscribe(&topics)
        .expect("Can't subscribe to specified topic");
    stream_consumer
}

fn get_topics() -> Vec<String> {
    vec!["test001".to_string()]
}



pub fn kafka_config_init() -> (HashMap<String, String>, Vec<String>) {
    let brokers = utils::get_env("ENVELOPE_BROKERS", "localhost:9092");
    let group_id = utils::get_env("ENVELOPE_GROUP_ID", "test");
    
    let mut config = std::collections::HashMap::new();
    config.insert("group.id".to_string(), group_id);
    config.insert("bootstrap.servers".to_string(), brokers);
    config.insert("enable.partition.eof".to_string(), "false".to_string());
    config.insert("session.timeout.ms".to_string(), "6000".to_string());
    config.insert("enable.auto.commit".to_string(), "false".to_string());
    let topics = get_topics();
    (config, topics)
}


#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn kakfa_client_test() {
        let (config, topics) = kafka_config_init();
        let mut client = ClientConfig::new();
        for (key, val) in config {
            client.set(key, val);
        }
        let stream_consumer: StreamConsumer = client.create().expect("Consumer creation failed");
        let topics: Vec<&str> = topics.iter().map(|s| s.as_str()).collect();
        stream_consumer
        .subscribe(&topics)
        .expect("Can't subscribe to specified topic");
    }
}
