use super::constant::{Constant, ConstantArr, ConstantMat};

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
