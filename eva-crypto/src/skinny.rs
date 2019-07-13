use super::generic::{create_u8x16, create_u8x4x4, u8x4x4, Ops, Permutation};

type SKIstate = u8x4x4;

pub struct SKINNY {
    pub round_keys: Vec<SKIstate>,
    sbox: [u8; 16],
    rsbox: [u8; 16],
}

impl SKINNY {
    pub fn new(key: &[u8]) -> SKINNY {
        let rounds = 62;
        let mut round_keys: Vec<SKIstate> = vec![Default::default(); rounds];
        key_expansion(key.to_vec(), &mut round_keys);

        SKINNY {
            round_keys,
            sbox: SBOX,
            rsbox: RSBOX,
        }
    }

    /// Encrypt a block.
    pub fn encrypt(&self, data: &[u8]) -> [u8; 16] {
        let rounds = 32;
        let mut state = create_u8x4x4(data);
        for i in 0..rounds {
            state = sub_cells(state, &self.sbox);
            state = add_constants(state, i);
            state = add_round_tweakey(state, self.round_keys[i]);
            state = shift_rows(state);
            state = state.gmul(&MDS, 4);
        }
        create_u8x16(&state)
    }

    /// Decrypt a block.
    pub fn decrypt(&self, data: &[u8]) -> [u8; 16] {
        let rounds = 32;
        let mut state = create_u8x4x4(data);
        for i in (0..rounds).rev() {
            state = state.gmul(&RMDS, 4);
            state = inv_shift_rows(state);
            state = add_round_tweakey(state, self.round_keys[i]);
            state = add_constants(state, i);
            state = sub_cells(state, &self.rsbox);
        }
        create_u8x16(&state)
    }

    // Fault injection
    pub fn with_sbox_byte(mut self, faulty_idx: usize, faulty_val: u8) -> Self {
        let mut sbox = SBOX;
        sbox[faulty_idx] = faulty_val;
        self.sbox = sbox;
        self
    }

    pub fn with_rsbox_byte(mut self, faulty_idx: usize, faulty_val: u8) -> Self {
        let mut rsbox = RSBOX;
        rsbox[faulty_idx] = faulty_val;
        self.sbox = rsbox;
        self
    }
}

fn add_round_tweakey(state: SKIstate, round_key: SKIstate) -> SKIstate {
    state.xor(&round_key)
}

fn key_expansion(key: Vec<u8>, round_keys: &mut [SKIstate]) {
    let mut tk = key;
    for i in 0..32 {
        let mut round_key = create_u8x4x4(&tk[0..16]);
        round_key[2] = [0; 4];
        round_key[3] = [0; 4];
        round_keys[i] = round_key;
        tk = tk
            .iter()
            .enumerate()
            .map(|(i, _)| tk[PBOX[i] as usize])
            .collect();
    }
}

fn sub_cells(state: SKIstate, sbox: &[u8; 16]) -> SKIstate {
    state.sub_sbox(sbox)
}

fn shift_rows(state: SKIstate) -> SKIstate {
    state.rrot()
}

fn inv_shift_rows(state: SKIstate) -> SKIstate {
    state.lrot()
}

fn add_constants(state: SKIstate, round: usize) -> SKIstate {
    state.xor(&[
        [RCON[round] & 0x0f, 0, 0, 0],
        [(RCON[round] & 0xf0) >> 4, 0, 0, 0],
        [0x2, 0, 0, 0],
        [0; 4],
    ])
}

pub static SBOX: [u8; 16] = [
    0xc, 0x6, 0x9, 0x0, 0x1, 0xa, 0x2, 0xb, 0x3, 0x8, 0x5, 0xd, 0x4, 0xe, 0x7, 0xf,
];

pub static RSBOX: [u8; 16] = [
    0x3, 0x4, 0x6, 0x8, 0xc, 0xa, 0x1, 0xe, 0x9, 0x2, 0x5, 0x7, 0x0, 0xb, 0xd, 0xf,
];

pub static PBOX: [u8; 16] = [9, 15, 8, 13, 10, 14, 12, 11, 0, 1, 2, 3, 4, 5, 6, 7];

pub static RCON: [u8; 62] = [
    0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3E, 0x3D, 0x3B, 0x37, 0x2F, 0x1E, 0x3C, 0x39, 0x33, 0x27, 0x0E,
    0x1D, 0x3A, 0x35, 0x2B, 0x16, 0x2C, 0x18, 0x30, 0x21, 0x02, 0x05, 0x0B, 0x17, 0x2E, 0x1C, 0x38,
    0x31, 0x23, 0x06, 0x0D, 0x1B, 0x36, 0x2D, 0x1A, 0x34, 0x29, 0x12, 0x24, 0x08, 0x11, 0x22, 0x04,
    0x09, 0x13, 0x26, 0x0C, 0x19, 0x32, 0x25, 0x0A, 0x15, 0x2A, 0x14, 0x28, 0x10, 0x20,
];

static MDS: [[u8; 4]; 4] = [[1, 0, 1, 1], [1, 0, 0, 0], [0, 1, 1, 0], [1, 0, 1, 0]];
static RMDS: [[u8; 4]; 4] = [[0, 1, 0, 0], [0, 1, 1, 1], [0, 1, 0, 1], [1, 0, 0, 1]];
