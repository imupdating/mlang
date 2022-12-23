use mlang::file;
use mlang::lexer;
use mlang::runner;
use mlang::parser;
use std::collections::HashMap;

fn main() {
    // 从文件读入源代码
    let source = file::read_all("test.ms".to_string());

    // 词法分析
    let lexer_ = lexer::Lexer::new(source);
    let tokens = lexer_.fetch_all();
    println!("{:#?}", tokens);

    // 语法分析
    let parser_ = parser::Parser::new(tokens.unwrap());
    let states = parser_.fetch_all().unwrap();
    println!("{:#?}", states);

    // 运行代码
    let mut runner = runner::Runner::new(states, HashMap::new());
    runner.run();
    println!("{:#?}", runner.stack);
}
