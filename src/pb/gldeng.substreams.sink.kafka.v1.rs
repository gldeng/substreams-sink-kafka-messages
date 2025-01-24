// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Batch {
    #[prost(message, repeated, tag="1")]
    pub topic_bundles: ::prost::alloc::vec::Vec<TopicBundle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicBundle {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub schema_id: i32,
    #[prost(message, repeated, tag="3")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
