use std::{env, fs};

#[derive(Debug, PartialEq)]
struct TagToken {
    name: String,
}

#[derive(Debug, PartialEq)]
struct TextToken {
    content: String,
}

#[derive(Debug, PartialEq)]
struct ClassToken {
    name: String,
}

#[derive(Debug, PartialEq)]
enum AttributeTokenValue {
    String(String),
    Expression(bool),
}

#[derive(Debug, PartialEq)]
struct AttributeToken {
    name: String,
    val: AttributeTokenValue,
}

#[derive(Debug, PartialEq)]
struct EndAttributesToken {}

#[derive(Debug, PartialEq)]
struct StartAttributesToken {}

#[derive(Debug, PartialEq)]
struct EosToken {}

#[derive(Debug, PartialEq)]
struct IndentToken {
    level: u8,
}

#[derive(Debug, PartialEq)]
struct OutdentToken {
    // val: u8,
}

#[derive(Debug, PartialEq)]
struct NewlineToken {}

#[derive(Debug, PartialEq)]
enum HsmlToken {
    Attribute(AttributeToken),
    Class(ClassToken),
    EndAttributes(EndAttributesToken),
    Eos(EosToken),
    Indent(IndentToken),
    Newline(NewlineToken),
    Outdent(OutdentToken),
    StartAttributes(StartAttributesToken),
    Tag(TagToken),
    Text(TextToken),
}

struct Context {
    tokens: Vec<HsmlToken>,
    content: String,
    content_len: usize,
    chars: Vec<char>,
    cursor: usize,
    indent_level: u8,
    indent_string: Option<String>,
}

fn process_tag_token(context: &mut Context) {
    let mut tag_name = String::new();

    while context.cursor < context.content_len {
        let current_char = context.chars[context.cursor];

        match current_char {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                tag_name.push(current_char);
            }
            _ => {
                break;
            }
        }

        context.cursor += 1;
    }

    context.tokens.push(HsmlToken::Tag(TagToken {
        name: tag_name.to_string(),
    }));

    // check what comes next
    if context.cursor < context.content_len {
        let next_char = context.chars[context.cursor];

        match next_char {
            // if it is a newline, then process newline token
            '\n' | '\r' => process_newline_token(context),
            // if it is a dot, then process class token
            '.' => process_class_token(context),
            // if it is a hash, then process id token
            // if it is a whitespace, then process text token
            ' ' => process_text_token(context),
            _ => {}
        }
    }
}

fn process_class_token(context: &mut Context) {
    let mut class_name = String::new();

    if context.chars[context.cursor] != '.' {
        panic!("Expected a dot");
    }

    // skip the dot
    context.cursor += 1;

    while context.cursor < context.content_len {
        let current_char = context.chars[context.cursor];

        match current_char {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                class_name.push(current_char);
            }
            _ => {
                break;
            }
        }

        context.cursor += 1;
    }

    context.tokens.push(HsmlToken::Class(ClassToken {
        name: class_name.to_string(),
    }));

    // check what comes next
    if context.cursor < context.content_len {
        let next_char = context.chars[context.cursor];

        match next_char {
            // if it is a newline, then process newline token
            '\n' | '\r' => process_newline_token(context),
            // if it is a dot, then process class token
            '.' => process_class_token(context),
            // if it is a hash, then process id token
            // if it is a whitespace, then process text token
            ' ' => process_text_token(context),
            _ => {}
        }
    }
}

fn process_newline_token(context: &mut Context) {
    while context.cursor < context.content_len {
        let current_char = context.chars[context.cursor];

        match current_char {
            '\n' => {
                context.tokens.push(HsmlToken::Newline(NewlineToken {}));
            }
            '\r' => match context.chars[context.cursor + 1] {
                '\n' => {
                    context.tokens.push(HsmlToken::Newline(NewlineToken {}));
                    context.cursor += 1;
                }
                _ => {
                    break;
                }
            },
            _ => {
                break;
            }
        }

        context.cursor += 1;
    }

    // check what comes next
    if context.cursor < context.content_len {
        let next_char = context.chars[context.cursor];

        match next_char {
            '.' => process_class_token(context),
            ' ' | '\t' => process_indent_token(context),
            _ => {}
        }
    }
}

