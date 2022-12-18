use mlang::file;
use mlang::lexer;
use mlang::states;
use mlang::runner;
use mlang::parser;
use std::collections::HashMap;

fn main() {
//    let source = file::read_all("test.ms".to_string());
//    let lexer_ = lexer::Lexer::new(source);
//    let tokens = lexer_.fetch_all();
//    println!("{:#?}", tokens);

    // let states = vec![states::States::NewFunction("test".to_string(), vec!["a".to_string()], vec![]),
    //      states::States::CallFunction("test".to_string())];
    
    let source = file::read_all("test.ms".to_string());
    let lexer_ = lexer::Lexer::new(source);
    let tokens = lexer_.fetch_all();
    println!("{:#?}", tokens);
    let parser_ = parser::Parser::new(tokens.unwrap());
    let mut states = parser_.fetch_all().unwrap();
    states.push(states::States::CallFunction("add".to_string()));
    println!("{:#?}", states);
    let mut runner = runner::Runner::new(states, HashMap::new());
    runner.run();
    println!("{:#?}", runner.stack);
}