use super::generic::{
    create_u8x4x4, expand_bits, restore_data, transpose, u8x4x4, Ops, Permutation,
};

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

    /// Encrypt a block.
    pub fn encrypt(self, data: &[u8]) -> Vec<u8> {
        let rounds = 32;
        let mut state = create_u8x4x4(data);
        for i in 0..rounds - 1 {
            state = add_round_key(&state, &self.round_keys[i]);
            state = sbox_layer(&state);
            state = p_layer(&state, &PBOX);
        }
        state = add_round_key(&state, &self.round_keys[rounds - 1]);
        state.concat()
    }

    /// Decrypt a block.
    pub fn decrypt(self, data: &[u8]) -> Vec<u8> {
        let rounds = 32;
        let mut state = create_u8x4x4(data);
        for i in (1..rounds).rev() {
            state = add_round_key(&state, &self.round_keys[i]);
            state = p_layer(&state, &RPBOX);
            state = inv_sbox_layer(&state);
        }
        state = add_round_key(&state, &self.round_keys[0]);
        state.concat()
    }
}
fn add_round_key(state: &PREstate, round_key: &PREstate) -> PREstate {
    state.xor(&transpose(&round_key))
}
fn sbox_layer(state: &PREstate) -> PREstate {
    state.sub_sbox(&SBOX)
}
fn inv_sbox_layer(state: &PREstate) -> PREstate {
    state.sub_sbox(&RSBOX)
}
fn p_layer(state: &PREstate, pbox: &[u8]) -> PREstate {
    let bits = expand_bits(&state.concat().to_vec(), 4);
    let mut p_bits: Vec<bool> = vec![true; 64];
    for i in 0..64 {
        p_bits[i] = bits[pbox[i] as usize];
    }
    create_u8x4x4(&restore_data(&p_bits, 4)[0..16])
}
fn key_expansion(key: Vec<u8>, round_keys: &mut [PREstate]) {
    let keysize = key.len() * 4;
    let rounds = round_keys.len();
    let mut k_register = expand_bits(&key, 4);
    match keysize {
        80 | 128 => (),
        _ => panic!("Key length {} is not valid!", keysize),
    }

    for i in 0..rounds {
        round_keys[i] = create_u8x4x4(&restore_data(&k_register, 4)[0..16]);
        // rotate left 61 bits
        k_register[..].rotate_left(61);
        let mut buffer = restore_data(&k_register, 4);
        if keysize == 80 {
            // Sbox
            buffer[0] = SBOX[buffer[0] as usize];
            //  XOR with the round counter
            buffer[15] ^= (i + 1 >> 1) as u8;
            buffer[16] ^= (i + 1 << 3 & 0xf) as u8;
        } else {
            // Sbox
            buffer[0] = SBOX[buffer[0] as usize];
            buffer[1] = SBOX[buffer[1] as usize];
            //  XOR with the round counter
            buffer[15] ^= (i + 1 >> 2) as u8;
            buffer[16] ^= (i + 1 << 2 & 0xf) as u8;
        }
        k_register = expand_bits(&buffer, 4);
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
