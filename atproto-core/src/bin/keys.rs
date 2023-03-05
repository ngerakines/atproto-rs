
use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer},
    SecretKey,
};
use rand_core::OsRng; // requires 'getrandom' feature

// Signing
fn main() {
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    println!("{:?}", signing_key.to_bytes());
}
