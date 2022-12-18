use crate::object::MObject;
use crate::states;
use crate::object;
use std::collections::HashMap;

pub struct Runner {
    pub states: Vec<states::States>,
    pub variables: HashMap<String, object::MObject>,
    pub stack: Vec<object::MObject>,
}

impl Runner {
    pub fn new(states: Vec<states::States>, variables: HashMap::<String, object::MObject>) -> Runner {
        Runner { 
            states,
            variables,
            stack: Vec::<object::MObject>::new(),
        }
    }

    pub fn run(&mut self) {
        for pos in 0..self.states.len() {
            let this = &self.states[pos];
            match this {
                states::States::NewFunction(function_name, args, states) => {
                    let obj = object::Function{ args: args.to_vec(), states: states.to_vec() };
                    self.variables.insert(function_name.to_string(), object::MObject::Function(obj));
                    println!("声明了{}函数，参数为{:#?}", function_name, args);
                },
                states::States::If(expr, states) => println!("If"),
                states::States::Return(expr) => {
                    let var = expr.clone().value(self.variables.clone());
                    self.stack.push(var);
                },
                states::States::CallFunction(function_name) => {
                    match self.variables.get(function_name).expect("你在访问一个不存在的函数！") {
                        object::MObject::Number(_) => panic!("不能调用数字"),
                        object::MObject::Function(f) => { self.stack.push(f.run(&self.variables, &self.stack)); },
                    }
                },
                states::States::SetValue(variable_name, value) =>  {
                    self.variables.insert(variable_name.to_string(), object::MObject::Number(object::Number::new(value.clone())));
                },
            }
        }
    }
}