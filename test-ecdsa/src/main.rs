use p256::ecdsa::{Signature, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use signature::{Signer, Verifier};
use rand_core::OsRng;

fn main() {
    // Test kernel: 4096 bytes of 0x42
    let test_kernel = [0x42; 4096];
    println!("Generating signature for test kernel...");
    let mut hasher = Sha256::new();
    hasher.update(&test_kernel);
    let hash = hasher.finalize();
    println!("Test kernel hash: {:?}", hash);

    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    match signing_key.try_sign(&hash) {
        Ok(signature) => {
            let signature_der = signature.to_der();
            println!("Generated signature: {:?}", signature_der.as_bytes());
            println!("Public key: {:?}", verifying_key.to_encoded_point(false).as_bytes());
            if verifying_key.verify(&hash, &signature).is_ok() {
                println!("Signature valid!");
            } else {
                println!("Signature invalid!");
            }
        }
        Err(e) => println!("Signature generation failed: {:?}", e),
    }
}