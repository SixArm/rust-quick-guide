use prost::Message;

// Define a simple protobuf message
#[derive(Clone, PartialEq, Message)]
pub struct MyMessage {
    #[prost(int32, tag="1")]
    pub my_field: i32,
}

fn main() {
    // Serialize the message to bytes
    let message = MyMessage { my_field: 42 };
    let mut bytes = Vec::new();
    message.encode(&mut bytes).unwrap();

    // Deserialize the message from bytes
    let decoded = MyMessage::decode(bytes.as_slice()).unwrap();

    // Verify the decoded message is the same as the original
    assert_eq!(message, decoded);
    println!("Success")
}
