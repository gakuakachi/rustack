mod operations;

use crate::value::Value;
use crate::Vm;

fn eval<'src>(code: Value, vm: &mut Vm) {
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

pub fn parse<'a>(line: &'a str, vm: &mut Vm) {
    let input: Vec<_> = line.split(" ").collect();
    let mut words = &input[..];

    while let Some((&word, mut rest)) = words.split_first() {
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            vm.stack.push(value);
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
        words = rest;
    }
}

fn parse_block<'src, 'a>(input: &'a [&'src str]) -> (Value, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }

        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value);
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else {
            let code = if let Ok(value) = word.parse::<i32>() {
                Value::Num(value)
            } else if word.starts_with("/") {
                let symbol = String::from(&word[1..]);
                Value::Symbol(symbol)
            } else {
                let op = String::from(word);
                Value::Op(op)
            };
            tokens.push(code);
        }
        words = rest;
    }
    (Value::Block(tokens), words)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value::*;

    #[test]
    fn test_group() {
        let mut vm = Vm::new();
        parse("1 2 + { 3 4 }", &mut vm);
        assert_eq!(vm.stack, vec![Num(3), Block(vec![Num(3), Num(4)])]);
    }

    #[test]
    fn test_if_true() {
        let mut vm = Vm::new();
        parse("{ 1 1 + } { 100 } { -100 } if", &mut vm);
        assert_eq!(vm.stack, vec![Num(100)],)
    }

    #[test]
    fn test_parse_block() {
        let input = "1 1 + }";
        let words: Vec<_> = input.split(" ").collect();
        let (res, _) = parse_block(&words);
        assert_eq!(res, Block(vec![Num(1), Num(1), Op("+".to_owned())]),)
    }
}
