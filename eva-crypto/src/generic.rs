use bitreader::BitReader;

#[allow(non_camel_case_types)]
pub type u8x4 = [u8; 4];

#[allow(non_camel_case_types)]
pub type u8x4x4 = [u8x4; 4];

pub trait Ops {
    fn lrot(&self) -> Self;
    fn rrot(&self) -> Self;
    fn xor(&self, rhs: &Self) -> Self;
    fn and(&self, rhs: &Self) -> Self;
    fn gmul(&self, rhs: &Self, bits: u8) -> Self;
}

pub trait Permutation {
    fn sub_sbox(&self, sbox: &[u8]) -> Self;
}

impl Ops for u8x4x4 {
    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [[1, 2, 3, 4]; 4].lrot(),
    ///     [
    ///         [1, 2, 3, 4],
    ///         [2, 3, 4, 1],
    ///         [3, 4, 1, 2],
    ///         [4, 1, 2, 3]
    ///     ]
    /// );
    /// ```
    fn lrot(&self) -> Self {
        [
            self[0],
            self[1].lrot(),
            self[2].lrot().lrot(),
            self[3].lrot().lrot().lrot(),
        ]
    }

    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [[1, 2, 3, 4]; 4].lrot().rrot(),
    ///     [[1, 2, 3, 4]; 4]
    /// );
    /// ```
    fn rrot(&self) -> Self {
        [
            self[0],
            self[1].rrot(),
            self[2].rrot().rrot(),
            self[3].rrot().rrot().rrot(),
        ]
    }

    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [[0x1, 0x2, 0x3, 0x4]; 4].xor(&[[0x11, 0x22, 0x33, 0x44]; 4]),
    ///     [[0x10, 0x20, 0x30, 0x40]; 4]
    /// );
    /// ```
    fn xor(&self, rhs: &Self) -> Self {
        [
            self[0].xor(&rhs[0]),
            self[1].xor(&rhs[1]),
            self[2].xor(&rhs[2]),
            self[3].xor(&rhs[3]),
        ]
    }

    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [[0x1, 0x2, 0x3, 0x4]; 4].and(&[[0x10, 0x20, 0x30, 0x40]; 4]),
    ///     [[0; 4]; 4]
    /// );
    /// ```
    fn and(&self, rhs: &Self) -> Self {
        [
            self[0].and(&rhs[0]),
            self[1].and(&rhs[1]),
            self[2].and(&rhs[2]),
            self[3].and(&rhs[3]),
        ]
    }

    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [
    ///        [0x0e, 0x09, 0x0d, 0x0b],
    ///        [0x0b, 0x0e, 0x09, 0x0d],
    ///        [0x0d, 0x0b, 0x0e, 0x09],
    ///        [0x09, 0x0d, 0x0b, 0x0e],
    ///    ].gmul(
    ///    &[
    ///        [0x02, 0x01, 0x01, 0x03],
    ///        [0x03, 0x02, 0x01, 0x01],
    ///        [0x01, 0x03, 0x02, 0x01],
    ///        [0x01, 0x01, 0x03, 0x02],
    ///    ], 8),
    ///    [[1, 0, 0, 0]; 4].rrot()
    /// );
    /// ```
    fn gmul(&self, rhs: &Self, bits: u8) -> Self {
        [
            self[0].gmul(&[rhs[0][0]; 4], bits),
            self[0].gmul(&[rhs[1][0]; 4], bits),
            self[0].gmul(&[rhs[2][0]; 4], bits),
            self[0].gmul(&[rhs[3][0]; 4], bits),
        ]
        .xor(&[
            self[1].gmul(&[rhs[0][1]; 4], bits),
            self[1].gmul(&[rhs[1][1]; 4], bits),
            self[1].gmul(&[rhs[2][1]; 4], bits),
            self[1].gmul(&[rhs[3][1]; 4], bits),
        ])
        .xor(&[
            self[2].gmul(&[rhs[0][2]; 4], bits),
            self[2].gmul(&[rhs[1][2]; 4], bits),
            self[2].gmul(&[rhs[2][2]; 4], bits),
            self[2].gmul(&[rhs[3][2]; 4], bits),
        ])
        .xor(&[
            self[3].gmul(&[rhs[0][3]; 4], bits),
            self[3].gmul(&[rhs[1][3]; 4], bits),
            self[3].gmul(&[rhs[2][3]; 4], bits),
            self[3].gmul(&[rhs[3][3]; 4], bits),
        ])
    }
}

impl Ops for u8x4 {
    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [1, 2, 3, 4].lrot(),
    ///     [2, 3, 4, 1]
    /// );
    /// ```
    fn lrot(&self) -> Self {
        [self[1], self[2], self[3], self[0]]
    }

    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [1, 2, 3, 4].lrot().rrot(),
    ///     [1, 2, 3, 4]
    /// );
    /// ```
    fn rrot(&self) -> Self {
        [self[3], self[0], self[1], self[2]]
    }

    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [0x0, 0x1, 0x2, 0x3].xor(&[0x3, 0x2, 0x1, 0x0]),
    ///     [0x3, 0x3, 0x3, 0x3]
    /// );
    /// ```

