# prost crate - example

[Runnable project](/projects/crates/prost/serialize_deserialize_protobuf)

Example code to serialize and deserialize a simple protobuf message:

```rust
use prost::Message;

// Define a simple protobuf message
#[derive(Clone, PartialEq, Message)]
pub struct MyMessage {
    #[prost(int32, tag="1")]
    pub my_field: i32,
}

// Serialize the message to bytes
let message = MyMessage { my_field: 42 };
let mut bytes = Vec::new();
message.encode(&mut bytes).unwrap();

// Deserialize the message from bytes
let decoded = MyMessage::decode(bytes.as_slice()).unwrap();

// Verify the decoded message is the same as the original
assert_eq!(message, decoded);
```

In this example code, we define a simple protobuf message `MyMessage` with a single field `my_field`. We use Prost's `#[derive(Message)]` macro to generate the serialization and deserialization code for this message.

We then create an instance of `MyMessage`, serialize it to bytes using the `encode()` method, and deserialize it back to a `MyMessage` instance using the `decode()` method.

Finally, we verify that the serialized and deserialized messages are equal using the `assert_eq()` macro.
