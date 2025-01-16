use crate::value::Value;

fn add(stack: &mut Vec<Value>) {
  let lhs = stack.pop().unwrap().as_num();
  let rhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs + rhs));
}

fn sub(stack: &mut Vec<Value>) {
  let lhs = stack.pop().unwrap().as_num();
  let rhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs - rhs));
}

fn mul(stack: &mut Vec<Value>) {
  let lhs = stack.pop().unwrap().as_num();
  let rhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs * rhs));
}

fn div(stack: &mut Vec<Value>) {
  let lhs = stack.pop().unwrap().as_num();
  let rhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs / rhs));
}

fn op_if(stack: &mut Vec<Value>) {
  let false_block = stack.pop().unwrap().as_block_vec();
  let true_block = stack.pop().unwrap().as_block_vec();
  let condition_block = stack.pop().unwrap().as_block_vec(); 

  for code in condition_block {
      eval(code, stack);
  }

  let condition_result = stack.pop().unwrap().as_num();

  if condition_result != 0 {
      for code in true_block {
          eval(code, stack);
      }
  } else {
      for code in false_block {
          eval(code, stack);
      }
  }
}

fn eval<'src>(code: Value<'src>, stack: &mut Vec<Value<'src>>) {
  match code {
      Value::Op(op) => match op {
          "+" => add(stack),
          "-" => sub(stack),
          "*" => mul(stack),
          "/" => div(stack),
          "if" => op_if(stack),
          _ => panic!("{code:?} could not be parsed"),
      }
      _ => stack.push(code.clone()),
  }
}

pub fn parse<'a>(line: &'a str) -> Vec<Value<'a>> {
  let mut stack: Vec<Value> = vec![];
  let input: Vec<_> = line.split(" ").collect();
  let mut words = &input[..];

  while let Some((&word, mut rest)) = words.split_first() {
      if word == "{" {
          let value;
          (value, rest) = parse_block(rest);
          stack.push(value);
      } else {
          if let Ok(parsed) = word.parse::<i32>() {
              stack.push(Value::Num(parsed));
          } else {
              eval(Value::Op(word), &mut stack);
          }                        
      }
      words = rest;
      println!("stack: {stack:?}");
  }
  stack
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
    use super::*;

    #[test]
    fn test_add() {
      let mut stack = vec![Num(10), Num(10)];
      add(&mut stack);
      let res = stack.last().unwrap().as_num();
      
      assert_eq!(
        res,
        20,
      )
    }

    #[test]
    fn test_sub() {
      let mut stack = vec![Num(10), Num(10)];
      sub(&mut stack);
      let res = stack.last().unwrap().as_num();
      
      assert_eq!(
        res,
        0,
      )
    }

    #[test]
    fn test_mul() {
      let mut stack = vec![Num(10), Num(10)];
      mul(&mut stack);
      let res = stack.last().unwrap().as_num();
      
      assert_eq!(
        res,
        100,
      )
    }

    #[test]
    fn test_div() {
      let mut stack = vec![Num(10), Num(10)];
      div(&mut stack);
      let res = stack.last().unwrap().as_num();
      
      assert_eq!(
        res,
        1,
      )
    }
    
  }

  #[test]
  fn test_group() {
    assert_eq!(
      parse("1 2 + { 3 4 }"),
      vec![Num(3), Block(vec![Num(3), Num(4)])]
    );
  }

  #[test]
  fn test_if_true() {
    assert_eq!(
        parse("{ 1 1 + } { 100 } { -100 } if"),
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
