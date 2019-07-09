#![allow(unused_variables, dead_code, unused_must_use)]
extern crate rand;

use eva_crypto::aes::*;
use eva_crypto::generic::*;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let key: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ];
    let cipher = AES::new(&key).with_sbox_byte(0, 0xd);

    // generate_data(key);

    let mut buffer = [0u8; 16];
    let mut f = File::open("examples/data/aes/out.bin")?;
    let mut stats = [[0u32; 256]; 16];
    let count = 2000;

    for _ in 0..count {
        f.read(&mut buffer)?;
        for i in 0..16 {
            stats[i as usize][buffer[i] as usize] += 1;
        }
    }

    // Round 10 Analysis
    let mut kc = [0u8; 16];
    let mut size = 1.0f32;
    for i in 0..16 {
        let mut tmp = 0;
        for byte in 0..0xff {
            if stats[i as usize][byte] == 0 {
                kc[i] = byte as u8 ^ 0x63u8;
                tmp += 1;
            }
        }
        size *= tmp as f32;
    }

    println!("Target: {:x?}", cipher.round_keys[10]);
    println!("Result: {:x?}", create_u8x4x4(&kc));
    println!("Residue Entropy: {}", size.log2());

    Ok(())
}

fn generate_data(key: [u8; 16]) -> io::Result<()> {
    let cipher = AES::new(&key).with_sbox_byte(0, 0xd);

    let mut fp = File::create("examples/data/aes/msg.bin")?;
    let mut fc = File::create("examples/data/aes/out.bin")?;
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
