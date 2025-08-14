use p256::ecdsa::{VerifyingKey, Signature, signature::Verifier};
use crate::println;

pub fn verify_signature(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    println!("Verifying signature for data len={}", data.len());
    match VerifyingKey::from_sec1_bytes(public_key) {
        Ok(verifying_key) => match Signature::from_der(signature) {
            Ok(sig) => {
                verifying_key.verify(data, &sig).is_ok()
            }
            Err(e) => {
                println!("Signature parsing error: {:?}", e);
                false
            }
        },
        Err(e) => {
            println!("Public key parsing error: {:?}", e);
            false
        }
    }
}
