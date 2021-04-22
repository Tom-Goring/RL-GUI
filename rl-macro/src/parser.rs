#![allow(dead_code)]
use proc_macro::{Delimiter, TokenTree};
use std::collections::HashMap;

// This is for easy toggling of the hundreds of debug prints in the parser.
macro_rules! println {
    ($($rest:tt)*) => {
        #[cfg(not(debug_assertions))]
        std::println!($($rest)*)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ParseItem {
    Button,
    Text,
    Row,
    Column,
    TextInput,
}

impl From<TokenTree> for ParseItem {
    fn from(tt: TokenTree) -> Self {
        match tt {
            TokenTree::Ident(name) => match name.to_string().as_str() {
                "Button" => ParseItem::Button,
                "Text" => ParseItem::Text,
                "Row" => ParseItem::Row,
                "Column" => ParseItem::Column,
                "TextInput" => ParseItem::TextInput,
                _ => panic!("Encountered unknown token: {}, aborting", name),
            },
            _ => unimplemented!(),
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
            "TextInput" => ParseItem::TextInput,
            _ => panic!("Encountered unknown token: {}, aborting", input),
        }
    }
}

impl ToString for ParseItem {
    fn to_string(&self) -> String {
        match self {
            ParseItem::Button => "Button".into(),
            ParseItem::Text => "Text".into(),
            ParseItem::Row => "Row".into(),
            ParseItem::Column => "Column".into(),
            ParseItem::TextInput => "TextInput".into(),
        }
    }
}

#[derive(Debug)]
pub struct ParseNode {
    pub entry: Option<ParseItem>,
    pub values: HashMap<String, String>,
    pub children: Vec<ParseNode>,
}

pub fn parse(tokens: &[TokenTree]) -> (Vec<ParseNode>, usize) {
    let mut nodes: Vec<ParseNode> = Vec::new();
    let mut unpaired_nodes = 0;
    let mut last_value_name = "".to_string();
    let mut just_started_tag = false;

    let mut idx: usize = 0;

    println!("{}", tokens.len());

    println!("{:?}", tokens);

    while idx < tokens.len() {
        match &tokens[idx] {
            TokenTree::Group(_) => {}
            TokenTree::Ident(name) => {
                println!("Encountered Ident: {}", name);
                if just_started_tag {
                    let string: String = name.to_string();
                    nodes.last_mut().unwrap().entry = Some(ParseItem::from(string.as_str()));
                    just_started_tag = false;
                } else {
                    last_value_name = name.to_string().clone();
                }
            }
            TokenTree::Punct(punct) => {
                println!("Encountered Punct: {}", punct);
                match punct.as_char() {
                    '<' => {
                        println!("Encountered <, checking if it is followed by a /...");
                        if let TokenTree::Punct(punct) = tokens[idx + 1].clone() {
                            println!("Next character is a punct, checking if it is a /...");
                            if punct.as_char() == '/' {
                                println!("/ detected, pairing off node...");
                                unpaired_nodes -= 1;
                                if unpaired_nodes == 0 {
                                    return (nodes, idx);
                                }
                            }
                        } else if unpaired_nodes == 0 {
                            println!("Checking for unpaired nodes...");
                            if unpaired_nodes == 0 {
                                println!("Opening new node");
                                just_started_tag = true;
                                unpaired_nodes += 1;
                                nodes.push(ParseNode {
                                    children: vec![],
                                    entry: None,
                                    values: Default::default(),
                                });
                            }
                        } else {
                            println!("Inside an unpaired node - calculating children...");
                            let children = parse(&tokens[idx..]);
                            idx += children.1;
                            nodes.last_mut().unwrap().children.extend(children.0);
                        }
                    }
                    '>' => {
                        if unpaired_nodes == 0 {
                            return (nodes, idx);
                        } else {
                            println!("Finished opening tag");
                        }
                    }
                    '=' => {
                        println!("Encountered =, processing following value");
                        idx += 1;
                        let mut string = String::new();
                        loop {
                            println!("Checking if we have reached the end of the value:");
                            match &tokens[idx] {
                                TokenTree::Ident(ident) => {
                                    println!("Token is an ident ({}), adding to string", ident);
                                    string = [string, ident.to_string()].join("");
                                }
                                TokenTree::Punct(punct) => {
                                    println!(
                                        "Token is punct, checking if we have hit the end or not"
                                    );
                                    if punct.as_char() == ','
                                        || punct.as_char() == '>'
                                        || punct.as_char() == '/'
                                    {
                                        println!("End reached, value is {}", string);
                                        idx -= 1;
                                        break;
                                    } else {
                                        println!("Non ending punct detected, brainlessly adding it to the string");
                                        string = [string, punct.to_string()].join("");
                                    }
                                }
                                TokenTree::Literal(literal) => {
                                    println!("Token is an literal ({}), adding to string", literal);
                                    string = [string, literal.to_string()].join("");
                                }
                                TokenTree::Group(group) => {
                                    println!("Encountered delimiter, processing...");
                                    match group.delimiter() {
                                        Delimiter::Parenthesis => {
                                            let mut bracketed_number = Vec::new();
                                            for token in group.stream() {
                                                bracketed_number.push(token.to_string());
                                            }
                                            string = format!(
                                                "{}({})",
                                                string,
                                                bracketed_number.join("")
                                            );
                                        }
                                        Delimiter::Bracket => {
                                            let mut bracketed_number = Vec::new();
                                            for token in group.stream() {
                                                bracketed_number.push(token.to_string())
                                            }
                                            string = format!(
                                                "{}[{}]",
                                                string,
                                                bracketed_number.join("")
                                            );
                                        }
                                        _ => panic!(
                                            "Encountered unexpected delimiter: {:?}",
                                            group.delimiter()
                                        ),
                                    }
                                }
                            }
                            idx += 1;
                        }
                        println!("Value of {} is {}", last_value_name, string);
                        nodes
                            .last_mut()
                            .unwrap()
                            .values
                            .insert(last_value_name.clone(), string);
                    }
                    '/' => {
                        println!("Encountered /, checking if next character is >...");
                        if let TokenTree::Punct(punct) = tokens[idx + 1].clone() {
                            if punct.as_char() == '>' {
                                println!("> detected, pairing off node.");
                                unpaired_nodes -= 1;
                                if unpaired_nodes == 0 {
                                    return (nodes, idx);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            TokenTree::Literal(value) => {
                println!("Encountered Literal: {}", value);
                nodes
                    .last_mut()
                    .unwrap()
                    .values
                    .insert(last_value_name.clone(), value.to_string());
            }
        }
        idx += 1;
    }

    (nodes, tokens.len())
}
