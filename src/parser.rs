use std::fmt::Error;
use crate::tokens::Token;
use crate::states::{States, self};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    fn this(&self) -> Result<&Token, Error> {
        if self.pos >= self.tokens.len() as i32 {
            return Err(Error);
        }
        Ok(&self.tokens[self.pos as usize])
    }

    fn forward(&mut self) {
        self.pos += 1;
    }

    pub fn fetch_all(mut self) -> Result<Vec<States>, String> {
        let mut result: Vec<States> = Vec::new();

        while self.pos < self.tokens.len() as i32 {
            let this = match self.this() {
                Ok(token) => token,
                Err(_) => return Err("[解释器主循环] 下标越界".to_string()),
            };
            
            match *this {
                Token::Function => {
                    result.push(self.function());
                },
                Token::Let => {
                    result.push(self.set_value());
                },
                _ => {
                    println!("{:#?}", result);
                    return Err(format!("不在预料之中的词:\"{:#?}\"", self.this().unwrap()));
                },
            }
        }

        Ok(result)
    }

    fn set_value(&mut self) -> States {
        self.forward(); // Token::Let

        let mut variable_name: String = String::new();
        if let Token::Word(name) = self.this().unwrap() {
            variable_name = name.to_string();
        }

        self.forward(); // skip this
        self.forward(); // Token::Assignment
        
        let value;
        if let Token::Number(v) = self.this().unwrap() {
            value = v.clone();
        } else { panic!(); }

        self.forward(); // skip this
        self.forward(); // Token::Semicolon

        States::SetValue(variable_name, value)
    }

    fn function(&mut self) -> States {
        self.forward(); // Token::Function

        let mut args: Vec<String> = Vec::new();
        let mut function_name_global: String = String::new();
        if let Token::Word(function_name) = self.this().unwrap() {
            function_name_global = function_name.to_string();
        }
        self.forward(); // skip this function name

        // (arg1, arg2...)
        self.forward(); // Token::LBracket
        if !matches!(self.this().unwrap(), Token::RBracket) {
            loop {
                let token = self.this().unwrap();
                if let Token::Word(s) = token {
                    args.push(s.to_string());
                }
                self.forward();

                if !matches!(self.this().unwrap(), Token::Comma) {
                    self.forward(); // Token::RBracket
                    break;
                }

                self.forward();
            }
        } else {
            self.forward(); // Token::RBracket
        }

        let states = self.block();

        States::NewFunction(function_name_global, args, states)
    }

    fn return_(&mut self) -> States {
        self.forward(); // Token::Return

        let var = self.this().unwrap().clone();
        self.forward(); // Token::Semicolon
        
        States::Return(states::Expr::new(vec![var]))
    }

    fn block(&mut self) -> Vec<States> {
        self.forward(); // Token::LSquareBrackets

        let mut result: Vec<States> = Vec::new();
        
        loop {
            let this = self.this().unwrap();

            if matches!(this, Token::RSquareBrackets) {
                self.forward(); // Token::RSquareBrackets
                break;
            }

            match this {
                Token::Function => {
                    result.push(self.function());
                },
                Token::Return => {
                    result.push(self.return_());
                },
                _ => {
                    println!("{:#?}", result);
                    panic!("不在预料之中的词:\"{:#?}\"", this);
                },
            }

            self.forward();
        }

        result
    }
}
