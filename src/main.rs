mod parser;
mod value;

use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use value::Value;

struct Vm {
    stack: Vec<Value>,
    vars: Vec<HashMap<String, Value>>,
    blocks: Vec<Vec<Value>>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            vars: vec![HashMap::new()],
            blocks: Vec::new(),
        }
    }

    fn find_var(&self, name: &str) -> Option<Value> {
        self.vars.iter().rev().find_map(
            |vars| vars.get(name).map(|var| var.to_owned())
        )
    }
}

fn main() {
    if let Some(f) = env::args().nth(1).and_then(|f| std::fs::File::open(f).ok()) {
        parser::parse_batch(BufReader::new(f));
    } else {
        parser::parse_interactive();
    }
}
