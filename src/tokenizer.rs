use std::io::{Bytes, Read};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    ParenLeft,
    ParenRight,
    Comma,
    Quote,
    ID(String),
    Number(u32),
    TTrue,
    TFalse,
    TNumC,
    TPlusC,
    TMultC,
    TIfC,
    TIdC,
    TAppC,
    TFdC,
    TType(String)
}

#[derive(Debug)]
pub struct TokenStream<T: Read> {
	character_stream: Peekable<Bytes<T>>, 
}

impl<T: Read> TokenStream<T> {
	pub fn build(character_stream: Peekable<Bytes<T>>) -> TokenStream<T> {
		TokenStream {
			character_stream,
		}
	}
}

impl<T: Read> Iterator for TokenStream<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        unimplemented!()
    }
}

