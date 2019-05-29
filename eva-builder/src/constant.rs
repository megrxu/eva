#[derive(Debug, PartialEq)]
pub enum Constant {
    Array(ConstantArr),
    Matrix(ConstantMat),
}

#[derive(Debug, PartialEq)]
pub struct ConstantArr {
    name: String,
    value: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantMat {
    name: String,
    value: Vec<Vec<u8>>,
}

impl Constant {
    pub fn new(line: &str) -> Self {
        if line.contains(';') {
            Constant::Matrix(ConstantMat::from(line))
        } else {
            Constant::Array(ConstantArr::from(line))
        }
    }
}

impl From<&str> for ConstantArr {
    fn from(line: &str) -> Self {
        let splited: Vec<&str> = line.trim().split('=').collect();
        let nums: Vec<&str> = splited[1][1..splited[1].len() - 1].split(',').collect();
        ConstantArr {
            name: String::from(splited[0]),
            value: nums
                .into_iter()
                .map(|x: &str| String::from(x).parse::<u8>().unwrap())
                .collect(),
        }
    }
}

impl From<&str> for ConstantMat {
    fn from(line: &str) -> Self {
        let splited: Vec<&str> = line.trim().split('=').collect();
        let nums: Vec<&str> = splited[1][1..splited[1].len() - 1].split(';').collect();
        let rows: Vec<Vec<&str>> = nums
            .into_iter()
            .map(|x: &str| x.split(',').collect())
            .collect();
        ConstantMat {
            name: String::from(splited[0]),
            value: rows
                .into_iter()
                .map(|x: Vec<&str>| {
                    x.into_iter()
                        .map(|y| String::from(y).parse::<u8>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
#[test]
fn parse_const() {
    assert_eq!(
        ConstantArr::from("p2=[8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,0,1,2,3,4,5,6,7]"),
        ConstantArr {
            name: "p2".to_string(),
            value: vec![8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,0,1,2,3,4,5,6,7]
        }
    );
    assert_eq!(
        ConstantMat::from("m1=[1,0,2,0;2,3,4,5;1,0,2,0;2,3,4,5]"),
        ConstantMat {
            name: "m1".to_string(),
            value: vec![
                vec![1, 0, 2, 0],
                vec![2, 3, 4, 5],
                vec![1, 0, 2, 0],
                vec![2, 3, 4, 5]
            ]
        }
    );
    assert_eq!(
        Constant::new("m1=[1,0,2,0;2,3,4,5;1,0,2,0;2,3,4,5]"),
        Constant::Matrix(ConstantMat {
            name: "m1".to_string(),
            value: vec![
                vec![1, 0, 2, 0],
                vec![2, 3, 4, 5],
                vec![1, 0, 2, 0],
                vec![2, 3, 4, 5]
            ]
        })
    );
}

#[test]
fn test_sbox() {
    let sbox: [u8; 16] = [
        0xc, 0x5, 0x6, 0xb, 0x9, 0x0, 0xa, 0xd, 0x3, 0xe, 0xf, 0x8, 0x4, 0x7, 0x1, 0x2,
    ];
    for i in 0..16 {
        assert_eq!(alge_sbox(i as u8), sbox[i]);
    }
}

fn alge_sbox(input: u8) -> u8 {
    use eva_crypto::generic::{expand_bits, restore_data};
    let sbox: Vec<bool> = ConstantArr::from(
        "s=[1,1,0,1,1,0,0,0,1,0,0,0,1,1,1,0,1,1,1,0,0,0,1,1,0,0,1,0,1,1,0,0,0,1,0,1,0,1,1,0,0,0,0,0,1,1,1,0,0,1,1,0,1,0,0,0,1,0,0,0,0,0,0,0]",
    )
    .value
    .into_iter()
    .map(|x| match x {
        0 => false,
        _ => true,
    })
    .collect();
    let bits = expand_bits(&vec![input], 4);
    let mut ret_bits = vec![false; 4];
    // bool's power
    let power = |x: bool, y: bool| match y {
        false => true,
        true => x,
    };
    // Convert the radix 2 index to the permutation index
    let convert_table: [u8; 16] = [
        0b0000, 0b1000, 0b0100, 0b0010, 0b0001, 0b1100, 0b1010, 0b1001, 0b0110, 0b0101, 0b0011,
        0b1110, 0b1101, 0b1011, 0b0111, 0b1111,
    ];

    for i in 0..4 {
        let mut bit = false;
        for j in 0..16 {
            let indices = expand_bits(&vec![convert_table[j]], 4);
            bit ^= (0..4)
                .into_iter()
                .fold(true, |s, t| s & power(bits[t], indices[t]))
                & sbox[(i * 16) as usize + j];
        }
        ret_bits[i] = bit;
    }
    restore_data(&ret_bits, 4)[0]
}
