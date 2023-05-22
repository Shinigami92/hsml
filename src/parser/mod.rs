use nom::{bytes::complete::take_till, character::complete::line_ending, IResult};

use self::{
    attribute::node::AttributeNode,
    class::node::ClassNode,
    comment::node::{comment_dev_node, comment_native_node, CommentNode},
    tag::node::{tag_node, TagNode},
    text::node::TextNode,
};

pub mod attribute;
pub mod class;
pub mod comment;
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
    Comment(CommentNode),
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

    let mut context = HsmlProcessContext::default();

    let mut input = input;

    loop {
        // eat leading and trailing newlines and whitespace if there are any
        if let Ok((rest, _)) =
            take_till::<_, &str, nom::error::Error<&str>>(|c: char| !c.is_whitespace())(input)
        {
            input = rest;

            if input.is_empty() {
                break;
            }
        }

        if let Ok((rest, node)) = comment_native_node(input) {
            nodes.push(HsmlNode::Comment(node));
            input = rest;
            continue;
        }

        if let Ok((rest, node)) = comment_dev_node(input) {
            nodes.push(HsmlNode::Comment(node));
            input = rest;
            continue;
        }

        match tag_node(input, &mut context) {
            Ok((rest, node)) => {
                nodes.push(HsmlNode::Tag(node));
                input = rest;
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }

        // TODO @Shinigami92 2023-05-18: Add support for doctype node
        // TODO @Shinigami92 2023-05-18: Add support for comment nodes
    }

    Ok((input, RootNode { nodes }))
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        attribute::node::AttributeNode, class::node::ClassNode, tag::node::TagNode,
        text::node::TextNode, HsmlNode, RootNode,
    };

    use super::parse;

    #[test]
    fn it_should_parse() {
        let input = "h1.text-red Vite CJS Faker Demo
  .card
    .card__image
      img(:src=\"natureImageUrl\" :alt=\"'Background image for ' + fullName\")
    .card__profile
      img(:src=\"avatarUrl\" :alt=\"'Avatar image of ' + fullName\")
    .card__body {{ fullName }}
";

        let (input, root_node) = parse(input).unwrap();

        assert_eq!(
            root_node,
            RootNode {
                nodes: vec![HsmlNode::Tag(TagNode {
                    tag: String::from("h1"),
                    classes: Some(vec![ClassNode {
                        name: String::from("text-red"),
                    }]),
                    attributes: None,
                    text: Some(TextNode {
                        text: String::from("Vite CJS Faker Demo"),
                    }),
                    children: Some(vec![HsmlNode::Tag(TagNode {
                        tag: String::from("div"),
                        classes: Some(vec![ClassNode {
                            name: String::from("card"),
                        }]),
                        attributes: None,
                        text: None,
                        children: Some(vec![
                            HsmlNode::Tag(TagNode {
                                tag: String::from("div"),
                                classes: Some(vec![ClassNode {
                                    name: String::from("card__image"),
                                }]),
                                attributes: None,
                                text: None,
                                children: Some(vec![HsmlNode::Tag(TagNode {
                                    tag: String::from("img"),
                                    classes: None,
                                    attributes: Some(vec![
                                        AttributeNode {
                                            key: String::from(":src"),
                                            value: Some(String::from("natureImageUrl")),
                                        },
                                        AttributeNode {
                                            key: String::from(":alt"),
                                            value: Some(String::from(
                                                "'Background image for ' + fullName"
                                            )),
                                        },
                                    ]),
                                    text: None,
                                    children: None,
                                })]),
                            }),
                            HsmlNode::Tag(TagNode {
                                tag: String::from("div"),
                                classes: Some(vec![ClassNode {
                                    name: String::from("card__profile"),
                                }]),
                                attributes: None,
                                text: None,
                                children: Some(vec![HsmlNode::Tag(TagNode {
                                    tag: String::from("img"),
                                    classes: None,
                                    attributes: Some(vec![
                                        AttributeNode {
                                            key: String::from(":src"),
                                            value: Some(String::from("avatarUrl")),
                                        },
                                        AttributeNode {
                                            key: String::from(":alt"),
                                            value: Some(String::from(
                                                "'Avatar image of ' + fullName"
                                            )),
                                        },
                                    ]),
                                    text: None,
                                    children: None,
                                })]),
                            }),
                            HsmlNode::Tag(TagNode {
                                tag: String::from("div"),
                                classes: Some(vec![ClassNode {
                                    name: String::from("card__body"),
                                }]),
                                attributes: None,
                                text: Some(TextNode {
                                    text: String::from("{{ fullName }}"),
                                }),
                                children: None,
                            })
                        ]),
                    })]),
                })],
            }
        );

        assert_eq!(input, "");
    }
}
