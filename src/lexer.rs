use std::iter::{Fuse, Peekable};
use std::str::Chars;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Number(f32),
    Operator(Op),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

struct CharStream<'a> {
    index: usize,
    iterator: Peekable<Fuse<Chars<'a>>>,
}

impl<'a> CharStream<'a> {
    fn new(line: &'a str) -> CharStream<'a> {
        Self {
            index: 0,
            iterator: line.chars().fuse().peekable(),
        }
    }

    fn next(&mut self) -> char {
        self.index += 1;
        self.iterator.next().unwrap_or('\0')
    }

    fn peek(&mut self) -> char {
        self.iterator.peek().cloned().unwrap_or('\0')
    }
}

pub(crate) type Lexer = VecDeque<Token>;

pub fn new(s: &str) -> Result<Lexer, String> {
    let mut stream = CharStream::new(s);
    let mut tokens = VecDeque::new();
    loop {
        let c = stream.peek();
        if c == '\0' {
            break;
        } else if c.is_whitespace() {
            stream.next();
            continue;
        } else {
            let token = parse_token(&mut stream)?;
            tokens.push_back(token);
        }
    }
    Ok(tokens)
}

fn parse_token(stream: &mut CharStream) -> Result<Token, String> {
    const DIGITS: &str = "0123456789.";
    const SYMBOLS: &str = "+-*/";
    let mut ch = stream.peek();
    if DIGITS.contains(ch) {
        let mut buffer: String = String::new();
        while DIGITS.contains(ch) {
            buffer.push(ch);
            stream.next();
            ch = stream.peek();
        }
        match buffer.parse::<f32>() {
            Ok(val) => Ok(Token::Number(val)),
            Err(_) => Err(format!("Invalid Token. {}", buffer)),
        }
    } else if SYMBOLS.contains(ch) {
        stream.next();
        match ch {
            '+' => Ok(Token::Operator(Op::Add)),
            '-' => Ok(Token::Operator(Op::Sub)),
            '*' => Ok(Token::Operator(Op::Mul)),
            '/' => Ok(Token::Operator(Op::Div)),
            _ => Err(String::from("Impossible")),
        }
    } else if ch == '(' {
        stream.next();
        Ok(Token::LeftBracket)
    } else if ch == ')' {
        stream.next();
        Ok(Token::RightBracket)
    } else {
        Err(format!("Invalid Character: {}", ch))
    }
}
