use nom::{bytes::complete::take_till, IResult};

use crate::parser::{
    attribute::node::{attribute_nodes, AttributeNode},
    class::node::{class_node, ClassNode},
    tag::process::process_tag,
    text::node::{text_node, TextNode},
    HsmlProcessContext,
};

#[derive(Debug, PartialEq)]
pub struct TagNode {
    pub tag: String,
    pub classes: Option<Vec<ClassNode>>,
    pub attributes: Option<Vec<AttributeNode>>,
    pub text: Option<TextNode>,
    pub children: Option<Vec<TagNode>>,
}

pub fn tag_node<'a>(input: &'a str, context: &mut HsmlProcessContext) -> IResult<&'a str, TagNode> {
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

    let (input, _prev) = take_till(|c| c == ' ' || c == '(' || c == '\n' || c == '\r')(input)?;

    let (mut input, attribute_nodes): (&str, Option<Vec<AttributeNode>>) = if input.starts_with('(')
    {
        let (input, nodes) = attribute_nodes(input, context)?;
        (input, Some(nodes))
    } else {
        (input, None)
    };

    let text_node: Option<TextNode> = if let Ok((rest, node)) = text_node(input) {
        input = rest;
        Some(node)
    } else {
        None
    };

    let mut children: Vec<TagNode> = vec![];

    while let Ok((rest, node)) = tag_node(input, context) {
        children.push(node);
        input = rest;
    }

    Ok((
        input,
        TagNode {
            tag: tag_name.to_string(),
            classes: (!class_nodes.is_empty()).then_some(class_nodes),
            attributes: attribute_nodes,
            text: text_node,
            children: (!children.is_empty()).then_some(children),
        },
    ))
}
