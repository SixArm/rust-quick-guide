# ed25519_dalek crate - example

[Runnable project](/projects/crates/ed25519-dalek/signing_and_verifying)

Example:

```rust
use rand::rngs::OsRng;

use ed25519_dalek::{
    Signer, SigningKey, Signature, 
    Verifier, VerifyingKey
};

pub fn main() {
    // Create a message that we will sign.
    let message: &[u8] = b"Hello, World!";

    // Load a random number generator (RNG).
    let mut rng = OsRng{};
    
    // Generate a signing key a.k.a. a keypair
    let signing_key: SigningKey = 
        SigningKey::generate(&mut rng);

    // Sign the message
    let signature: Signature = signing_key.sign(message);
    
    // Get the verifying key a.k.a. the keypair private key.
    let verifying_key: VerifyingKey = 
        signing_key.verifying_key();

    // Verify the signature.
    assert!(verifying_key.verify(message, &signature).is_ok());
}
```
