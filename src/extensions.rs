use prost::Message;
use crate::pb::kafka as pb;
use crate::TopicBundle;

impl pb::Message {
    pub fn new(key: String, value: impl Message) -> Self {
        let mut msg = pb::Message::default();
        msg.key = key.as_bytes().to_vec();
        msg.value = value.encode_to_vec();
        msg
    }
}

impl pb::TopicBundle {
    pub fn new(topic: &str, schema_id: i32, messages: Vec<pb::Message>) -> Self {
        pb::TopicBundle {
            topic: topic.to_string(),
            schema_id,
            messages,
        }
    }
    pub fn add(&mut self, key: &str, value: impl Message) -> &mut Self {
        self.messages.push(pb::Message::new(key.to_string(), value));
        self
    }
}

impl pb::Publish {
    pub fn new() -> Self {
        pb::Publish::default()
    }

    pub fn add(&mut self, topic: &str, schema_id: i32, messages: Vec<pb::Message>) -> &mut Self {
        self.topic_bundles.push(pb::TopicBundle::new(topic, schema_id, messages));
        self
    }
}