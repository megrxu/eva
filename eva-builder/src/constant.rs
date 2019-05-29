#[derive(Debug, PartialEq)]
pub enum Constant {
    Array(ConstantArr),
    Matrix(ConstantMat),
}

#[derive(Debug, PartialEq)]
pub struct ConstantArr {
    pub name: String,
    pub value: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantMat {
    pub name: String,
    pub value: Vec<Vec<u8>>,
}
