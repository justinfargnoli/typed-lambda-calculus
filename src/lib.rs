pub mod codegen;
pub mod parse;
mod read;
mod tokenize;
pub mod type_check;

use parse::AST;
use tokenize::TokenStream;
use type_check::Type;

pub fn type_check(input_file: &str) -> Type {
    let characters = read::build(input_file).unwrap();
    let mut tokenizer = TokenStream::build(characters);
    let ast = AST::build(&mut tokenizer);
    type_check::tc(&ast)
}

pub fn check(input_file: &str) -> Type {
    let characters = read::build(input_file).unwrap();
    let mut tokenizer = TokenStream::build(characters);
    let ast = AST::build(&mut tokenizer);
    type_check::tc(&ast)
}

pub fn compile(input_file: &str) {
    let characters = read::build(input_file).unwrap();
    let mut tokenizer = TokenStream::build(characters);
    let _ast = AST::build(&mut tokenizer);
}
