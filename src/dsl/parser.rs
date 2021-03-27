#![allow(dead_code)]
use crate::dsl::lexer::GrammarItem;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum ParseItem {
    Button,
    Text,
    Row,
    Column,
}

impl From<&GrammarItem> for ParseItem {
    fn from(token: &GrammarItem) -> Self {
        match token {
            GrammarItem::NodeName(name) => match name.as_str() {
                "Button" => ParseItem::Button,
                "Text" => ParseItem::Text,
                "Row" => ParseItem::Row,
                "Column" => ParseItem::Column,
                _ => {
                    unimplemented!()
                }
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

impl From<&str> for ParseItem {
    fn from(input: &str) -> Self {
        match input {
            "Button" => ParseItem::Button,
            "Text" => ParseItem::Text,
            "Row" => ParseItem::Row,
            "Column" => ParseItem::Column,
            _ => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: Option<ParseItem>,
    pub values: HashMap<String, String>,
}

pub fn parse(tokens: &[GrammarItem]) -> (Vec<ParseNode>, usize) {
    println!("{:?}", tokens);
    let mut nodes = Vec::new();
    let mut unpaired_nodes = 0;
    let mut last_value_name = "".to_string();

    let mut idx: usize = 0;

    while idx < tokens.len() {
        match &tokens[idx] {
            GrammarItem::TagBegin => {
                if unpaired_nodes == 0 {
                    unpaired_nodes += 1;
                    nodes.push(ParseNode {
                        children: vec![],
                        entry: None,
                        values: HashMap::new(),
                    });
                } else {
                    let children = parse(&tokens[idx..]);
                    idx += children.1;
                    nodes.last_mut().unwrap().children = children.0;
                }
            }
            GrammarItem::TagEnd => {
                if unpaired_nodes == 0 {
                    return (nodes, idx);
                }
            }
            GrammarItem::TagClose => {
                unpaired_nodes -= 1;
                if unpaired_nodes == 0 {
                    return (nodes, idx);
                }
            }
            GrammarItem::Equals => {}
            GrammarItem::NodeName(name) => {
                nodes.last_mut().unwrap().entry = Some(ParseItem::from(name.as_str()));
            }
            GrammarItem::AttributeName(name) => {
                last_value_name = name.clone();
            }
            GrammarItem::AttributeValue(value) => {
                nodes
                    .last_mut()
                    .unwrap()
                    .values
                    .insert(last_value_name.clone(), value.clone());
            }
        }
        idx += 1;
    }

    (nodes, tokens.len())
}

#[cfg(test)]
mod tests {
    use crate::dsl::lexer::lex;
    use crate::dsl::parser::parse;

    #[test]
    fn parser_test() {
        let string = r#"<Button><Text value="Hello There"/></Button>"#;
        let tokens = lex(string).unwrap();
        let parse_tree = parse(&tokens);
        println!("{:?}", x);
    }
}
