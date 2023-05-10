use nom::{
    bytes::complete::{tag, take_till1},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ClassNode {
    pub name: String,
}

pub fn process_class(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(".")(input)?;
    take_till1(|c| c == ' ' || c == '.' || c == '(' || c == '\n')(input)
}

pub fn class_node(input: &str) -> IResult<&str, ClassNode> {
    let (input, class_name) = process_class(input)?;

    Ok((
        input,
        ClassNode {
            name: class_name.to_string(),
        },
    ))
}
