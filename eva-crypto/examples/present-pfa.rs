#![allow(unused_variables, dead_code, unused_must_use)]
extern crate rand;

use eva_crypto::generic::*;
use eva_crypto::present::*;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let key = [9u8; 20];
    let cipher = PRESENT::new(&key).with_sbox_byte(0, 0xd);

    // generate_data(key);

    let mut fc = File::open("examples/data/present/out.bin")?;
    let count = 120;

    println!("Target: {:x?}", cipher.round_keys[31]);

    // Pbox layer constants
    let mut table = [[0u8, 0u8, 0u8, 0u8]; 32];
    for bit in (0..64).step_by(4) {
        table[bit / 4] = [
            PBOX[bit + 0 as usize],
            PBOX[bit + 1 as usize],
            PBOX[bit + 2 as usize],
            PBOX[bit + 3 as usize],
        ];
    }

    let mut stats = [[0; 16]; 16];

    for _ in 0..count {
        let mut ciphertext = [0u8; 16];
        fc.read(&mut ciphertext)?;

        let c_bits = expand_bits(&ciphertext.to_vec(), 4);
        let data = restore_data(&pbox(c_bits, &RPBOX), 4);
        for i in 0..16 {
            stats[i][(data[i] ^ 0xc) as usize] += 1;
        }
    }

    let mut kc = [0u8; 16];

    let mut size = 1.0f32;
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

    println!(
        "Result: {:x?}",
        transpose(&create_u8x4x4(
            &restore_data(&(pbox(expand_bits(&kc.to_vec(), 4), &PBOX)), 4)[0..16]
        ))
    );
    println!("Residue Entropy: {}", size.log2());

    Ok(())
}

fn pbox(input: Vec<bool>, pbox: &[u8; 64]) -> Vec<bool> {
    input
        .iter()
        .enumerate()
        .map(|(i, _)| input[pbox[i] as usize])
        .collect()
}

fn generate_data(key: [u8; 20]) -> io::Result<()> {
    let cipher = PRESENT::new(&key).with_sbox_byte(0, 0xd);

    let mut fp = File::create("examples/data/present/msg.bin")?;
    let mut fc = File::create("examples/data/present/out.bin")?;
    let count = 300;

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
