use super::operations;
use crate::value::Value;
use crate::Vm;

pub fn eval<'src>(code: Value, vm: &mut Vm) {
    if let Some(top_block) = vm.blocks.last_mut() {
        top_block.push(code);
        return;
    }

    match code {
        Value::Op(op) => match op.as_str() {
            "+" => operations::add(vm),
            "-" => operations::sub(vm),
            "*" => operations::mul(vm),
            "/" => operations::div(vm),
            "<" => operations::lt(vm),
            "def" => operations::op_def(vm),
            "if" => operations::op_if(vm),
            "puts" => operations::op_puts(vm),
            _ => {
                let val = vm
                    .vars
                    .get(&op)
                    .expect(&format!("{op:?} is not a defined operation"));
                vm.stack.push(val.clone());
            }
        },
        _ => vm.stack.push(code.clone()),
    }
}

pub fn parse_word(word: &str, vm: &mut Vm) {
    if word.is_empty() {
        return;
    }

    if word == "{" {
        vm.blocks.push(Vec::new());
    } else if word == "}" {
        let block = vm.blocks.pop().expect("Block stack underrun!");
        eval(Value::Block(block), vm);
    } else {
        let code = if let Ok(parsed) = word.parse::<i32>() {
            Value::Num(parsed)
        } else if word.starts_with("/") {
            let symbol = String::from(&word[1..]);
            Value::Symbol(symbol)
        } else {
            let op = String::from(word);
            Value::Op(op)
        };
        eval(code, vm);
    }
}

#[cfg(test)]
mod test {
    use super::super::parse_batch;

    use crate::value::Value::*;
    use std::io::Cursor;

    #[test]
    fn test_group() {
        let input = Cursor::new("1 2 + { 3 4 }");
        let stack = parse_batch(input);
        assert_eq!(stack, vec![Num(3), Block(vec![Num(3), Num(4)])]);
    }

    #[test]
    fn test_if_true() {
        let input = Cursor::new("{ 1 1 + } { 100 } { -100 } if");
        let stack = parse_batch(input);
        assert_eq!(stack, vec![Num(100)]);
    }
}
