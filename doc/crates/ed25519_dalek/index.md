# ed25519_dalek crate

<https://crates.io/crates/ed25519_dalek>

The Rust ed25519_dalek crate provides cryptographic message signing, verifying,
and related techniques to ensure the authenticity and integrity of a message. 

Ed25519 is a type of elliptic curve cryptography (ECC) that uses the EdDSA
(Edwards-curve Digital Signature Algorithm) signature scheme, and named after
the prime field of order 2^255-19, which is the field over which the elliptic
curve is defined.

Ed25519 was designed to be a fast, secure, and efficient signature algorithm for
use in a variety of cryptographic applications, including key agreement,
encryption, and authentication. It was developed by Daniel J. Bernstein, Niels
Duif, Tanja Lange, Peter Schwabe, and Bo-Yin Yang.

The crate creates a digital signature, which is a mathematical algorithm that
creates a unique code that represents the contents of the message. The digital
signature is then attached to the message, indicating that the message has not
been altered or tampered with, and that it originated from a particular sender.

When the recipient receives the message, they can use the sender's public key to
decrypt the digital signature, and verify that the message is not altered or corrupted,
and that the message originated from the sender who possesses the private key.

Cryptographic message signing is widely used in electronic communication to
ensure the authenticity and integrity of messages, including email, digital
documents, and online transactions. It provides a way for parties to verify the
identity of each other and ensure that the information exchanged is trustworthy.
