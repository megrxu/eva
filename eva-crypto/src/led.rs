use super::generic::{u8x4x4, Ops, Permutation};

pub struct LED {
    key: Vec<u8>,
    ns: u8,
    keysize: u8,
}

type LEDstate = u8x4x4;

impl LED {
    pub fn new(key: &[u8]) -> Self {
        let keysize = key.len() * 4;
        let ns = match keysize {
            64 => 8,
            80 => 10,
            128 => 12,
            _ => unimplemented!(),
        };
        LED {
            key: key.to_vec(),
            ns: ns,
            keysize: keysize as u8,
        }
    }

    pub fn encrypt(self, data: &[u8]) -> Vec<u8> {
        let mut state = create_state(data);
        for i in 0..self.ns {
            state = add_round_key(&state, &self.key, self.keysize, i);
            state = step(&state, self.keysize, i);
        }
        state = add_round_key(&state, &self.key, self.keysize, self.ns);
        state.concat()
    }

    pub fn decrypt(self, data: &[u8]) -> Vec<u8> {
        let mut state = create_state(data);
        for i in 1..self.ns + 1 {
            state = add_round_key(&state, &self.key, self.keysize, self.ns - i);
            state = inv_step(&state, self.keysize, self.ns - i);
        }
        state = add_round_key(&state, &self.key, self.keysize, 0);
        state.concat()
    }
}

fn create_state(input: &[u8]) -> LEDstate {
    assert_eq!(input.len(), 16);
    let mut state = [[0; 4]; 4];
    for (i, &j) in input.iter().enumerate() {
        state[i / 4][i % 4] = j;
    }
    state
}
fn step(state: &LEDstate, keysize: u8, round: u8) -> LEDstate {
    let mut out = state.clone();
    for i in 0..4 {
        out = add_constants(&out, round * 4 + i, keysize);
        out = sub_cells(&out);
        out = shift_rows(&out);
        out = mix_columns_serial(&out);
    }
    out
}
fn inv_step(state: &LEDstate, keysize: u8, round: u8) -> LEDstate {
    let mut out = state.clone();
    for i in 0..4 {
        out = inv_mix_columns_serial(&out);
        out = inv_shift_rows(&out);
        out = inv_sub_cells(&out);
        out = add_constants(&out, round * 4 + (3 - i), keysize);
    }
    out
}
fn add_round_key(state: &LEDstate, key: &Vec<u8>, keysize: u8, round: u8) -> LEDstate {
    let mut out = state.clone();
    let mut rkey: LEDstate = Default::default();
    for i in 0..4 {
        for j in 0..4 {
            rkey[i][j] = key[((round * 16 + 4 * i as u8 + j as u8) % (keysize / 4)) as usize];
            out[i][j] ^= rkey[i][j];
        }
    }
    out
}
fn add_constants(state: &LEDstate, r: u8, keysize: u8) -> LEDstate {
    state.xor(&[
        [0 ^ (keysize >> 4), (RCON[r as usize] >> 3) & 0x07, 0, 0],
        [1 ^ ((keysize >> 4) & 0x0f), RCON[r as usize] & 0x07, 0, 0],
        [2 ^ (keysize & 0x0f), (RCON[r as usize] >> 3) & 0x07, 0, 0],
        [3 ^ (keysize & 0x0f), RCON[r as usize] & 0x07, 0, 0],
    ])
}
fn sub_cells(state: &LEDstate) -> LEDstate {
    state.sub_sbox(&SBOX)
}
fn inv_sub_cells(state: &LEDstate) -> LEDstate {
    state.sub_rsbox(&RSBOX)
}
fn shift_rows(state: &LEDstate) -> LEDstate {
    state.lrot()
}
fn inv_shift_rows(state: &LEDstate) -> LEDstate {
    state.rrot()
}
fn mix_columns_serial(state: &LEDstate) -> LEDstate {
    MDS.gmul(&state, 4)
}
fn inv_mix_columns_serial(state: &LEDstate) -> LEDstate {
    RMDS.gmul(&state, 4)
}

static RCON: [u8; 48] = [
    0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3E, 0x3D, 0x3B, 0x37, 0x2F, 0x1E, 0x3C, 0x39, 0x33, 0x27, 0x0E,
    0x1D, 0x3A, 0x35, 0x2B, 0x16, 0x2C, 0x18, 0x30, 0x21, 0x02, 0x05, 0x0B, 0x17, 0x2E, 0x1C, 0x38,
    0x31, 0x23, 0x06, 0x0D, 0x1B, 0x36, 0x2D, 0x1A, 0x34, 0x29, 0x12, 0x24, 0x08, 0x11, 0x22, 0x04,
];

static SBOX: [u8; 16] = [
    0xC, 0x5, 0x6, 0xB, 0x9, 0x0, 0xA, 0xD, 0x3, 0xE, 0xF, 0x8, 0x4, 0x7, 0x1, 0x2,
];

static RSBOX: [u8; 16] = [
    0x5, 0xe, 0xF, 0x8, 0xC, 0x1, 0x2, 0xD, 0xB, 0x4, 0x6, 0x3, 0x0, 0x7, 0x9, 0xA,
];

static MDS: [[u8; 4]; 4] = [
    [0x4, 0x1, 0x2, 0x2],
    [0x8, 0x6, 0x5, 0x6],
    [0xB, 0xE, 0xA, 0x9],
    [0x2, 0x2, 0xF, 0xB],
];

static RMDS: [[u8; 4]; 4] = [
    [0xc, 0xc, 0xd, 0x4],
    [0x3, 0x8, 0x4, 0x5],
    [0x7, 0x6, 0x2, 0xe],
    [0xd, 0x9, 0x9, 0xd],
];
