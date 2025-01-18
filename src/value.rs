#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Symbol(&'src str),
    Block(Vec<Value<'src>>),
}

impl <'src> Value<'src> {
    pub fn as_block_vec(self) -> Vec<Value<'src>> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a block")
        }
    }
    pub fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("value is not number"),
        }
    }
    pub fn as_str(&self) -> &'src str {
        match self {
            Self::Symbol(val) => val,
            _ => panic!("value is not str"),
        }
    }
}
