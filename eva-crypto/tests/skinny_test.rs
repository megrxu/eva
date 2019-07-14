use eva_crypto::skinny;

#[cfg(test)]
#[test]
fn skinny_64_64() {
    let key: [u8; 16] = [
        0xf, 0x5, 0x2, 0x6, 0x9, 0x8, 0x2, 0x6, 0xf, 0xc, 0x6, 0x8, 0x1, 0x2, 0x3, 0x8,
    ];
    let plaintext: [u8; 16] = [
        0x0, 0x6, 0x0, 0x3, 0x4, 0xf, 0x9, 0x5, 0x7, 0x7, 0x2, 0x4, 0xd, 0x1, 0x9, 0xd,
    ];
    let ciphertext: [u8; 16] = [
        0xb, 0xb, 0x3, 0x9, 0xd, 0xf, 0xb, 0x2, 0x4, 0x2, 0x9, 0xb, 0x8, 0xa, 0xc, 0x7,
    ];

    let cipher = skinny::SKINNY::new(&key, 4);
    assert_eq!(cipher.encrypt(&plaintext), ciphertext);
    assert_eq!(cipher.decrypt(&ciphertext), plaintext);
}

#[test]
fn skinny_64_128() {
    let key: [u8; 32] = [
        0x9, 0xe, 0xb, 0x9, 0x3, 0x6, 0x4, 0x0, 0xd, 0x0, 0x8, 0x8, 0xd, 0xa, 0x6, 0x3, 0x7, 0x6,
        0xa, 0x3, 0x9, 0xd, 0x1, 0xc, 0x8, 0xb, 0xe, 0xa, 0x7, 0x1, 0xe, 0x1,
    ];
    let plaintext: [u8; 16] = [
        0xc, 0xf, 0x1, 0x6, 0xc, 0xf, 0xe, 0x8, 0xf, 0xd, 0x0, 0xf, 0x9, 0x8, 0xa, 0xa,
    ];
    let ciphertext: [u8; 16] = [
        0x6, 0xc, 0xe, 0xd, 0xa, 0x1, 0xf, 0x4, 0x3, 0xd, 0xe, 0x9, 0x2, 0xb, 0x9, 0xe,
    ];

    let cipher = skinny::SKINNY::new(&key, 4);
    assert_eq!(cipher.encrypt(&plaintext), ciphertext);
    assert_eq!(cipher.decrypt(&ciphertext), plaintext);
}

#[test]
fn skinny_64_192() {
    let key: [u8; 48] = [
        0xe, 0xd, 0x0, 0x0, 0xc, 0x8, 0x5, 0xb, 0x1, 0x2, 0x0, 0xd, 0x6, 0x8, 0x6, 0x1, 0x8, 0x7,
        0x5, 0x3, 0xe, 0x2, 0x4, 0xb, 0xf, 0xd, 0x9, 0x0, 0x8, 0xf, 0x6, 0x0, 0xb, 0x2, 0xd, 0xb,
        0xb, 0x4, 0x1, 0xb, 0x4, 0x2, 0x2, 0xd, 0xf, 0xc, 0xd, 0x0,
    ];
    let plaintext: [u8; 16] = [
        0x5, 0x3, 0x0, 0xc, 0x6, 0x1, 0xd, 0x3, 0x5, 0xe, 0x8, 0x6, 0x6, 0x3, 0xc, 0x3,
    ];
    let ciphertext: [u8; 16] = [
        0xd, 0xd, 0x2, 0xc, 0xf, 0x1, 0xa, 0x8, 0xf, 0x3, 0x3, 0x0, 0x3, 0x0, 0x3, 0xc,
    ];

    let cipher = skinny::SKINNY::new(&key, 4);
    assert_eq!(cipher.encrypt(&plaintext), ciphertext);
    assert_eq!(cipher.decrypt(&ciphertext), plaintext);
}

