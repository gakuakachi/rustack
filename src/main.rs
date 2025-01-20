mod parser;
mod value;

use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use value::Value;

struct Vm {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    blocks: Vec<Vec<Value>>,
}

impl Vm {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            vars: HashMap::new(),
            blocks: Vec::new(),
        }
    }
}

fn main() {
    if let Some(f) = env::args().nth(1).and_then(|f| std::fs::File::open(f).ok()) {
        parser::parse_batch(BufReader::new(f));
    } else {
        parser::parse_interactive();
    }
}
