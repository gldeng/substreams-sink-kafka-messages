use std::collections::HashMap;
use crate::pb::kafka as pb;
use crate::Publish;

#[derive(Debug)]
pub struct Topics {
    pub topics: HashMap<String, pb::TopicBundle>,
}

impl Topics {
    pub fn new() -> Self {
        Topics {
            topics: HashMap::new()
        }
    }
    pub fn create_topic(&mut self, topic: &str, schema_id: i32) -> &mut pb::TopicBundle {
        let topic = self.topics.entry(topic.to_string()).or_insert(pb::TopicBundle::new(topic, schema_id, Vec::new()));
        topic
    }

    pub fn to_publish(self) -> Publish {
        let mut publish = Publish::new();
        for (_, topic) in self.topics {
            publish.topic_bundles.push(topic);
        }
        publish
    }
}