use super::generic::{create_u8x16, create_u8x4x4, u8x4x4, Ops, Permutation};

type SKIstate = u8x4x4;

pub struct SKINNY {
    pub round_keys: Vec<SKIstate>,
    cell_size: u8,
    sbox: Vec<u8>,
    rsbox: Vec<u8>,
}

impl SKINNY {
    pub fn new(key: &[u8], cell_size: u8) -> SKINNY {
        let round_keys: Vec<SKIstate> = key_expansion(key.to_vec(), cell_size);

        let (sbox, rsbox) = if cell_size == 4 {
            (SBOX_4.to_vec(), RSBOX_4.to_vec())
        } else {
            (SBOX_8.to_vec(), RSBOX_8.to_vec())
        };

        SKINNY {
            round_keys,
            cell_size,
            sbox,
            rsbox,
        }
    }

    /// Encrypt a block.
    pub fn encrypt(&self, data: &[u8]) -> [u8; 16] {
        let mut state = create_u8x4x4(data);
        for i in 0..self.round_keys.len() {
            state = sub_cells(state, &self.sbox[..]);
            state = add_constants(state, i);
            state = add_round_tweakey(state, self.round_keys[i]);
            state = shift_rows(state);
            state = state.gmul(&MDS, self.cell_size);
        }
        create_u8x16(&state)
    }

    /// Decrypt a block.
    pub fn decrypt(&self, data: &[u8]) -> [u8; 16] {
        let mut state = create_u8x4x4(data);
        for i in (0..self.round_keys.len()).rev() {
            state = state.gmul(&RMDS, self.cell_size);
            state = inv_shift_rows(state);
            state = add_round_tweakey(state, self.round_keys[i]);
            state = add_constants(state, i);
            state = sub_cells(state, &self.rsbox[..]);
        }
        create_u8x16(&state)
    }

    // Fault injection
    pub fn with_sbox_byte(mut self, faulty_idx: usize, faulty_val: u8) -> Self {
        let mut sbox = self.sbox;
        sbox[faulty_idx] = faulty_val;
        self.sbox = sbox;
        self
    }

    pub fn with_rsbox_byte(mut self, faulty_idx: usize, faulty_val: u8) -> Self {
        let mut rsbox = self.rsbox;
        rsbox[faulty_idx] = faulty_val;
        self.rsbox = rsbox;
        self
    }
}

fn add_round_tweakey(state: SKIstate, round_key: SKIstate) -> SKIstate {
    state.xor(&round_key)
}

fn key_expansion(key: Vec<u8>, cell_size: u8) -> Vec<SKIstate> {
    let mut tks = key;
    let mut round_keys = vec![];
    let round = match (cell_size, tks.len() / 16) {
        (4, 1) => 32,
        (4, 2) => 36,
        (4, 3) => 40,
        (8, 1) => 40,
        (8, 2) => 48,
        (8, 3) => 56,
        _ => panic!(""),
    };
    for _ in 0..round {
        let round_key = tks
            .chunks(16)
            .map(|x| create_u8x4x4(x))
            .fold([[0; 4]; 4], |res, i| res.xor(&i))
            .and(&[[0xff; 4], [0xff; 4], [0x00; 4], [0x00; 4]]);
        round_keys.push(round_key);
        for (i, tk) in tks.chunks_mut(16).enumerate() {
            let tmp: Vec<u8> = tk
                .iter()
                .enumerate()
                .map(|(j, _)| {
                    let cell = tk[PBOX[j] as usize];
                    if j >= 8 {
                        return cell;
                    }
                    match (i, cell_size) {
                        (1, 4) => {
                            ((cell << 1) & 0x0f)
                                ^ ((cell >> 2) & 0b00000001)
                                ^ ((cell >> 3) & 0b00000001)
                        }
                        (2, 4) => (cell >> 1) ^ ((cell << 3) & 0b00001000) ^ (cell & 0b00001000),
                        (1, 8) => cell.rotate_left(1) ^ (0x01 & cell >> 5),
                        (2, 8) => cell.rotate_right(1) ^ (0x80 & cell << 1),
                        _ => cell,
                    }
                })
                .collect();
            for (j, byte) in tk.iter_mut().enumerate() {
                *byte = tmp[j];
            }
        }
    }
    round_keys
}

