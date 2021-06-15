#![allow(unused_variables, dead_code, unused_must_use, unused_imports)]
extern crate rand;

use eva_crypto::generic::*;
use eva_crypto::led::*;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
    let cipher = LED::new(&key).with_sbox_byte(0, 0xd);

    // generate_data(key);

    let mut fc = File::open("examples/data/led/out.bin")?;
    let count = 80;

    let mut stats = [[0; 16]; 16];

    for _ in 0..count {
        let mut buffer = [0u8; 16];
        fc.read(&mut buffer);

        let state = transpose(&create_u8x4x4(&buffer));
        for x in 0..4 {
            for y in 0..4 {
                let e = state[x]
                    .gmul(&RMDS[y], 4)
                    .iter()
                    .fold(0x00, |res, i| res ^ i);
                stats[x * 4 + y as usize][e as usize] += 1;
            }
        }
    }

    let mut kc = [0u8; 16];
    let mut size = 1.0f32;
    let fault_state = create_u8x4x4(&cipher.key[0..16]).gmul(&RMDS, 4);
    let faults = create_u8x4x4(&[0xcu8; 16]);

    for i in 0..16 {
        let mut tmp = 0;
        for j in 0..16 {
            if stats[i][j] == 0 {
                kc[i] = j as u8;
                tmp += 1;
            }
        }
        size *= tmp as f32;
    }

    println!("Target: {:?}", create_u8x4x4(&cipher.key));
    println!(
        "Result: {:?}",
        transpose(&create_u8x4x4(&kc)).xor(&faults).gmul(&MDS, 4)
    );
    println!("Residue Entropy: {}", size.log2());

    Ok(())
}

fn generate_data(key: [u8; 16]) -> io::Result<()> {
    let cipher = LED::new(&key).with_sbox_byte(0, 0xd);

    let mut fp = File::create("examples/data/led/msg.bin")?;
    let mut fc = File::create("examples/data/led/out.bin")?;
    let count = 10000;

    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0..16);

    for _ in 0..count {
        let mut plaintext = [0u8; 16];
        for i in 0..16 {
            plaintext[i] = dist.sample(&mut rng);
        }
        let ciphertext = cipher.encrypt(&plaintext);
        fp.write(&plaintext)?;
        fc.write(&ciphertext)?;
    }

    Ok(())
}
