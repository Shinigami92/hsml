use nom::IResult;

use super::process::{process_dev_comment, process_native_comment};

#[derive(Debug, PartialEq, Eq)]
pub struct CommentNode {
    pub text: String,
    pub is_dev: bool,
}

pub fn comment_dev_node(input: &str) -> IResult<&str, CommentNode> {
    let (input, comment) = process_dev_comment(input)?;

    Ok((
        input,
        CommentNode {
            text: comment.to_string(),
            is_dev: true,
        },
    ))
}

pub fn comment_native_node(input: &str) -> IResult<&str, CommentNode> {
    let (input, comment) = process_native_comment(input)?;

    Ok((
        input,
        CommentNode {
            text: comment.to_string(),
            is_dev: false,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::comment::node::{CommentNode, comment_dev_node, comment_native_node};

    #[test]
    fn it_should_return_comment_dev_node() {
        let (input, comment) = comment_dev_node("// This is a dev comment\n").unwrap();

        assert_eq!(
            comment,
            CommentNode {
                text: String::from(" This is a dev comment"),
                is_dev: true,
            }
        );

        assert_eq!(input, "\n");
    }

    #[test]
    fn it_should_return_comment_native_node() {
        let (input, comment) = comment_native_node("//! This is a native comment\n").unwrap();

        assert_eq!(
            comment,
            CommentNode {
                text: String::from(" This is a native comment"),
                is_dev: false,
            }
        );

        assert_eq!(input, "\n");
    }
}
