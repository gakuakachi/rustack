use super::core::eval;
use crate::value::Value;
use crate::Vm;

macro_rules! impl_op {
	($name:ident, $op:tt) => {
		pub fn $name(vm: &mut Vm) {
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

pub fn duplicate(vm: &mut Vm) {
    let value = vm.stack.last().unwrap();
    vm.stack.push(value.clone());
}

pub fn exchange(vm: &mut Vm) {
    let x = vm.stack.pop().unwrap();
    let y = vm.stack.pop().unwrap();
    vm.stack.push(x);
    vm.stack.push(y);
}

pub fn op_def(vm: &mut Vm) {
    let val = vm.stack.pop().unwrap();
    let var = vm.stack.pop().unwrap().to_string();
    vm.vars.insert(var, val);
}

pub fn op_puts(vm: &mut Vm) {
    let val = vm.stack.pop().unwrap();
    println!("{}", val.to_string());
}

pub fn op_if(vm: &mut Vm) {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value::*;
    use std::collections::HashMap;

    #[test]
    fn test_add() {
        let mut vm = Vm {
            stack: vec![Num(10), Num(10)],
            vars: HashMap::new(),
            blocks: Vec::new(),
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
            blocks: Vec::new(),
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
            blocks: Vec::new(),
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
            blocks: Vec::new(),
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
            blocks: Vec::new(),
        };

        op_def(&mut vm);

        let res = vm.vars.get("test").unwrap().as_num();

        assert_eq!(res, 10);
    }

    #[test]
    fn test_duplicate() {
        let mut vm = Vm {
            stack: vec![Num(10)],
            vars: HashMap::new(),
            blocks: Vec::new(),
        };

        duplicate(&mut vm);
        assert_eq!(vm.stack, vec![Num(10), Num(10)]);        
    }

    #[test]
    fn test_exchange() {
        let mut vm = Vm {
            stack: vec![Num(10), Num(20)],
            vars: HashMap::new(),
            blocks: Vec::new(),
        };
        exchange(&mut vm);
        assert_eq!(vm.stack, vec![Num(20), Num(10)]);
    }
}
