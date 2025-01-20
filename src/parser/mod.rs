mod core;
mod operations;

use std::io::BufRead;

use crate::value::Value;
use crate::Vm;

pub fn parse_batch(source: impl BufRead) -> Vec<Value> {
    let mut vm = Vm::new();
    for line in source.lines().flatten() {
        for word in line.split(" ") {
            core::parse_word(word, &mut vm);
        }
    }

    vm.stack
}

pub fn parse_interactive() {
    let mut vm = Vm::new();
    for line in std::io::stdin().lines().flatten() {
        for word in line.split(" ") {
            core::parse_word(word, &mut vm);
        }
    }
    println!("stack: {:?}", vm.stack);
}

#[cfg(test)]
mod tests {
    use super::parse_batch;
    use std::io::Cursor;

    use crate::value::Value;

    #[test]
    fn test_nested_block() {
        let res = parse_batch(Cursor::new("{ { 1 } }"));
        assert_eq!(
            res,
            vec![Value::Block(vec![Value::Block(vec![Value::Num(1)])])]
        )
    }
}
