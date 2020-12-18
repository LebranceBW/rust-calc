use crate::lexer::{Lexer, Token, Op};
use crate::lexer;

pub(crate) fn calc(s: &str) -> String {
    match lexer::new(s) {
        Err(message) => message,
        Ok(mut lexer) => match expr(&mut lexer) {
            Ok(val) => val.to_string(),
            Err(message) => message,
        },
    }
}

fn expr(lexer: &mut Lexer) -> Result<f32, String> {
    let mut addend = term(lexer)?;
    while let Some(token) = lexer.pop() {
        match token {
            Token::Operator(Op::Add) => {
                addend += term(lexer)?;
            }
            Token::Operator(Op::Sub) => {
                addend -= term(lexer)?;
            }
            Token::RightBracket => {
                lexer.push(token);
                break;
            }
            _ => {
                return Err(format!("Invalid Token: {:?}", token));
            }
        }
    }
    Ok(addend)
}

fn term(lexer: &mut Lexer) -> Result<f32, String> {
    let mut factor = unit(lexer)?;
    while let Some(token) = lexer.pop() {
        match token {
            Token::Operator(Op::Mul) => {
                factor *= unit(lexer)?;
            }
            Token::Operator(Op::Div) => {
                factor /= unit(lexer)?;
            }
            _ => {
                lexer.push(token);
                break;
            }
        }
    }
    Ok(factor)
}

fn unit(lexer: &mut Lexer) -> Result<f32, String> {
    match lexer.pop() {
        Some(token) => match token {
            Token::Number(val) => Ok(val),
            Token::LeftBracket => {
                let val = expr(lexer)?;
                match lexer.pop() {
                    Some(Token::RightBracket) => {}
                    _ => return Err(String::from("Mismatched bracket")),
                }
                Ok(val)
            }
            _ => Err(format!("Invalid Token: {:?}", token)),
        },
        _ => Ok(0.0f32),
    }
}