#[test]
fn skinny_128_128() {
    let key: [u8; 16] = [
        0x4f, 0x55, 0xcf, 0xb0, 0x52, 0x0c, 0xac, 0x52, 0xfd, 0x92, 0xc1, 0x5f, 0x37, 0x07, 0x3e,
        0x93,
    ];
    let plaintext: [u8; 16] = [
        0xf2, 0x0a, 0xdb, 0x0e, 0xb0, 0x8b, 0x64, 0x8a, 0x3b, 0x2e, 0xee, 0xd1, 0xf0, 0xad, 0xda,
        0x14,
    ];
    let ciphertext: [u8; 16] = [
        0x22, 0xff, 0x30, 0xd4, 0x98, 0xea, 0x62, 0xd7, 0xe4, 0x5b, 0x47, 0x6e, 0x33, 0x67, 0x5b,
        0x74,
    ];

    let cipher = skinny::SKINNY::new(&key, 8);
    assert_eq!(cipher.encrypt(&plaintext), ciphertext);
    assert_eq!(cipher.decrypt(&ciphertext), plaintext);
}

#[test]
fn skinny_128_256() {
    let key: [u8; 32] = [
        0x00, 0x9c, 0xec, 0x81, 0x60, 0x5d, 0x4a, 0xc1, 0xd2, 0xae, 0x9e, 0x30, 0x85, 0xd7, 0xa1,
        0xf3, 0x1a, 0xc1, 0x23, 0xeb, 0xfc, 0x00, 0xfd, 0xdc, 0xf0, 0x10, 0x46, 0xce, 0xed, 0xdf,
        0xca, 0xb3,
    ];
    let plaintext: [u8; 16] = [
        0x3a, 0x0c, 0x47, 0x76, 0x7a, 0x26, 0xa6, 0x8d, 0xd3, 0x82, 0xa6, 0x95, 0xe7, 0x02, 0x2e,
        0x25,
    ];
    let ciphertext: [u8; 16] = [
        0xb7, 0x31, 0xd9, 0x8a, 0x4b, 0xde, 0x14, 0x7a, 0x7e, 0xd4, 0xa6, 0xf1, 0x6b, 0x9b, 0x58,
        0x7f,
    ];

    let cipher = skinny::SKINNY::new(&key, 8);
    assert_eq!(cipher.encrypt(&plaintext), ciphertext);
    assert_eq!(cipher.decrypt(&ciphertext), plaintext);
}

#[test]
fn skinny_128_384() {
    let key: [u8; 48] = [
        0xdf, 0x88, 0x95, 0x48, 0xcf, 0xc7, 0xea, 0x52, 0xd2, 0x96, 0x33, 0x93, 0x01, 0x79, 0x74,
        0x49, 0xab, 0x58, 0x8a, 0x34, 0xa4, 0x7f, 0x1a, 0xb2, 0xdf, 0xe9, 0xc8, 0x29, 0x3f, 0xbe,
        0xa9, 0xa5, 0xab, 0x1a, 0xfa, 0xc2, 0x61, 0x10, 0x12, 0xcd, 0x8c, 0xef, 0x95, 0x26, 0x18,
        0xc3, 0xeb, 0xe8,
    ];
    let plaintext: [u8; 16] = [
        0xa3, 0x99, 0x4b, 0x66, 0xad, 0x85, 0xa3, 0x45, 0x9f, 0x44, 0xe9, 0x2b, 0x08, 0xf5, 0x50,
        0xcb,
    ];
    let ciphertext: [u8; 16] = [
        0x94, 0xec, 0xf5, 0x89, 0xe2, 0x01, 0x7c, 0x60, 0x1b, 0x38, 0xc6, 0x34, 0x6a, 0x10, 0xdc,
        0xfa,
    ];

    let cipher = skinny::SKINNY::new(&key, 8);
    assert_eq!(cipher.encrypt(&plaintext), ciphertext);
    assert_eq!(cipher.decrypt(&ciphertext), plaintext);
}
