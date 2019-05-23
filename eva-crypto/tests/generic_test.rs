use eva_crypto::generic::*;

#[cfg(test)]
#[test]
fn test_bits() {
    assert_eq!(
        expand_bits(&vec![0b0001, 0b1101]),
        [false, false, false, true, true, true, false, true]
    );
    assert_eq!(
        restore_data(&vec![false, false, false, true, true, true, false, true]),
        [0b0001, 0b1101]
    );
}
