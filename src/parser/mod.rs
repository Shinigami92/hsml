use nom::{character::complete::line_ending, IResult};

use self::{
    attribute::node::AttributeNode,
    class::node::ClassNode,
    tag::node::{tag_node, TagNode},
    text::node::TextNode,
};

pub mod attribute;
pub mod class;
pub mod tag;
pub mod text;

#[derive(Debug, PartialEq)]
pub struct RootNode {
    pub nodes: Vec<HsmlNode>,
}

#[derive(Debug, PartialEq)]
pub enum HsmlNode {
    Root(RootNode),
    Tag(TagNode),
    Class(ClassNode),
    Attribute(AttributeNode),
    Text(TextNode),
}

#[derive(Debug, Default)]
pub struct HsmlProcessContext {
    pub indent_level: usize,
    pub indent_string: Option<String>,
}

pub fn process_newline(input: &str) -> IResult<&str, &str> {
    line_ending(input)
}

pub fn parse(input: &str) -> IResult<&str, RootNode> {
    let mut nodes: Vec<HsmlNode> = vec![];

    let mut input = input;

    while let Ok((rest, node)) = tag_node(input) {
        nodes.push(HsmlNode::Tag(node));
        input = rest;
    }

    // println!("input: {}", input);

    Ok((input, RootNode { nodes }))
}
