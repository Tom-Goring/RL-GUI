#![allow(dead_code)]
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexer_test() {
        let string = r#"<Button><Text value="Hello There"/></Button>"#;
        lex(string).unwrap();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GrammarItem {
    TagBegin, // <
    TagEnd,   // >
    TagClose, // </
    Equals,
    NodeName(String),
    AttributeName(String),
    AttributeValue(String),
}

pub fn lex(input: &str) -> Result<Vec<GrammarItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '<' => {
                it.next();
                if it.peek() == Some(&'/') {
                    it.next();
                    result.push(GrammarItem::TagClose);
                } else {
                    result.push(GrammarItem::TagBegin);
                }
                let node_name = it.peeking_take_while(|c| *c != ' ' && *c != '>').collect();
                result.push(GrammarItem::NodeName(node_name));
            }
            '/' => {
                it.next();
                if it.peek() == Some(&'>') {
                    it.next();
                    result.push(GrammarItem::TagClose);
                }
            }
            '>' => {
                it.next();
                result.push(GrammarItem::TagEnd);
            }
            '=' => {
                it.next();
                result.push(GrammarItem::Equals);
            }
            '"' => {
                it.next();
                let word = it.peeking_take_while(|c| *c != '"').collect::<String>();
                result.push(GrammarItem::AttributeValue(word));
                it.next();
            }
            'A'..='Z' | 'a'..='z' => {
                let word = it
                    .peeking_take_while(|c| c.is_ascii_alphabetic())
                    .collect::<String>();
                result.push(GrammarItem::AttributeName(word));
            }
            ' ' | '\n' => {
                it.next();
            }
            _ => return Err(format!("unexpected character {}", c)),
        }
    }
    Ok(result)
}
