use nom::{
    bytes::complete::{tag, take_till},
    IResult,
};

use crate::parser::HsmlProcessContext;

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
) -> IResult<&'a str, Vec<AttributeNode>> {
    let (mut input, _) = tag("(")(input)?;

    let mut attributes: Vec<AttributeNode> = vec![];

    // loop until `)`
    // take until attr starts (trim , and whitespace)
    // collect attr
    // if attr is empty, break
    loop {
        let (remaining, _) = take_till(|c: char| !c.is_whitespace() && c != ',')(input)?;

        if remaining.starts_with(')') {
            break;
        }

        let (remaining, attribute) = attribute_node(remaining, context)?;

        attributes.push(attribute);
        input = remaining;
    }

    let (input, _) = tag(")")(input)?;

    Ok((input, attributes))
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        attribute::node::{attribute_node, attribute_nodes, AttributeNode},
        HsmlProcessContext,
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
                AttributeNode {
                    key: String::from("key"),
                    value: Some(String::from("value"))
                },
                AttributeNode {
                    key: String::from(":key2"),
                    value: Some(String::from("value2"))
                }
            ]
        );

        assert_eq!(input, "");
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
                AttributeNode {
                    key: String::from("class"),
                    value: Some(String::from(
                        r#"{
        'is-active': isActive,
        'is-disabled': isDisabled,
    }"#
                    )),
                },
                AttributeNode {
                    key: String::from(":key"),
                    value: Some(String::from("item.id")),
                },
            ]
        );

        assert_eq!(input, "");
    }
}
