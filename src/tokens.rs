#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i32),
    Float(f32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Main,
    Let,
    Do,
    Not,
    Include,
    Word(String),
    Colon,
    Function,
    LBracket,
    LCurlyBraces,
    LSquareBrackets,
    RBracket,
    RCurlyBraces,
    RSquareBrackets,
    Annotation(String),
    If,
    Symbol(String),
    Return,
    Number(Number),
    Semicolon,
    Throw,
    String(String),
    Class,
    Wave,
    Dot,
    Comma,
    Try,
    Catch,
    As,
    Test,
    Devide,
    This,
    Else,
    While,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    UnEqual,
    Plus,
    Minus,
    Multiply,
    Assignment,
    Elif,
    For,
}