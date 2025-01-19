mod parser;
mod value;

use std::collections::HashMap;
use std::env;
use value::Value;

struct Vm {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
}

impl Vm {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            vars: HashMap::new(),
        }
    }
}

fn main() {
    // if let Some(f) = env::args().nth(1).and_then(|f| std::fs::File::open(f).ok()) {
	// 	parse_batch(BufReader::new(f));
	// } else {
	// 	parse_interactive();
	// }
    let mut vm = Vm::new();
    for raw_line in std::io::stdin().lines().flatten() {
        parser::parse(&raw_line, &mut vm);
    }
}
