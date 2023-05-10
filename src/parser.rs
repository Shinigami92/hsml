use nom::branch::alt;
use nom::{
    bytes::complete::{tag, take_till1, take_until1},
    character::complete::line_ending,
    IResult,
};

pub fn process_tag(input: &str) -> IResult<&str, &str> {
    take_until1(" ")(input)
}

pub fn process_class(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(".")(input)?;
    take_till1(|c| c == ' ' || c == '.' || c == '\n')(input)
}

pub fn process_text(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ")(input)?;
    take_until1("\n")(input)
}

pub fn process_newline(input: &str) -> IResult<&str, &str> {
    line_ending(input)
}

#[derive(Debug, PartialEq)]
pub enum HsmlToken {
    Tag(String),
    Class(String),
    Text(String),
    Newline,
}

pub fn parse(input: &str) -> IResult<&str, Vec<HsmlToken>> {
    let mut tokens: Vec<HsmlToken> = vec![];

    let mut input = input;

    loop {
        let (rest, tag_token) = process_tag(input)?;
        tokens.push(HsmlToken::Tag(tag_token.to_string()));
        input = rest;

        let mut classes: Vec<String> = vec![];

        loop {
            let (rest, class) = process_class(input)?;
            classes.push(class.to_string());
            input = rest;

            let (rest, _) = alt((tag("."), tag("\n")))(input)?;
            input = rest;

            if input.starts_with(" ") {
                let (rest, _) = tag(" ")(input)?;
                input = rest;
            } else {
                break;
            }
        }

        tokens.push(HsmlToken::Class(classes.join(" ")));

        let (rest, text) = process_text(input)?;
        tokens.push(HsmlToken::Text(text.to_string()));
        input = rest;

        let (rest, newline) = process_newline(input)?;
        tokens.push(HsmlToken::Newline);
        input = rest;

        if input.is_empty() {
            break;
        }
    }

    Ok((input, tokens))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_process_tag() {
        let content = "h1 Vite CJS Faker Demo\n";

        let (rest, tag) = super::process_tag(content).unwrap();

        assert_eq!(tag, "h1");
        assert_eq!(rest, " Vite CJS Faker Demo\n");
    }

    #[test]
    fn it_should_process_class() {
        let content = ".text-red Vite CJS Faker Demo\n";

        let (rest, class) = super::process_class(content).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, " Vite CJS Faker Demo\n");
    }

    #[test]
    fn it_should_process_class_followed_by_class() {
        let content = ".text-red.font-bold Vite CJS Faker Demo\n";

        let (rest, class) = super::process_class(content).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, ".font-bold Vite CJS Faker Demo\n");

        let (rest, class) = super::process_class(rest).unwrap();

        assert_eq!(class, "font-bold");
        assert_eq!(rest, " Vite CJS Faker Demo\n");
    }

    #[test]
    fn it_should_process_text_after_tag() {
        let content = "h1 Vite CJS Faker Demo\n";

        let (rest, _) = super::process_tag(content).unwrap();
        let (rest, text) = super::process_text(rest).unwrap();

        assert_eq!(text, "Vite CJS Faker Demo");
        assert_eq!(rest, "\n");
    }

    #[test]
    fn it_should_process_newline() {
        let content = "h1 Vite CJS Faker Demo\n";

        let (rest, _) = super::process_tag(content).unwrap();
        let (rest, _) = super::process_text(rest).unwrap();
        let (rest, newline) = super::process_newline(rest).unwrap();

        assert_eq!(newline, "\n");
        assert_eq!(rest, "");
    }
}
