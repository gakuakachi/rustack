use crate::Vm;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Num(i32),
    Op(String),
    Symbol(String),
    Block(Vec<Value>),
    Native(NativeOp)
}

#[derive(Clone)]
pub struct NativeOp(pub fn(&mut Vm));

impl PartialEq for NativeOp {
    fn eq(&self, other: &NativeOp) -> bool {
        self.0 as *const fn() == other.0 as *const fn()
    }
}

impl Eq for NativeOp {}

impl std::fmt::Debug for NativeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NativeOp>")
    }
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
            Self::Native(_) => "<NativeOp>".to_owned(),
        }
    }
}
