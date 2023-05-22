use nom::{bytes::complete::take_till, IResult};

use super::{
    comment::node::{comment_dev_node, comment_native_node},
    tag::node::tag_node,
    HsmlNode, HsmlProcessContext, RootNode,
};

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
        attribute::node::AttributeNode, class::node::ClassNode, comment::node::CommentNode,
        parse::parse, tag::node::TagNode, text::node::TextNode, HsmlNode, RootNode,
    };

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
                nodes: vec![
                    HsmlNode::Tag(TagNode {
                        tag: String::from("h1"),
                        classes: Some(vec![ClassNode {
                            name: String::from("text-red")
                        }]),
                        attributes: None,
                        text: Some(TextNode {
                            text: String::from("Vite CJS Faker Demo"),
                        }),
                        children: None,
                    }),
                    HsmlNode::Tag(TagNode {
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
                    }),
                ],
            }
        );

        assert_eq!(input, "");
    }

    #[test]
    fn it_should_parse_with_comments() {
        let input = "// this is a root dev comment
//! this is a root native comment (will get rendered)
div
    // this is a child comment
    p another tag
    //! this is a child comment that gets rendered
";

        let (input, root_node) = parse(input).unwrap();

        assert_eq!(
            root_node,
            RootNode {
                nodes: vec![
                    HsmlNode::Comment(CommentNode {
                        text: String::from(" this is a root dev comment"),
                        is_dev: true,
                    }),
                    HsmlNode::Comment(CommentNode {
                        text: String::from(" this is a root native comment (will get rendered)"),
                        is_dev: false,
                    }),
                    HsmlNode::Tag(TagNode {
                        tag: String::from("div"),
                        classes: None,
                        attributes: None,
                        text: None,
                        children: Some(vec![
                            HsmlNode::Comment(CommentNode {
                                text: String::from(" this is a child comment"),
                                is_dev: true,
                            }),
                            HsmlNode::Tag(TagNode {
                                tag: String::from("p"),
                                classes: None,
                                attributes: None,
                                text: Some(TextNode {
                                    text: String::from("another tag")
                                }),
                                children: None,
                            }),
                            HsmlNode::Comment(CommentNode {
                                text: String::from(" this is a child comment that gets rendered"),
                                is_dev: false,
                            })
                        ])
                    })
                ]
            }
        );

        assert_eq!(input, "");
    }
}
