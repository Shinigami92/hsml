use nom::{
    bytes::complete::{tag, take_until1},
    IResult,
};

pub fn process_text(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ")(input)?;
    take_until1("\n")(input)
}
