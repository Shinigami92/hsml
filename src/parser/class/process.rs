use nom::{
    bytes::complete::{tag, take_till1},
    IResult,
};

pub fn process_class(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(".")(input)?;
    take_till1(|c| c == ' ' || c == '.' || c == '(' || c == '\n')(input)
}
