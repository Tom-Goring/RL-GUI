mod parser;

extern crate proc_macro;
use crate::parser::{parse, ParseNode};
use proc_macro::{TokenStream, TokenTree};

// I'm pretty sure this entire thing is a string allocation nightmare

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    // println!("{}", input);
    let tokens = input.into_iter().collect::<Vec<TokenTree>>();
    let result = parse(&tokens);

    // println!("{:?}", result.0);

    let code = process_node(&result.0.first().unwrap());

    // println!("{}", code);

    code.parse().unwrap()
}

fn process_node(node: &ParseNode) -> String {
    let name = node.entry.expect("Encountered a node without a type");
    let children = &node.children;
    let values = &node.values;

    let strings: Vec<String> = children.iter().map(|child| process_node(&child)).collect();

    let code = match name.to_string().as_str() {
        "Row" => {
            let strings = strings.join(", "); // We have a number of children that need to be inserted into a vec
            format!(
                "Row::with_children(vec![{}]).padding({}).into()",
                strings,
                values.get("padding").unwrap_or(&String::from("0.0"))
            )
        }
        "Button" => format!(
            "Button::new(&mut {}, {}, {}, {}).into()",
            values
                .get("state")
                .expect("No state found for button")
                .replace('"', ""),
            strings[0],
            values
                .get("on_press")
                .map_or(String::from("None"), |message| format!("Some({})", message)),
            values
                .get("color")
                // .map_or(String::from("None"), |color| format!("Some({})", color)),
                .expect("Button is missing a background!")
        ),
        "Text" => format!(
            "Text::new({}, {}).into()",
            values.get("value").unwrap_or(&String::from("")),
            values
                .get("size")
                .map_or(String::from("None"), |size| format!("Some({})", size)),
        ),
        "Column" => {
            let strings = strings.join(", "); // We have a number of children that need to be inserted into a vec
            format!(
                "Column::with_children(vec![{}]).padding({}).into()",
                strings,
                values.get("padding").unwrap_or(&String::from("0.0"))
            )
        }
        "TextInput" => format!(
            "TextInput::new(&mut {}, {}, {}, {}).into()",
            values.get("state").expect("No state found for text input"),
            values.get("placeholder").unwrap_or(&String::from("")),
            values.get("value").unwrap_or(&String::from("")),
            values
                .get("on_change")
                .expect("No on_change found for text input")
        ),
        _ => "".into(),
    };

    code
}
