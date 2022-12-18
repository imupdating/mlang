use std::fmt::Error;
use crate::tokens;

#[derive(Debug)]
pub struct Lexer {
    code: Vec<char>,
    pos: i32,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer { code: code.chars().collect(), pos: 0 }
    }

    fn this(&self) -> Result<char, Error> {
        if self.pos >= self.code.len() as i32 {
            return Err(Error);
        }
        Ok(self.code[self.pos as usize])
    }

    fn next(&mut self) -> Result<char, Error> {
        self.pos += 1;
        self.this()
    }

    fn forward(&mut self) {
        self.pos += 1;
    }

    pub fn fetch_all(mut self) -> Result<Vec<tokens::Token>, String> {
        let mut result: Vec<tokens::Token> = Vec::new();

        while self.pos < self.code.len() as i32 {
            let this = match self.this() {
                Ok(char) => char,
                Err(_) => return Err("[解析器主循环] 下标越界".to_string()),
            };
            
            match this {
                '\r' => { self.forward(); continue; },
                '\n' => { self.forward(); continue; },
                ' ' => { self.forward(); continue; },
                ':' => { 
                    result.push(tokens::Token::Colon);
                    self.forward(); 
                    continue;
                },
                ';' => { 
                    result.push(tokens::Token::Semicolon);
                    self.forward(); 
                    continue;
                },
                '(' => { 
                    result.push(tokens::Token::LBracket);
                    self.forward(); 
                    continue;
                },
                ')' => { 
                    result.push(tokens::Token::RBracket);
                    self.forward(); 
                    continue;
                },
                '[' => { 
                    result.push(tokens::Token::LSquareBrackets);
                    self.forward(); 
                    continue;
                },
                ']' => { 
                    result.push(tokens::Token::RSquareBrackets);
                    self.forward(); 
                    continue;
                },
                '{' => { 
                    result.push(tokens::Token::LSquareBrackets);
                    self.forward(); 
                    continue;
                },
                '}' => { 
                    result.push(tokens::Token::RSquareBrackets);
                    self.forward(); 
                    continue;
                },
                '=' => {
                    match self.next() {
                        Ok(char) => {
                            match char {
                                '=' => { result.push(tokens::Token::Equal); self.forward(); }
                                _ => result.push(tokens::Token::Assignment),
                            }
                        },
                        Err(_) => { result.push(tokens::Token::Assignment); break; },
                    }
                    continue;
                },
                '.' => { 
                    result.push(tokens::Token::Dot);
                    self.forward(); 
                    continue;
                },
                '+' => { 
                    result.push(tokens::Token::Plus);
                    self.forward(); 
                    continue;
                },
                '-' => { 
                    result.push(tokens::Token::Minus);
                    self.forward(); 
                    continue;
                },
                '*' => { 
                    result.push(tokens::Token::Multiply);
                    self.forward(); 
                    continue;
                },
                '~' => { 
                    result.push(tokens::Token::Wave);
                    self.forward(); 
                    continue;
                },
                ',' => { 
                    result.push(tokens::Token::Comma);
                    self.forward(); 
                    continue;
                },
                '/' => {
                    match self.next() {
                        Ok(char) => {
                            match char {
                                '/' => { self.parse_annotation(); },// result.push(self.parse_annotation()),
                                _ => result.push(tokens::Token::Devide),
                            }
                        }
                        Err(_) => { result.push(tokens::Token::Devide); break;},
                    }
                },
                '<' => {
                    match self.next() {
                        Ok(char) => {
                            match char {
                                '=' => { result.push(tokens::Token::LessOrEqual); self.forward(); },
                                _ => result.push(tokens::Token::Less),
                            }
                        }
                        Err(_) => { result.push(tokens::Token::Less); break; },
                    }
                    continue;
                },
                '>' => {
                    match self.next() {
                        Ok(char) => {
                            match char {
                                '=' => { result.push(tokens::Token::GreaterOrEqual); self.forward(); },
                                _ => result.push(tokens::Token::Greater),
                            }
                        }
                        Err(_) => { result.push(tokens::Token::Greater); break; },
                    }
                    continue;
                },
                '"' => {
                    result.push(self.parse_string());
                    continue;
                },
                '!' => {
                    match self.next() {
                        Ok(char) => {
                            match char {
                                '=' => { result.push(tokens::Token::UnEqual); self.forward(); },
                                _ => result.push(tokens::Token::Not),
                            }
                        }
                        Err(_) => { result.push(tokens::Token::Not); break; },
                    }
                    continue;
                },
                word if self.is_digit(word) => {
                    result.push(self.parse_number());
                    continue;
                },
                word if self.is_valid_header(word) => {
                    result.push(self.parse_word());
                    continue;
                },
                _ => {
                    println!("{:#?}", result);
                    return Err(format!("不在预料之中的字符:\"{}\"", self.this().unwrap()));
                },
            }
        }

        Ok(result)
    }

    fn is_valid_header(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_valid_body(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' || (c >= '0' && c <= '9')
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn parse_word(&mut self) -> tokens::Token {
        let mut str = String::new();
        loop {
            if self.is_valid_body(self.this().unwrap()) {
                str += &String::from(self.this().unwrap());
                self.forward();
            } else {
                return match str.as_str() {
                    "inc" => tokens::Token::Include,
                    "as" => tokens::Token::As,
                    "if" => tokens::Token::If,
                    "return" => tokens::Token::Return,
                    "throw" => tokens::Token::Throw,
                    "class" => tokens::Token::Class,
                    "this" => tokens::Token::This,
                    "catch" => tokens::Token::Catch,
                    "test" => tokens::Token::Test,
                    "while" => tokens::Token::While,
                    "fn" => tokens::Token::Function,
                    "let" => tokens::Token::Let,
                    "else" => tokens::Token::Else,
                    "elif" => tokens::Token::Elif,
                    "for" => tokens::Token::For,
                    "do" => tokens::Token::Do,
                    "main" => tokens::Token::Main,
                    _ => tokens::Token::Word(str),
                };
            }
        }
    }

    fn parse_number(&mut self) -> tokens::Token {
        let mut n:f32 = 0_f32;
        let mut decimal = false;
        let mut decimal_n = 0; // 小数点后几位
        loop {
            let c = self.this().unwrap();
            if c == '.' && !decimal {
                decimal = true;
            } else {
                if self.is_digit(c) {
                    match decimal {
                        false => {
                            n *= 10_f32;
                            n += (c as i32 - '0' as i32) as f32;
                        },
                        true => {
                            decimal_n += 1;
                            n += 0.1_f32.powi(decimal_n) * (c as i32 - '0' as i32) as f32;
                        },
                    }
                } else {
                    return match decimal {
                        false => tokens::Token::Number(
                            tokens::Number::Int(
                                n as i32
                            )
                        ),
                        true => tokens::Token::Number(
                            tokens::Number::Float(
                                n
                            )
                        ),
                    };
                }
            }
            self.forward();
        }
    }

    fn parse_annotation(&mut self) -> tokens::Token {
        let mut str = String::new();
        loop {
            if self.next().unwrap() != '\n' {
                str += &String::from(self.this().unwrap());
            } else {
                self.forward();
                return tokens::Token::Annotation(str);
            }
        }
    }

    fn parse_string(&mut self) -> tokens::Token {
        let mut str = String::new();
        loop {
            if self.next().unwrap() != '"' {
                str += &String::from(self.this().unwrap());
            } else {
                self.forward();
                return tokens::Token::String(str);
            }
        }
    }
}