fn process_indent_token(context: &mut Context) {
    if context.indent_string.is_none() {
        // find the indent string
        let mut indent_string = String::new();

        let mut cursor = context.cursor;

        while cursor < context.content_len {
            let current_char = context.chars[cursor];

            match current_char {
                ' ' | '\t' => indent_string.push(current_char),
                _ => break,
            }

            cursor += 1;
        }

        context.indent_string = Some(indent_string);
    }

    let indent_string = context.indent_string.as_ref().unwrap();
    let indent_chars: Vec<char> = indent_string.chars().collect();

    let mut indent_level = 0;
    let mut counter = 0;

    while context.cursor < context.content_len {
        let current_char = context.chars[context.cursor];

        match current_char {
            ' ' | '\t' => {
                // panic if indent char is not the same as indent string
                if current_char != indent_chars[counter] {
                    panic!("Indentation is not consistent");
                }

                counter += 1;
                if counter == indent_string.len() {
                    indent_level += 1;
                    counter = 0;
                }
                context.cursor += 1;
            }
            _ => {
                break;
            }
        }
    }

    context.tokens.push(HsmlToken::Indent(IndentToken {
        level: indent_level,
    }));

    // check what comes next
    if context.cursor < context.content_len {
        let next_char = context.chars[context.cursor];

        match next_char {
            // if it is a dot, then process class token
            '.' => process_class_token(context),
            // if it is a hash, then process id token
            // if it is a whitespace, then process text token
            ' ' => process_text_token(context),
            _ => {}
        }
    }
}

fn process_text_token(context: &mut Context) {
    let mut text_content = String::new();

    while context.cursor < context.content_len {
        let current_char = context.chars[context.cursor];

        match current_char {
            ' ' | 'A'..='Z' | 'a'..='z' | '0'..='9' => {
                text_content.push(current_char);
            }
            _ => {
                break;
            }
        }

        context.cursor += 1;
    }

    context.tokens.push(HsmlToken::Text(TextToken {
        content: text_content.trim().to_string(),
    }));
}

fn get_tokens(content: &str) -> Result<Vec<HsmlToken>, &'static str> {
    let mut context = Context {
        tokens: vec![],
        content: content.to_string(),
        content_len: content.len(),
        chars: content.chars().collect(),
        cursor: 0,
        indent_level: 0,
        indent_string: None,
    };

    // Identify first character
    // If it is a dot, then it is a class
    // If it is a hash, then it is an id
    // If it is a letter, then it is a tag

    while context.cursor < context.content_len {
        let current_char = context.chars[context.cursor];

        match current_char {
            // match tag
            'A'..='Z' | 'a'..='z' => process_tag_token(&mut context),
            // match class
            '.' => process_class_token(&mut context),
            // match id
            '#' => {}
            // match newline
            '\n' | '\r' => process_newline_token(&mut context),
            // match indent
            ' ' | '\t' => {}
            // match anything else
            _ => {
                // return Err("Invalid character");
            }
        }

        context.cursor += 1;
    }

    context.tokens.push(HsmlToken::Eos(EosToken {}));

    Ok(context.tokens)
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let path = &args[1];

    // let content = fs::read_to_string(path).unwrap();
    let content = "h1 Vite CJS Faker Demo\n
  .card\n
    .card__image\n
      img(:src=\"natureImageUrl\" :alt=\"'Background image for ' + fullName\")\n
    .card__profile\n
      img(:src=\"avatarUrl\" :alt=\"'Avatar image of ' + fullName\")\n
    .card__body {{ fullName }}\n
";

    let tokens = get_tokens(content).unwrap();

    println!("{:?}", tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let content = "h1 Vite CJS Faker Demo\n
  .card\n
    .card__image\n
      img(:src=\"natureImageUrl\" :alt=\"'Background image for ' + fullName\")\n
    .card__profile\n
      img(:src=\"avatarUrl\" :alt=\"'Avatar image of ' + fullName\")\n
    .card__body {{ fullName }}\n
";

        let tokens = super::get_tokens(content).unwrap();

        assert_eq!(
            tokens[0],
            super::HsmlToken::Tag(super::TagToken {
                name: "h1".to_string()
            })
        );
        assert_eq!(
            tokens[1],
            super::HsmlToken::Text(super::TextToken {
                content: "Vite CJS Faker Demo".to_string()
            })
        );
        assert_eq!(tokens[2], super::HsmlToken::Newline(super::NewlineToken {}));
        assert_eq!(
            tokens[3],
            super::HsmlToken::Class(super::ClassToken {
                name: "card".to_string()
            })
        );
        assert_eq!(
            tokens[4],
            super::HsmlToken::Indent(super::IndentToken { level: 1 })
        );
        assert_eq!(tokens.len(), 25);
    }
}
