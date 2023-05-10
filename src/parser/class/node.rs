use nom::IResult;

use super::process::process_class;

#[derive(Debug, PartialEq, Eq)]
pub struct ClassNode {
    pub name: String,
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
