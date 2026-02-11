use std::fs::File;
use std::io::{BufRead, BufReader};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use pqcrypto_dilithium::dilithium5::{PublicKey};
use pqcrypto_traits::sign::PublicKey as PQPublicTrait;

use crate::security::verify;

pub fn verify_receipts() {

    let path = "keys/arbitration_receipts.jsonl";

    if !std::path::Path::new(path).exists() {
        println!("No receipts file found.");
        return;
    }

    // Load arbiter public key
    let pub_bytes = std::fs::read("keys/arbiter-node.pub")
        .expect("arbiter public key missing");

    let public: PublicKey =
        PQPublicTrait::from_bytes(&pub_bytes)
            .expect("invalid arbiter public key");

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut valid = 0;
    let mut invalid = 0;

    for line in reader.lines() {
        if let Ok(txt) = line {

            let value: serde_json::Value =
                serde_json::from_str(&txt).unwrap();

            let signature_b64 =
                value["signature"].as_str().unwrap();

            let sig_bytes =
                STANDARD.decode(signature_b64).unwrap();

            // Reconstruct signed payload (exclude signature)
            let payload = serde_json::json!({
                "device_id": value["device_id"],
                "sequence": value["sequence"],
                "authority_id": value["authority_id"],
                "role": value["role"],
                "timestamp": value["timestamp"],
                "outcome": value["outcome"]
            });

            let canonical =
                serde_json::to_vec(&payload).unwrap();

            let ok = verify::verify_signature(
                &public,
                &canonical,
                &sig_bytes,
            );

            if ok {
                println!("✅ VALID receipt for device={} seq={}",
                    value["device_id"],
                    value["sequence"]
                );
                valid += 1;
            } else {
                println!("❌ INVALID receipt detected!");
                invalid += 1;
            }
        }
    }

    println!("\n📊 Receipt Verification Summary");
    println!("Valid: {}", valid);
    println!("Invalid: {}", invalid);
}
