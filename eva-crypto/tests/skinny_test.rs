use eva_crypto::skinny;

#[cfg(test)]
#[test]
fn skinny_64_64() {
    let key: [u8; 16] = [0xf, 0x5, 0x2, 0x6, 0x9, 0x8, 0x2, 0x6, 0xf, 0xc, 0x6, 0x8, 0x1, 0x2, 0x3, 0x8];
    let plaintext: [u8; 16] = [0x0, 0x6, 0x0, 0x3, 0x4, 0xf, 0x9, 0x5, 0x7, 0x7, 0x2, 0x4, 0xd, 0x1, 0x9, 0xd];
    let ciphertext: [u8; 16] = [
        0xb, 0xb , 0x3 , 0x9 , 0xd , 0xf , 0xb , 0x2 , 0x4 , 0x2 , 0x9 , 0xb , 0x8 , 0xa , 0xc , 0x7
    ];

    let cipher = skinny::SKINNY::new(&key);
    assert_eq!(cipher.encrypt(&plaintext), ciphertext);
    assert_eq!(cipher.decrypt(&ciphertext), plaintext);
}