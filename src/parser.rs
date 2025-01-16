use crate::value::Value;
use crate::Vm;

fn add(vm: &mut Vm) {
  let lhs = vm.stack.pop().unwrap().as_num();
  let rhs = vm.stack.pop().unwrap().as_num();
  vm.stack.push(Value::Num(lhs + rhs));
}

fn sub(vm: &mut Vm) {
  let lhs = vm.stack.pop().unwrap().as_num();
  let rhs = vm.stack.pop().unwrap().as_num();
  vm.stack.push(Value::Num(lhs - rhs));
}

fn mul(vm: &mut Vm) {
  let lhs = vm.stack.pop().unwrap().as_num();
  let rhs = vm.stack.pop().unwrap().as_num();
  vm.stack.push(Value::Num(lhs * rhs));
}

fn div(vm: &mut Vm) {
  let lhs = vm.stack.pop().unwrap().as_num();
  let rhs = vm.stack.pop().unwrap().as_num();
  vm.stack.push(Value::Num(lhs / rhs));
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

fn eval<'src>(code: Value<'src>, vm: &mut Vm<'src>) {
  match code {
      Value::Op(op) => match op {
          "+" => add(vm),
          "-" => sub(vm),
          "*" => mul(vm),
          "/" => div(vm),
          "if" => op_if(vm),
          _ => panic!("{code:?} could not be parsed"),
      }
      _ => vm.stack.push(code.clone()),
  }
}

pub fn parse<'a>(line: &'a str) -> Vm<'a> {
  let mut vm = Vm::new();
  let input: Vec<_> = line.split(" ").collect();
  let mut words = &input[..];

  while let Some((&word, mut rest)) = words.split_first() {
      if word == "{" {
          let value;
          (value, rest) = parse_block(rest);
          vm.stack.push(value);
      } else {
          if let Ok(parsed) = word.parse::<i32>() {
              vm.stack.push(Value::Num(parsed));
          } else {
              eval(Value::Op(word), &mut vm);
          }                        
      }
      words = rest;
  }
  vm
}

fn parse_block<'src, 'a>(
  input: &'a[&'src str],
) -> (Value<'src>, &'a[&'src str]) {
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
      } else if let Ok(value) = word.parse::<i32>() {
          tokens.push(Value::Num(value));
      } else {
          tokens.push(Value::Op(word));
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
      
      assert_eq!(
        res,
        20,
      )
    }

    #[test]
    fn test_sub() {
      let mut vm = Vm {
        stack: vec![Num(10), Num(10)],
        vars: HashMap::new(),
      };
      sub(&mut vm);
      let res = vm.stack.last().unwrap().as_num();
      
      assert_eq!(
        res,
        0,
      )
    }

    #[test]
    fn test_mul() {
      let mut vm = Vm {
        stack: vec![Num(10), Num(10)],
        vars: HashMap::new(),
      };

      mul(&mut vm);
      let res = vm.stack.last().unwrap().as_num();
      
      assert_eq!(
        res,
        100,
      )
    }

    #[test]
    fn test_div() {
      let mut vm = Vm {
        stack: vec![Num(10), Num(10)],
        vars: HashMap::new(),
      };

      div(&mut vm);
      let res = vm.stack.last().unwrap().as_num();
      
      assert_eq!(
        res,
        1,
      )
    }
    
  }

  #[test]
  fn test_group() {
    let res = parse("1 2 + { 3 4 }");
    assert_eq!(
      res.stack,
      vec![Num(3), Block(vec![Num(3), Num(4)])]
    );
  }

  #[test]
  fn test_if_true() {
    let res = parse("{ 1 1 + } { 100 } { -100 } if");
    assert_eq!(
        res.stack,
        vec![Num(100)],
    )
  }

  #[test]
  fn test_parse_block() {
    let input = "1 1 + }";
    let words: Vec<_> = input.split(" ").collect();
    let (res, _) = parse_block(&words);
    assert_eq!(
      res,
      Block(vec![Num(1), Num(1), Op("+")]),
    )
  }
}
