mod tokenizer;
mod parse;
mod read;
mod type_check;

use tokenizer::TokenStream;
use parse::AST;

#[derive(Debug, PartialEq)]
pub enum Type {
	NumT,
	BoolT,
	FunT {arg: Box<Type>, ret: Box<Type>},
}

pub fn type_check(input_file: &str) -> Type {
    let characters = read::build(input_file).unwrap();
    let tokenizer = TokenStream::build(characters);
    let ast = AST::build(tokenizer);
    type_check::tc(ast)
    // tokenizer.for_each(|token| println!("{:?}" token));
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn type_check_1() {
    	assert_eq!(type_check("input1.txt"), Type::NumT);
    }
}
