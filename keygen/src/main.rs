use p256::ecdsa::{SigningKey, Signature};
use p256::ecdsa::signature::Signer;
use rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use sha2::{Digest, Sha256};

fn main() {
    // Use a fixed seed for reproducible keys
    let seed = [0u8; 32];
    let mut rng = ChaCha8Rng::from_seed(seed);
    let signing_key = SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key();

    // Kernel data (matches flash.rs)
    let mut kernel_data = [0u8; 4096];
    for i in 0..4096 {
        kernel_data[i] = (i % 256) as u8;
    }
    let mut hasher = Sha256::new();
    hasher.update(&kernel_data);
    let kernel_hash = hasher.finalize();
    let kernel_signature: Signature = signing_key.sign(&kernel_hash);

    // Update data (matches tftp.rs)
    let update_data = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let mut hasher = Sha256::new();
    hasher.update(&update_data);
    let update_hash = hasher.finalize();
    let update_signature: Signature = signing_key.sign(&update_hash);

    // Print keys and signatures in Rust array format
    println!("pub const PUBLIC_KEY: [u8; 65] = {:?};", public_key.to_encoded_point(false).as_bytes());
    println!("pub const KERNEL_SIGNATURE: [u8; {}] = {:?};", kernel_signature.to_der().len(), kernel_signature.to_der().as_bytes());
    println!("pub const UPDATE_SIGNATURE: [u8; {}] = {:?};", update_signature.to_der().len(), update_signature.to_der().as_bytes());
}
