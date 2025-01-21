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
    use std::{fs::File, io::{BufRead, BufReader, Cursor}, path::PathBuf};

    use crate::value::Value;
    
    fn read_test_file(target: &str) -> impl BufRead {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(target);
        let f = File::open(d.as_path()).unwrap();
        BufReader::new(f)
    }

    #[test]
    fn test_nested_block() {
        let res = parse_batch(Cursor::new("{ { 1 } }"));
        assert_eq!(
            res,
            vec![Value::Block(vec![Value::Block(vec![Value::Num(1)])])]
        )
    }

    #[test]
    fn test_if_txt() {
        let buf = read_test_file("tests/if.txt");
        let res = parse_batch(buf);
        assert_eq!(
            res,
            vec![Value::Num(10)],
        )        
    }
    
    #[test]
    fn test_fn_txt() {
        let buf = read_test_file("tests/fn.txt");
        let res = parse_batch(buf);
        assert_eq!(
            res,
            vec![Value::Num(500)],
        )
    }
    
}
