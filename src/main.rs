use hex;
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tiny_keccak::{Hasher, Keccak};

const FACTORY_ADDRESS: &str = "0x000000f2529cafe47f13bc4d674e343a97a870c1"; 
const INIT_CODE_HASH: &str = "YOUR_INIT_CODE_HASH";


#[derive(Clone)]
struct IntermediateResult {
    address: String,
    salt: String,
    zeros: usize,
}

fn compute_create2_address(factory: &[u8], salt: &[u8], init_code_hash: &[u8]) -> String {
    let mut preimage = vec![0xff];
    preimage.extend_from_slice(factory);
    preimage.extend_from_slice(salt);
    preimage.extend_from_slice(init_code_hash);

    let mut keccak = Keccak::v256();
    keccak.update(&preimage);
    let mut hash = [0u8; 32];
    keccak.finalize(&mut hash);
    format!("0x{}", hex::encode(&hash[12..]))
}

fn count_leading_zeros(address: &str) -> usize {
    address[2..]
        .chars()
        .take_while(|&c| c == '0')
        .count()
}

fn generate_vanity_create2_address(
    target_zeros: usize,
    batch_size: usize,
) -> (String, String) {
    let factory_bytes = hex::decode(&FACTORY_ADDRESS[2..]).expect("Invalid factory address");
    let init_code_hash = hex::decode(&INIT_CODE_HASH[2..]).expect("Invalid init code hash");
    let counter = Arc::new(AtomicUsize::new(0));
    let found_intermediate = Arc::new(Mutex::new(Vec::<IntermediateResult>::new()));
    let start_time = Instant::now();

    let counter_clone = Arc::clone(&counter);
    let found_clone = Arc::clone(&found_intermediate);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));
            let count = counter_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64();
            let rate = if elapsed > 0.0 { count as f64 / elapsed } else { 0.0 };
            let found = found_clone.lock().unwrap();
            print!(
                "\rAddresses tried: {}, Speed: {:.2} addr/s, Found 8 zeros: {}",
                count,
                rate,
                found.len()
            );
            io::stdout().flush().unwrap();
            if count >= usize::MAX - 1000 {
                break;
            }
        }
    });

    loop {
        let results: Vec<_> = (0..batch_size)
            .into_par_iter()
            .filter_map(|_| {
                let mut salt = [0u8; 32];
                for byte in &mut salt {
                    *byte = rand::random();
                }
                let salt_hex = format!("0x{}", hex::encode(&salt));
                counter.fetch_add(1, Ordering::Relaxed);

                let contract_address = compute_create2_address(&factory_bytes, &salt, &init_code_hash);
                let zeros = count_leading_zeros(&contract_address);

                if zeros >= target_zeros {
                    let mut found = found_intermediate.lock().unwrap();
                    found.push(IntermediateResult {
                        address: contract_address.clone(),
                        salt: salt_hex.clone(),
                        zeros,
                    });
                    println!(
                        "\nFound {} zero address: {} (Salt: {})",
                        zeros, contract_address, salt_hex
                    );
                    return Some((contract_address, salt_hex));
                }
                None
            })
            .collect();

        if let Some((contract_address, salt)) = results.into_iter().next() {
            let final_count = counter.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64();
            println!("\n\nFound target {} zero address after {} attempts", target_zeros, final_count);
            println!("Contract Address: {}", contract_address);
            println!("Salt: {}", salt);
            println!("Time taken: {:.2} seconds", elapsed);
            return (contract_address, salt);
        }
    }
}

fn main() {
    // Your want generate zero
    let target_zeros = 10; 
    let batch_size = num_cpus::get() * 1000;

    println!("Starting vanity address generation with Gotchiopus Create2 Factory...");
    println!("Target: {} leading zeros", target_zeros);
    println!("Factory Address: {}", FACTORY_ADDRESS);
    println!("Batch size: {}", batch_size);
    println!("CPU cores: {}", num_cpus::get());

    let (contract_address, salt) = generate_vanity_create2_address(target_zeros, batch_size);

    println!("\nFinal Summary:");
    println!("Contract Address: {}", contract_address);
    println!("Salt: {}", salt);
}