use nom::IResult;

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

    let mid = attribute.find('=').unwrap_or_else(|| attribute.len());
    let (key, value) = attribute.split_at(mid);

    let value = value.strip_prefix('=').map(|v| v.to_string());

    Ok((
        input,
        AttributeNode {
            key: key.to_string(),
            value,
        },
    ))
}
