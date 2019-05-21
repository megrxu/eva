macro_rules! vector_type {
    ($t:tt, $e:tt, $col:expr) => {
        #[allow(non_camel_case_types)]
        pub type $t = [$e; $col];
    };
}

macro_rules! matrix_type {
    ($t:tt, $e:tt, $row:expr) => {
        #[allow(non_camel_case_types)]
        pub type $t = [$e; $row];
    };
}

vector_type!(u8x4, u8, 4);
vector_type!(u4x4, u8, 4);
matrix_type!(u8x4x4, u8x4, 4);
matrix_type!(u4x4x4, u4x4, 4);

pub trait Ops {
    fn lrot(&self) -> Self;
    fn rrot(&self) -> Self;
    fn xor(&self, rhs: &Self) -> Self;
    fn and(&self, rhs: &Self) -> Self;
    fn gmul(&self, rhs: &Self, bits: u8) -> Self;
}

pub trait Permutation {
    fn sub_sbox(&self, rsbox: &[u8]) -> Self;
    fn sub_rsbox(&self, rsbox: &[u8]) -> Self;
}

macro_rules! impl_matrix_ops {
    (for $($t:ty),+) => {
        $(impl Ops for $t {
            fn lrot(&self) -> Self {
                [
                    self[0],
                    self[1].lrot(),
                    self[2].lrot().lrot(),
                    self[3].lrot().lrot().lrot(),
                ]
            }

            fn rrot(&self) -> Self {
                [
                    self[0],
                    self[1].rrot(),
                    self[2].rrot().rrot(),
                    self[3].rrot().rrot().rrot(),
                ]
            }

            fn xor(&self, rhs: &Self) -> Self {
                [
                    self[0].xor(&rhs[0]),
                    self[1].xor(&rhs[1]),
                    self[2].xor(&rhs[2]),
                    self[3].xor(&rhs[3]),
                ]
            }
            fn and(&self, rhs: &Self) -> Self {
                [
                    self[0].and(&rhs[0]),
                    self[1].and(&rhs[1]),
                    self[2].and(&rhs[2]),
                    self[3].and(&rhs[3]),
                ]
            }
            fn gmul(&self, rhs: &Self, bits: u8) -> Self {
                [
                    rhs[0].gmul(&[self[0][0]; 4], bits),
                    rhs[0].gmul(&[self[1][0]; 4], bits),
                    rhs[0].gmul(&[self[2][0]; 4], bits),
                    rhs[0].gmul(&[self[3][0]; 4], bits),
                ]
                .xor(&[
                    rhs[1].gmul(&[self[0][1]; 4], bits),
                    rhs[1].gmul(&[self[1][1]; 4], bits),
                    rhs[1].gmul(&[self[2][1]; 4], bits),
                    rhs[1].gmul(&[self[3][1]; 4], bits),
                ])
                .xor(&[
                    rhs[2].gmul(&[self[0][2]; 4], bits),
                    rhs[2].gmul(&[self[1][2]; 4], bits),
                    rhs[2].gmul(&[self[2][2]; 4], bits),
                    rhs[2].gmul(&[self[3][2]; 4], bits),
                ])
                .xor(&[
                    rhs[3].gmul(&[self[0][3]; 4], bits),
                    rhs[3].gmul(&[self[1][3]; 4], bits),
                    rhs[3].gmul(&[self[2][3]; 4], bits),
                    rhs[3].gmul(&[self[3][3]; 4], bits),
                ])
            }
        })*
    };
}

macro_rules! impl_vector_ops {
    (for $($t:ty),+) => {
        $(
            impl Ops for $t {
    fn lrot(&self) -> Self {
        [self[1], self[2], self[3], self[0]]
    }
    fn rrot(&self) -> Self {
        [self[3], self[0], self[1], self[2]]
    }
    fn xor(&self, rhs: &Self) -> Self {
        [
            self[0] ^ rhs[0],
            self[1] ^ rhs[1],
            self[2] ^ rhs[2],
            self[3] ^ rhs[3],
        ]
    }
    fn and(&self, rhs: &Self) -> Self {
        [
            self[0] & rhs[0],
            self[1] & rhs[1],
            self[2] & rhs[2],
            self[3] & rhs[3],
        ]
    }
    fn gmul(&self, rhs: &Self, bits: u8) -> Self {
        [
            self[0].gmul(&rhs[0], bits),
            self[1].gmul(&rhs[1], bits),
            self[2].gmul(&rhs[2], bits),
            self[3].gmul(&rhs[3], bits),
        ]
    }
}
        )*
    };
}

impl_matrix_ops!(for u8x4x4);
impl_vector_ops!(for u8x4);

impl Ops for u8 {
    fn lrot(&self) -> Self {
        Default::default()
    }

    fn rrot(&self) -> Self {
        Default::default()
    }
    fn xor(&self, rhs: &Self) -> Self {
        self ^ rhs
    }
    fn and(&self, rhs: &Self) -> Self {
        self & rhs
    }
    fn gmul(&self, rhs: &Self, bits: u8) -> Self {
        match bits {
            8 => gmul_x(*self, *rhs, 0x1b, 8),
            4 => gmul_x(*self, *rhs, 0x03, 4),
            _ => unimplemented!(),
        }
    }
}

fn gmul_x(mut a: u8, mut b: u8, poly: u8, bits: u8) -> u8 {
    let mut p = 0;
    while a != 0 && b != 0 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi_bit_set = a & (1 << (bits - 1));
        a <<= 1;
        if hi_bit_set != 0 {
            a ^= poly;
        }
        b >>= 1;
    }
    p & (0xff >> (8 - bits))
}

impl Permutation for u8x4 {
    fn sub_sbox(&self, sbox: &[u8]) -> Self {
        [
            sbox[self[0] as usize],
            sbox[self[1] as usize],
            sbox[self[2] as usize],
            sbox[self[3] as usize],
        ]
    }
    fn sub_rsbox(&self, rsbox: &[u8]) -> Self {
        [
            rsbox[self[0] as usize],
            rsbox[self[1] as usize],
            rsbox[self[2] as usize],
            rsbox[self[3] as usize],
        ]
    }
}

impl Permutation for u8x4x4 {
    fn sub_sbox(&self, sbox: &[u8]) -> Self {
        [
            self[0].sub_sbox(sbox),
            self[1].sub_sbox(sbox),
            self[2].sub_sbox(sbox),
            self[3].sub_sbox(sbox),
        ]
    }

    fn sub_rsbox(&self, rsbox: &[u8]) -> Self {
        [
            self[0].sub_rsbox(rsbox),
            self[1].sub_rsbox(rsbox),
            self[2].sub_rsbox(rsbox),
            self[3].sub_rsbox(rsbox),
        ]
    }
}

pub fn transpose(input: &u8x4x4) -> u8x4x4 {
    let mut out = [[0; 4]; 4];
    for (i, &n) in input.iter().enumerate() {
        for (j, &u) in n.iter().enumerate() {
            out[j][i] = u;
        }
    }
    out
}
