use nom::{
    bytes::complete::{take_till, take_till1},
    error::{Error, ErrorKind},
    IResult,
};

use crate::parser::{
    attribute,
    class::node::{class_node, ClassNode},
    comment::node::{comment_dev_node, comment_native_node},
    tag::process::process_tag,
    text::{self, node::TextNode},
    HsmlNode, HsmlProcessContext,
};

#[derive(Debug, PartialEq)]
pub struct TagNode {
    pub tag: String,
    pub classes: Option<Vec<ClassNode>>,
    pub attributes: Option<Vec<HsmlNode>>,
    pub text: Option<TextNode>,
    pub children: Option<Vec<HsmlNode>>,
}

pub fn tag_node<'a>(input: &'a str, context: &mut HsmlProcessContext) -> IResult<&'a str, TagNode> {
    // tag node starts with a tag name or a dot/hash
    // if it starts with a dot/hash, the tag name is div

    let (mut input, tag_name) = if input.starts_with('.') || input.starts_with('#') {
        (input, "div")
    } else {
        process_tag(input)?
    };

    // if the next char is a dot, we have a id node
    // if the next char is a dot, we have a class node
    // collect id and class nodes until we hit a whitespace, newline, start of attributes or single dot without trailing alphabetical char

    let mut class_nodes: Vec<ClassNode> = vec![];
    let mut attribute_nodes: Option<Vec<HsmlNode>> = None;
    let mut text_node: Option<TextNode> = None;
    let mut child_nodes: Vec<HsmlNode> = vec![];

    loop {
        let first_char = input.get(..1);
        let first_two_chars = input.get(..2);

        if first_char == Some("#") {
            // we hit an id node

            todo!("id nodes are not supported yet");
        }

        if first_char == Some(".") {
            // TODO @Shinigami92 2023-05-18: Maybe we want to support piped text. That would start with a `.\n`

            // we hit a class node
            let (rest, node) = class_node(input)?;
            class_nodes.push(node);
            input = rest;

            continue;
        }

        if first_char == Some("(") {
            // we hit the start of attributes

            let (rest, nodes) = attribute::node::attribute_nodes(input, context)?;
            attribute_nodes = Some(nodes);
            input = rest;

            continue;
        }

        if first_char == Some(" ") {
            // we hit a whitespace and there should be text

            let (rest, node) = text::node::text_node(input)?;
            text_node = Some(node);
            input = rest;

            // TODO @Shinigami92 2023-05-22: Theoretically here could also follow a comment

            // there could be child tag nodes, but this will be handled in the next loop iteration by the line ending check

            continue;
        }

        if first_char == Some("\n") || first_two_chars == Some("\r\n") {
            // we hit a newline and the tag ended but could have child tag nodes

            // check indentation
            let (rest, _) = take_till1(|c| c != '\r' && c != '\n')(input)?;

            // check if the next char is a tab or whitespace
            // if yes, check for indentation level
            // if no, we have no child tag nodes and can break the loop

            let (remaining, indentation) = take_till(|c: char| !c.is_whitespace())(rest)?;

            if !indentation.is_empty() {
                // check that the indentation is consistent and does not include tabs and spaces at the same time
                // if it does, throw an error

                if indentation.contains('\t') && indentation.contains(' ') {
                    // TODO @Shinigami92 2023-05-18: This error could be more specific
                    return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)));
                }

                // if we never hit an indentation yet, set it
                // this only happens once
                if context.indent_string.is_none() {
                    // println!("set indent string = \"{}\"", indentation);
                    context.indent_string = Some(indentation.to_string());
                }

                // persist the indentation level so we can restore it later
                let indentation_level = context.indent_level;

                context.indent_level += 1;

                // check that we are at the correct indentation level, otherwise break out of the loop
                let indent_string_len = context.indent_string.as_ref().unwrap().len();
                let indent_size = indent_string_len * context.indent_level;
                // dbg!(indent_size, indentation.len());
                if indent_size != indentation.len() {
                    // dbg!("break out of loop");
                    break;
                }

                // we are at the correct indentation level, so we can continue parsing the child tag nodes

                // there could be a comment (dev or native) node
                if let Ok((rest, node)) = comment_native_node(remaining) {
                    child_nodes.push(HsmlNode::Comment(node));
                    input = rest;
                } else if let Ok((rest, node)) = comment_dev_node(remaining) {
                    child_nodes.push(HsmlNode::Comment(node));
                    input = rest;
                }
                // or we have now a child tag node
                else {
                    // now we have a child tag node
                    let (rest, node) = tag_node(remaining, context).expect("child tag node");
                    child_nodes.push(HsmlNode::Tag(node));
                    input = rest;
                }

                // restore the indentation level
                context.indent_level = indentation_level;

                continue;
            }

            // we have no child tag nodes
            break;
        }

        break;
    }

    Ok((
        input,
        TagNode {
            tag: tag_name.to_string(),
            classes: (!class_nodes.is_empty()).then_some(class_nodes),
            attributes: attribute_nodes,
            text: text_node,
            children: (!child_nodes.is_empty()).then_some(child_nodes),
        },
    ))
}
