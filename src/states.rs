use std::collections::HashMap;
use crate::tokens;
use crate::object;
use crate::tokens::Number;

#[derive(Debug, Clone)]
pub struct Expr {
    tokens: Vec<crate::tokens::Token>
}

impl Expr {
    pub fn new(tokens: Vec<crate::tokens::Token>) -> Expr {
        Expr { tokens }
    }

    pub fn value(&mut self, variables: HashMap::<String, object::MObject>) -> crate::object::MObject {
        let token = self.tokens.get(0).unwrap();
        if let tokens::Token::Word(name) = token {
            return match variables.get(name) {
                Some(v) => v.clone(),
                None => panic!("你在访问不存在的变量:{}", name),
            }
        }
        panic!()
    }
}

#[derive(Debug, Clone)]
pub enum States {
    NewFunction(String, Vec<String>, Vec<States>),
    If(Expr, Vec<States>),
    Return(Expr),
    CallFunction(String),
    SetValue(String, Number),
}