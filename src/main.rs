mod parser;
mod value;

use std::collections::HashMap;
use value::Value;

struct Vm<'src> {
    stack: Vec<Value<'src>>,
    vars: HashMap<&'src str, Value<'src>>,
}

impl<'src> Vm<'src> {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            vars: HashMap::new(),
        }
    }
}

fn main() {
    for raw_line in std::io::stdin().lines().flatten() {
        parser::parse(&raw_line);
    }
}
