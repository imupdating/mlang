use crate::states::States;
use crate::runner::Runner;
use crate::{object, tokens};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MObject {
    Number(Number),
    Function(Function),
}

// Number
#[derive(Debug, Clone)]
pub struct Number {
    value: tokens::Number,
}

impl Number {
    pub fn new(value: tokens::Number) -> Number {
        Number { value }
    }
}

// Function
#[derive(Debug, Clone)]
pub struct Function {
    pub args: Vec<String>,
    pub states: Vec<States>,
}

impl Function {
    pub fn new(args: Vec<String>, states: Vec<States>) -> Function {
        Function { args, states }
    }

    pub fn run(&self, variables: &HashMap<String, object::MObject>, stack: &Vec<object::MObject>) -> MObject { // Vec<mObject> is the stack
        let mut run = Runner::new(self.states.clone(), variables.clone());
        run.run();
        run.stack.pop().unwrap()
    }
}