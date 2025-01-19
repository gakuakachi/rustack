#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Num(i32),
    Op(String),
    Symbol(String),
    Block(Vec<Value>),
}

impl Value {
    pub fn as_block_vec(self) -> Vec<Value> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a block"),
        }
    }
    pub fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("value is not number"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Num(num) => num.to_string(),
            Self::Symbol(val) => val.to_owned(),
            Self::Op(op) => op.to_owned(),
            Self::Block(_) => "<Block>".to_owned(),
        }
    }
}
