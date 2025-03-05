use nom::{
    IResult,
    bytes::complete::{tag, take_till},
};

use crate::parser::{HsmlNode, HsmlProcessContext, comment::node::comment_dev_node};

use super::process::process_attribute;

#[derive(Debug, PartialEq, Eq)]
pub struct AttributeNode {
    pub key: String,
    pub value: Option<String>,
}

pub fn attribute_node<'a>(
    input: &'a str,
    context: &mut HsmlProcessContext,
) -> IResult<&'a str, AttributeNode> {
    let (input, attribute) = process_attribute(input, context)?;

    let equal_sign_index = attribute.find('=').unwrap_or(attribute.len());
    let (key, value) = attribute.split_at(equal_sign_index);

    // Remove surrounding quotes and leading `=` from value
    let value = value
        .strip_prefix(r#"=""#)
        .and_then(|v| v.strip_suffix('"'))
        .map(|v| v.to_string());

    Ok((
        input,
        AttributeNode {
            key: key.to_string(),
            value,
        },
    ))
}

pub fn attribute_nodes<'a>(
    input: &'a str,
    context: &mut HsmlProcessContext,
) -> IResult<&'a str, Vec<HsmlNode>> {
    let (mut input, _) = tag("(")(input)?;

    let mut nodes: Vec<HsmlNode> = vec![];

    // loop until `)`
    // take until attr starts (trim , and whitespace)
    // collect attr
    // if attr is empty, break
    loop {
        let (remaining, _) = take_till(|c: char| !c.is_whitespace() && c != ',')(input)?;

        if remaining.starts_with(')') {
            input = remaining;
            break;
        }

        // if remaining starts with `//`, it is a dev comment
        if remaining.starts_with("//") {
            let (remaining, comment) = comment_dev_node(remaining)?;
            nodes.push(HsmlNode::Comment(comment));

            input = remaining;
            continue;
        }

        let (remaining, attribute) = attribute_node(remaining, context)?;

        nodes.push(HsmlNode::Attribute(attribute));
        input = remaining;
    }

    let (input, _) = tag(")")(input)?;

    Ok((input, nodes))
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        HsmlNode, HsmlProcessContext,
        attribute::node::{AttributeNode, attribute_node, attribute_nodes},
        comment::node::CommentNode,
    };

    #[test]
    fn it_should_return_attribute_node() {
        let mut context = HsmlProcessContext::default();

        let (input, attribute) = attribute_node(r#"key="value""#, &mut context).unwrap();

        assert_eq!(
            attribute,
            AttributeNode {
                key: String::from("key"),
                value: Some(String::from("value"))
            }
        );

        assert_eq!(input, "");
    }

    #[test]
    fn it_should_return_attribute_node_with_multiline() {
        let mut context = HsmlProcessContext::default();

        let (input, attribute) = attribute_node(
            r#"class="{
        'is-active': isActive,
        'is-disabled': isDisabled,
    }"
    :key="item.id""#,
            &mut context,
        )
        .unwrap();

        assert_eq!(
            attribute,
            AttributeNode {
                key: String::from("class"),
                value: Some(String::from(
                    r#"{
        'is-active': isActive,
        'is-disabled': isDisabled,
    }"#
                ))
            }
        );

        assert_eq!(
            input,
            r#"
    :key="item.id""#
        );
    }

    #[test]
    fn it_should_return_attribute_nodes() {
        let mut context = HsmlProcessContext::default();

        let (input, attribute_nodes) =
            attribute_nodes(r#"(key="value", :key2="value2")"#, &mut context).unwrap();

        assert_eq!(
            attribute_nodes,
            vec![
                HsmlNode::Attribute(AttributeNode {
                    key: String::from("key"),
                    value: Some(String::from("value"))
                }),
                HsmlNode::Attribute(AttributeNode {
                    key: String::from(":key2"),
                    value: Some(String::from("value2"))
                })
            ]
        );

        assert_eq!(input, "");
    }

    #[test]
    fn it_should_return_attribute_nodes_with_wrapped() {
        let mut context = HsmlProcessContext::default();

        let (input, attribute_nodes) = attribute_nodes(
            r#"(
    key="value"
    :key2="value2"
)
"#,
            &mut context,
        )
        .unwrap();

        assert_eq!(
            attribute_nodes,
            vec![
                HsmlNode::Attribute(AttributeNode {
                    key: String::from("key"),
                    value: Some(String::from("value"))
                }),
                HsmlNode::Attribute(AttributeNode {
                    key: String::from(":key2"),
                    value: Some(String::from("value2"))
                })
            ]
        );

        assert_eq!(input, "\n");
    }

    #[test]
    fn it_should_return_attribute_nodes_with_dev_comments() {
        let mut context = HsmlProcessContext::default();

        let (input, attribute_nodes) = attribute_nodes(
            r#"(
    // comment 1
    key="value"
    // comment 2
    :key2="value2"
)
"#,
            &mut context,
        )
        .unwrap();

        assert_eq!(
            attribute_nodes,
            vec![
                HsmlNode::Comment(CommentNode {
                    is_dev: true,
                    text: String::from(" comment 1"),
                }),
                HsmlNode::Attribute(AttributeNode {
                    key: String::from("key"),
                    value: Some(String::from("value")),
                }),
                HsmlNode::Comment(CommentNode {
                    is_dev: true,
                    text: String::from(" comment 2"),
                }),
                HsmlNode::Attribute(AttributeNode {
                    key: String::from(":key2"),
                    value: Some(String::from("value2")),
                }),
            ]
        );

        assert_eq!(input, "\n");
    }

    #[test]
    fn it_should_return_attribute_nodes_with_multiline() {
        let mut context = HsmlProcessContext::default();

        let (input, attributes) = attribute_nodes(
            r#"(class="{
        'is-active': isActive,
        'is-disabled': isDisabled,
    }"
    :key="item.id")"#,
            &mut context,
        )
        .unwrap();

        assert_eq!(
            attributes,
            vec![
                HsmlNode::Attribute(AttributeNode {
                    key: String::from("class"),
                    value: Some(String::from(
                        r#"{
        'is-active': isActive,
        'is-disabled': isDisabled,
    }"#
                    )),
                }),
                HsmlNode::Attribute(AttributeNode {
                    key: String::from(":key"),
                    value: Some(String::from("item.id")),
                }),
            ]
        );

        assert_eq!(input, "");
    }
}
