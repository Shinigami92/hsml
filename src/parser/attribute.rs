use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct AttributeNode {
    pub key: String,
    pub value: Option<String>,
}

pub fn attribute_node(input: &str) -> IResult<&str, AttributeNode> {
    // let (input, _) = take_till(":")(input)?;

    Ok((
        input,
        AttributeNode {
            key: "".to_owned(),
            value: None,
        },
    ))
}
