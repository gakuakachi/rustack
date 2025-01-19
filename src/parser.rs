use crate::value::Value;
use crate::Vm;

macro_rules! impl_op {
	($name:ident, $op:tt) => {
		fn $name(vm: &mut Vm) {
			let rhs = vm.stack.pop().unwrap().as_num();
			let lhs = vm.stack.pop().unwrap().as_num();
			vm.stack.push(Value::Num((lhs $op rhs) as i32));
		}
	}
}

impl_op!(add, +);
impl_op!(sub, -);
impl_op!(mul, *);
impl_op!(div, /);
impl_op!(lt, <);

fn op_def(vm: &mut Vm) {
    let val = vm.stack.pop().unwrap();
    let var = vm.stack.pop().unwrap().to_string();
    vm.vars.insert(var, val);
}

fn op_puts(vm: &mut Vm) {
    let val = vm.stack.pop().unwrap();
    println!("{}", val.to_string());
}

fn op_if(vm: &mut Vm) {
    let false_block = vm.stack.pop().unwrap().as_block_vec();
    let true_block = vm.stack.pop().unwrap().as_block_vec();
    let condition_block = vm.stack.pop().unwrap().as_block_vec();

    for code in condition_block {
        eval(code, vm);
    }

    let condition_result = vm.stack.pop().unwrap().as_num();

    if condition_result != 0 {
        for code in true_block {
            eval(code, vm);
        }
    } else {
        for code in false_block {
            eval(code, vm);
        }
    }
}

fn eval<'src>(code: Value, vm: &mut Vm) {
    match code {
        Value::Op(op) => match op.as_str() {
            "+" => add(vm),
            "-" => sub(vm),
            "*" => mul(vm),
            "/" => div(vm),
            "<" => lt(vm),
            "def" => op_def(vm),
            "if" => op_if(vm),
            "puts" => op_puts(vm),
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

    mod ops {
        use std::collections::HashMap;

        use super::*;

        #[test]
        fn test_add() {
            let mut vm = Vm {
                stack: vec![Num(10), Num(10)],
                vars: HashMap::new(),
            };
            add(&mut vm);
            let res = vm.stack.last().unwrap().as_num();

            assert_eq!(res, 20,)
        }

        #[test]
        fn test_sub() {
            let mut vm = Vm {
                stack: vec![Num(10), Num(10)],
                vars: HashMap::new(),
            };
            sub(&mut vm);
            let res = vm.stack.last().unwrap().as_num();

            assert_eq!(res, 0,)
        }

        #[test]
        fn test_mul() {
            let mut vm = Vm {
                stack: vec![Num(10), Num(10)],
                vars: HashMap::new(),
            };

            mul(&mut vm);
            let res = vm.stack.last().unwrap().as_num();

            assert_eq!(res, 100,)
        }

        #[test]
        fn test_div() {
            let mut vm = Vm {
                stack: vec![Num(10), Num(10)],
                vars: HashMap::new(),
            };

            div(&mut vm);
            let res = vm.stack.last().unwrap().as_num();

            assert_eq!(res, 1,)
        }

        #[test]
        fn test_op_def() {
            let mut vm = Vm {
                stack: vec![Symbol("test".to_owned()), Num(10)],
                vars: HashMap::new(),
            };

            op_def(&mut vm);

            let res = vm.vars.get("test").unwrap().as_num();

            assert_eq!(res, 10);
        }
    }

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
