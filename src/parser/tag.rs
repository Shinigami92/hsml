use nom::{
    bytes::complete::{take_till, take_till1},
    IResult,
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

pub fn process_tag(input: &str) -> IResult<&str, &str> {
    // TODO @Shinigami92 2023-05-10: Should start with a letter
    take_till1(|c: char| c != '-' && !c.is_ascii_alphanumeric())(input)
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

    // TODO @Shinigami92 2023-05-10: Add negative tests for
    // - tag starting with a number
    // - tag starting with a special character
    // - tag starting with a space
    // - tag starting with a dot
    // - tag starting with a hash
    // - tag starting with a line ending
}