fn sub_cells(state: SKIstate, sbox: &[u8]) -> SKIstate {
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

pub static SBOX_4: [u8; 16] = [
    0xc, 0x6, 0x9, 0x0, 0x1, 0xa, 0x2, 0xb, 0x3, 0x8, 0x5, 0xd, 0x4, 0xe, 0x7, 0xf,
];

pub static RSBOX_4: [u8; 16] = [
    0x3, 0x4, 0x6, 0x8, 0xc, 0xa, 0x1, 0xe, 0x9, 0x2, 0x5, 0x7, 0x0, 0xb, 0xd, 0xf,
];

pub const SBOX_8: [u8; 256] = [
    0x65, 0x4c, 0x6a, 0x42, 0x4b, 0x63, 0x43, 0x6b, 0x55, 0x75, 0x5a, 0x7a, 0x53, 0x73, 0x5b, 0x7b,
    0x35, 0x8c, 0x3a, 0x81, 0x89, 0x33, 0x80, 0x3b, 0x95, 0x25, 0x98, 0x2a, 0x90, 0x23, 0x99, 0x2b,
    0xe5, 0xcc, 0xe8, 0xc1, 0xc9, 0xe0, 0xc0, 0xe9, 0xd5, 0xf5, 0xd8, 0xf8, 0xd0, 0xf0, 0xd9, 0xf9,
    0xa5, 0x1c, 0xa8, 0x12, 0x1b, 0xa0, 0x13, 0xa9, 0x05, 0xb5, 0x0a, 0xb8, 0x03, 0xb0, 0x0b, 0xb9,
    0x32, 0x88, 0x3c, 0x85, 0x8d, 0x34, 0x84, 0x3d, 0x91, 0x22, 0x9c, 0x2c, 0x94, 0x24, 0x9d, 0x2d,
    0x62, 0x4a, 0x6c, 0x45, 0x4d, 0x64, 0x44, 0x6d, 0x52, 0x72, 0x5c, 0x7c, 0x54, 0x74, 0x5d, 0x7d,
    0xa1, 0x1a, 0xac, 0x15, 0x1d, 0xa4, 0x14, 0xad, 0x02, 0xb1, 0x0c, 0xbc, 0x04, 0xb4, 0x0d, 0xbd,
    0xe1, 0xc8, 0xec, 0xc5, 0xcd, 0xe4, 0xc4, 0xed, 0xd1, 0xf1, 0xdc, 0xfc, 0xd4, 0xf4, 0xdd, 0xfd,
    0x36, 0x8e, 0x38, 0x82, 0x8b, 0x30, 0x83, 0x39, 0x96, 0x26, 0x9a, 0x28, 0x93, 0x20, 0x9b, 0x29,
    0x66, 0x4e, 0x68, 0x41, 0x49, 0x60, 0x40, 0x69, 0x56, 0x76, 0x58, 0x78, 0x50, 0x70, 0x59, 0x79,
    0xa6, 0x1e, 0xaa, 0x11, 0x19, 0xa3, 0x10, 0xab, 0x06, 0xb6, 0x08, 0xba, 0x00, 0xb3, 0x09, 0xbb,
    0xe6, 0xce, 0xea, 0xc2, 0xcb, 0xe3, 0xc3, 0xeb, 0xd6, 0xf6, 0xda, 0xfa, 0xd3, 0xf3, 0xdb, 0xfb,
    0x31, 0x8a, 0x3e, 0x86, 0x8f, 0x37, 0x87, 0x3f, 0x92, 0x21, 0x9e, 0x2e, 0x97, 0x27, 0x9f, 0x2f,
    0x61, 0x48, 0x6e, 0x46, 0x4f, 0x67, 0x47, 0x6f, 0x51, 0x71, 0x5e, 0x7e, 0x57, 0x77, 0x5f, 0x7f,
    0xa2, 0x18, 0xae, 0x16, 0x1f, 0xa7, 0x17, 0xaf, 0x01, 0xb2, 0x0e, 0xbe, 0x07, 0xb7, 0x0f, 0xbf,
    0xe2, 0xca, 0xee, 0xc6, 0xcf, 0xe7, 0xc7, 0xef, 0xd2, 0xf2, 0xde, 0xfe, 0xd7, 0xf7, 0xdf, 0xff,
];

pub const RSBOX_8: [u8; 256] = [
    0xac, 0xe8, 0x68, 0x3c, 0x6c, 0x38, 0xa8, 0xec, 0xaa, 0xae, 0x3a, 0x3e, 0x6a, 0x6e, 0xea, 0xee,
    0xa6, 0xa3, 0x33, 0x36, 0x66, 0x63, 0xe3, 0xe6, 0xe1, 0xa4, 0x61, 0x34, 0x31, 0x64, 0xa1, 0xe4,
    0x8d, 0xc9, 0x49, 0x1d, 0x4d, 0x19, 0x89, 0xcd, 0x8b, 0x8f, 0x1b, 0x1f, 0x4b, 0x4f, 0xcb, 0xcf,
    0x85, 0xc0, 0x40, 0x15, 0x45, 0x10, 0x80, 0xc5, 0x82, 0x87, 0x12, 0x17, 0x42, 0x47, 0xc2, 0xc7,
    0x96, 0x93, 0x03, 0x06, 0x56, 0x53, 0xd3, 0xd6, 0xd1, 0x94, 0x51, 0x04, 0x01, 0x54, 0x91, 0xd4,
    0x9c, 0xd8, 0x58, 0x0c, 0x5c, 0x08, 0x98, 0xdc, 0x9a, 0x9e, 0x0a, 0x0e, 0x5a, 0x5e, 0xda, 0xde,
    0x95, 0xd0, 0x50, 0x05, 0x55, 0x00, 0x90, 0xd5, 0x92, 0x97, 0x02, 0x07, 0x52, 0x57, 0xd2, 0xd7,
    0x9d, 0xd9, 0x59, 0x0d, 0x5d, 0x09, 0x99, 0xdd, 0x9b, 0x9f, 0x0b, 0x0f, 0x5b, 0x5f, 0xdb, 0xdf,
    0x16, 0x13, 0x83, 0x86, 0x46, 0x43, 0xc3, 0xc6, 0x41, 0x14, 0xc1, 0x84, 0x11, 0x44, 0x81, 0xc4,
    0x1c, 0x48, 0xc8, 0x8c, 0x4c, 0x18, 0x88, 0xcc, 0x1a, 0x1e, 0x8a, 0x8e, 0x4a, 0x4e, 0xca, 0xce,
    0x35, 0x60, 0xe0, 0xa5, 0x65, 0x30, 0xa0, 0xe5, 0x32, 0x37, 0xa2, 0xa7, 0x62, 0x67, 0xe2, 0xe7,
    0x3d, 0x69, 0xe9, 0xad, 0x6d, 0x39, 0xa9, 0xed, 0x3b, 0x3f, 0xab, 0xaf, 0x6b, 0x6f, 0xeb, 0xef,
    0x26, 0x23, 0xb3, 0xb6, 0x76, 0x73, 0xf3, 0xf6, 0x71, 0x24, 0xf1, 0xb4, 0x21, 0x74, 0xb1, 0xf4,
    0x2c, 0x78, 0xf8, 0xbc, 0x7c, 0x28, 0xb8, 0xfc, 0x2a, 0x2e, 0xba, 0xbe, 0x7a, 0x7e, 0xfa, 0xfe,
    0x25, 0x70, 0xf0, 0xb5, 0x75, 0x20, 0xb0, 0xf5, 0x22, 0x27, 0xb2, 0xb7, 0x72, 0x77, 0xf2, 0xf7,
    0x2d, 0x79, 0xf9, 0xbd, 0x7d, 0x29, 0xb9, 0xfd, 0x2b, 0x2f, 0xbb, 0xbf, 0x7b, 0x7f, 0xfb, 0xff,
];

pub static PBOX: [u8; 16] = [9, 15, 8, 13, 10, 14, 12, 11, 0, 1, 2, 3, 4, 5, 6, 7];

pub static RCON: [u8; 62] = [
    0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3E, 0x3D, 0x3B, 0x37, 0x2F, 0x1E, 0x3C, 0x39, 0x33, 0x27, 0x0E,
    0x1D, 0x3A, 0x35, 0x2B, 0x16, 0x2C, 0x18, 0x30, 0x21, 0x02, 0x05, 0x0B, 0x17, 0x2E, 0x1C, 0x38,
    0x31, 0x23, 0x06, 0x0D, 0x1B, 0x36, 0x2D, 0x1A, 0x34, 0x29, 0x12, 0x24, 0x08, 0x11, 0x22, 0x04,
    0x09, 0x13, 0x26, 0x0C, 0x19, 0x32, 0x25, 0x0A, 0x15, 0x2A, 0x14, 0x28, 0x10, 0x20,
];

pub static MDS: [[u8; 4]; 4] = [[1, 0, 1, 1], [1, 0, 0, 0], [0, 1, 1, 0], [1, 0, 1, 0]];
pub static RMDS: [[u8; 4]; 4] = [[0, 1, 0, 0], [0, 1, 1, 1], [0, 1, 0, 1], [1, 0, 0, 1]];
