use nom::{
    bytes::complete::{tag, take_until1},
    IResult,
};

pub fn process_text(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ")(input)?;
    take_until1("\n")(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::text::process::process_text;

    #[test]
    fn test_process_text() {
        let input = " hello world\n";

        let (rest, text) = process_text(input).unwrap();

        assert_eq!(text, "hello world");
        assert_eq!(rest, "\n");
    }
}
