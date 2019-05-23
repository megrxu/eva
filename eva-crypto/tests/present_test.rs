use eva_crypto::present::*;

#[cfg(test)]
#[test]
fn test_key_expansion() {
    assert_eq!(
        PRESENT::new(&[0; 20]).round_keys[31].concat(),
        [0x6, 0xd, 0xa, 0xb, 0x3, 0x1, 0x7, 0x4, 0x4, 0xf, 0x4, 0x1, 0xd, 0x7, 0x0, 0x0]
    );
}

#[test]
fn test_present_80() {
    let key = [0u8; 20];
    let plaintext = [0u8; 16];
    let ciphertext = [5, 0xc, 7, 8, 5, 1, 0xb, 4, 7, 3, 2, 4, 9, 8, 2, 5];
    assert_eq!(PRESENT::new(&key).encrypt(&plaintext), ciphertext);
    assert_eq!(PRESENT::new(&key).decrypt(&ciphertext), plaintext);
}

#[test]
fn test_present_128() {
    let key = [0u8; 32];
    let plaintext = [0u8; 16];
    let ciphertext = [9, 7, 2, 0, 6, 0, 0xe, 0, 0xd, 2, 6, 0xa, 0xb, 0xa, 0x9, 0xf];
    assert_eq!(PRESENT::new(&key).encrypt(&plaintext), ciphertext);
    assert_eq!(PRESENT::new(&key).decrypt(&ciphertext), plaintext);
}
