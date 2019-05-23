use super::generic::{
    create_u8x4x4, expand_bits, restore_data, transpose, u8x4x4, Ops, Permutation,
};
use bitreader::BitReader;

type PREstate = u8x4x4;

#[derive(Debug)]
pub struct PRESENT {
    pub round_keys: Vec<PREstate>,
}

impl PRESENT {
    pub fn new(key: &[u8]) -> PRESENT {
        let rounds = 32;
        let mut round_keys: Vec<PREstate> = vec![Default::default(); rounds];
        key_expansion(key.to_vec(), &mut round_keys);

        PRESENT {
            round_keys: round_keys,
        }
    }
}

fn key_expansion(key: Vec<u8>, round_keys: &mut [PREstate]) {
    let keysize = key.len() * 4;
    let rounds = round_keys.len();
    let mut k_register = expand_bits(&key);
    match keysize {
        80 => {
            for i in 0..rounds {
                round_keys[i] = create_u8x4x4(&restore_data(&k_register)[0..16]);
                k_register[..].rotate_left(61);
                let mut buffer = restore_data(&k_register);
                buffer[0] = SBOX[buffer[0] as usize];
                buffer[15] ^= (i + 1 >> 1) as u8;
                buffer[16] ^= (i + 1 << 3 & 0xf) as u8;
                k_register = expand_bits(&buffer);
            }
        }
        128 => {
            for i in 0..rounds {
                round_keys[i] = create_u8x4x4(&restore_data(&k_register)[0..16]);
                k_register[..].rotate_left(61);
                let mut buffer = restore_data(&k_register);
                buffer[0] = SBOX[buffer[0] as usize];
                buffer[1] = SBOX[buffer[1] as usize];
                buffer[0] ^= (i + 1 >> 1) as u8;
                buffer[1] ^= (i + 1 << 4 & 0xf) as u8;
                k_register = expand_bits(&buffer);
            }
        }
        _ => panic!("Key length {} is not valid!", keysize),
    }
}

static SBOX: [u8; 16] = [
    0xc, 0x5, 0x6, 0xb, 0x9, 0x0, 0xa, 0xd, 0x3, 0xe, 0xf, 0x8, 0x4, 0x7, 0x1, 0x2,
];
static RSBOX: [u8; 16] = [
    0x5, 0xe, 0xf, 0x8, 0xc, 0x1, 0x2, 0xd, 0xb, 0x4, 0x6, 0x3, 0x0, 0x7, 0x9, 0xa,
];

static PBOX: [u8; 64] = [
    0, 16, 32, 48, 1, 17, 33, 49, 2, 18, 34, 50, 3, 19, 35, 51, 4, 20, 36, 52, 5, 21, 37, 53, 6,
    22, 38, 54, 7, 23, 39, 55, 8, 24, 40, 56, 9, 25, 41, 57, 10, 26, 42, 58, 11, 27, 43, 59, 12,
    28, 44, 60, 13, 29, 45, 61, 14, 30, 46, 62, 15, 31, 47, 63,
];
static RPBOX: [u8; 64] = [
    0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 1, 5, 9, 13, 17, 21, 25, 29, 33,
    37, 41, 45, 49, 53, 57, 61, 2, 6, 10, 14, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58, 62, 3, 7,
    11, 15, 19, 23, 27, 31, 35, 39, 43, 47, 51, 55, 59, 63,
];
