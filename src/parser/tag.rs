use nom::{
    bytes::complete::{take_till, take_till1},
    IResult, Needed,
};

use crate::parser::{class::class_node, text::text_node};

use super::{attribute::AttributeNode, class::ClassNode, text::TextNode};

#[derive(Debug, PartialEq)]
pub struct TagNode {
    pub tag: String,
    pub classes: Option<Vec<ClassNode>>,
    pub attributes: Option<Vec<AttributeNode>>,
    pub text: Option<TextNode>,
    pub children: Option<Vec<TagNode>>,
}

fn starts_with_ascii_alphabetic(s: &str) -> bool {
    if let Some(c) = s.chars().next() {
        c.is_ascii_alphabetic()
    } else {
        false
    }
}

pub fn process_tag(input: &str) -> IResult<&str, &str> {
    let (input, tag_name) = take_till1(|c: char| c != '-' && !c.is_ascii_alphanumeric())(input)?;

    if starts_with_ascii_alphabetic(tag_name) {
        Ok((input, tag_name))
    } else {
        Err(nom::Err::Incomplete(Needed::Unknown))
    }
}

pub fn tag_node(input: &str) -> IResult<&str, TagNode> {
    let (_, prev) = take_till(|c: char| c.is_ascii_alphabetic())(input)?;

    let (input, tag_name) = if !prev.ends_with('.') {
        process_tag(input)?
    } else {
        let (_, input) = input.split_at(prev.len() - 1);
        (input, "div")
    };

    let (mut input, _) = take_till(|c| c == ' ' || c == '.' || c == '(')(input)?;

    let mut class_nodes: Vec<ClassNode> = vec![];

    while let Ok((rest, node)) = class_node(input) {
        class_nodes.push(node);
        input = rest;
    }

    let (mut input, _prev) = take_till(|c| c == ' ' || c == '(' || c == '\n' || c == '\r')(input)?;

    let mut attribute_nodes: Vec<AttributeNode> = vec![];

    println!("input: '{}'", input);

    // while let Ok((rest, node)) = attribute_node(input) {
    //     attribute_nodes.push(node);
    //     input = rest;
    // }

    let text_node: Option<TextNode> = if let Ok((rest, node)) = text_node(input) {
        input = rest;
        Some(node)
    } else {
        None
    };

    let mut children: Vec<TagNode> = vec![];

    while let Ok((rest, node)) = tag_node(input) {
        children.push(node);
        input = rest;
    }

    Ok((
        input,
        TagNode {
            tag: tag_name.to_string(),
            classes: (!class_nodes.is_empty()).then_some(class_nodes),
            attributes: (!attribute_nodes.is_empty()).then_some(attribute_nodes),
            text: text_node,
            children: (!children.is_empty()).then_some(children),
        },
    ))
}

#[cfg(test)]
mod tests {
    use nom::{
        error::{Error, ErrorKind},
        Needed,
    };

    use crate::parser::tag::process_tag;

    #[test]
    fn it_should_process_tag_div_with_text() {
        let input = "div Text";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "div");
        assert_eq!(rest, " Text");
    }

    #[test]
    fn it_should_process_tag_h1_with_text() {
        let input = "h1 Text";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "h1");
        assert_eq!(rest, " Text");
    }

    #[test]
    fn it_should_process_tag_with_id() {
        let input = "input#name";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "input");
        assert_eq!(rest, "#name");
    }

    #[test]
    fn it_should_process_tag_with_class() {
        let input = "p.text-red";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "p");
        assert_eq!(rest, ".text-red");
    }

    #[test]
    fn it_should_process_tag_with_attribute() {
        let input = "p()";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "p");
        assert_eq!(rest, "()");
    }

    #[test]
    fn it_should_process_tag_without_content() {
        let input = "span\n";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "span");
        assert_eq!(rest, "\n");
    }

    #[test]
    fn it_should_process_tag_pascal_case() {
        let input = "CInput.input";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "CInput");
        assert_eq!(rest, ".input");
    }

    #[test]
    fn it_should_process_tag_kebab_case() {
        let input = "c-input.input";

        let (rest, tag) = process_tag(input).unwrap();

        assert_eq!(tag, "c-input");
        assert_eq!(rest, ".input");
    }

    // Negative tests

    #[test]
    fn it_should_not_process_tag_with_number() {
        let input = "42.input";

        assert_eq!(
            Err(nom::Err::Incomplete(Needed::Unknown)),
            process_tag(input)
        );
    }

    #[test]
    fn it_should_not_process_tag_with_special_character() {
        let input = "$span.input";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "$span.input",
                code: ErrorKind::TakeTill1
            })),
            process_tag(input)
        );

        let input = "]span.input";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "]span.input",
                code: ErrorKind::TakeTill1
            })),
            process_tag(input)
        );

        let input = ")span.input";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: ")span.input",
                code: ErrorKind::TakeTill1
            })),
            process_tag(input)
        );
    }

    #[test]
    fn it_should_not_process_tag_with_whitespace() {
        let input = " span.input";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: " span.input",
                code: ErrorKind::TakeTill1
            })),
            process_tag(input)
        );
    }

    #[test]
    fn it_should_not_process_tag_with_dot() {
        let input = ".span.input";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: ".span.input",
                code: ErrorKind::TakeTill1
            })),
            process_tag(input)
        );
    }

    #[test]
    fn it_should_not_process_tag_with_hash() {
        let input = "#span.input";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "#span.input",
                code: ErrorKind::TakeTill1
            })),
            process_tag(input)
        );
    }

    #[test]
    fn it_should_not_process_tag_with_line_ending() {
        let input = "\nspan.input";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "\nspan.input",
                code: ErrorKind::TakeTill1
            })),
            process_tag(input)
        );
    }
}