    fn xor(&self, rhs: &Self) -> Self {
        [
            self[0] ^ rhs[0],
            self[1] ^ rhs[1],
            self[2] ^ rhs[2],
            self[3] ^ rhs[3],
        ]
    }

    /// ```
    /// use eva_crypto::generic::Ops;
    /// assert_eq!(
    ///     [0x0, 0x1, 0x1, 0x0].and(&[0x1, 0x3, 0x3, 0x1]),
    ///     [0x0, 0x1, 0x1, 0x0]
    /// );
    /// ```
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

impl Ops for u8 {
    fn lrot(&self) -> Self {
        self.rotate_left(1)
    }
    fn rrot(&self) -> Self {
        self.rotate_right(1)
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

/// Implementation of Galois field multiplication. `poly` denotes the used irreducible polynomial in bits.
pub fn gmul_x(mut a: u8, mut b: u8, poly: u8, bits: u8) -> u8 {
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
}

/// Create a 4x4 state matrix from a 16 sized u8 array, used in many blcok ciphers.
/// ```
/// use eva_crypto::generic::create_u8x4x4;
/// assert_eq!(
///        create_u8x4x4(&[0x1; 16]),
///        [[0x1;4]; 4]
///    );
/// ```
pub fn create_u8x4x4(data: &[u8]) -> u8x4x4 {
    assert_eq!(data.len(), 16);
    let mut state = [[0; 4]; 4];
    for (i, &j) in data.iter().enumerate() {
        state[i / 4][i % 4] = j;
    }
    state
}

/// Create u8x4 from an u32 value.
/// ```
/// use eva_crypto::generic::create_u8x4;
/// assert_eq!(
///        create_u8x4(0x0102_0304),
///        [0x01, 0x02, 0x03, 0x04]
///    );
/// ```
pub fn create_u8x4(data: u32) -> u8x4 {
    [
        (data >> 24) as u8,
        (data >> 16) as u8,
        (data >> 8) as u8,
        data as u8,
    ]
}

/// Create u32 from an u8 array.
/// ```
/// use eva_crypto::generic::u8x4_to_u32;
/// assert_eq!(
///        u8x4_to_u32(&[0x01, 0x02, 0x03, 0x04]),
///        0x0102_0304
///    );
/// ```
pub fn u8x4_to_u32(data: &u8x4) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret ^= (data[3 - i] as u32) << (i * 8);
    }
    ret
}

/// Transpose a 4x4 state matrix, used in many blcok ciphers.
/// ```
/// use eva_crypto::generic::transpose;
/// assert_eq!(
///     transpose(&[
///         [0x1, 0x2, 0x3, 0x4],
///         [0x0, 0x0, 0x0, 0x0],
///         [0x0, 0x0, 0x0, 0x0],
///         [0x0, 0x0, 0x0, 0x0],
///     ]),
///     [
///         [0x1, 0x0, 0x0, 0x0],
///         [0x2, 0x0, 0x0, 0x0],
///         [0x3, 0x0, 0x0, 0x0],
///         [0x4, 0x0, 0x0, 0x0],
///     ]
/// );
/// ```
pub fn transpose(input: &u8x4x4) -> u8x4x4 {
    let mut out = [[0; 4]; 4];
    for (i, &n) in input.iter().enumerate() {
        for (j, &u) in n.iter().enumerate() {
            out[j][i] = u;
        }
    }
    out
}

/// Expand the data(4 bits) to bits vector.
/// ```
/// use eva_crypto::generic::expand_bits;
/// assert_eq!(
///        expand_bits(&vec![0b0001, 0b1101]),
///        [false, false, false, true, true, true, false, true]
///    );
/// ```
pub fn expand_bits(data: &Vec<u8>) -> Vec<bool> {
    let bytes = &data[..];
    let mut reader = BitReader::new(bytes);
    let mut ret: Vec<bool> = vec![];
    for _ in 0..data.len() {
        match reader.skip(4) {
            Ok(_) => (),
            _ => unreachable!(),
        };
        for _ in 0..4 {
            ret.push(reader.read_bool().unwrap());
        }
    }
    ret
}

/// Restore the data(4 bits) from a bit vector.
/// ```
/// use eva_crypto::generic::restore_data;
/// assert_eq!(
///     restore_data(&vec![false, false, false, true, true, true, false, true]),
///     [0b0001, 0b1101]
/// );
/// ```
pub fn restore_data(bits: &Vec<bool>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![];
    let mut buffer: u8;
    for i in (0..bits.len()).step_by(4) {
        buffer = 0;
        for j in 0..4 {
            match bits[i + j] {
                true => buffer ^= 0b1000 >> j,
                _ => (),
            }
        }
        ret.push(buffer);
    }
    ret
}
