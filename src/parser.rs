use nom::{
    bytes::complete::{tag, take_till1, take_until1},
    character::complete::line_ending,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum HsmlToken {
    Tag(String),
    Class(String),
    Text(String),
    Newline,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TextNode {
    pub text: String,
}

#[derive(Debug, PartialEq)]
pub struct TagNode {
    pub tag: String,
    pub classes: Option<Vec<ClassNode>>,
    pub text: Option<TextNode>,
    pub children: Option<Vec<HsmlNode>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClassNode {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum HsmlNode {
    Tag(TagNode),
    Class(ClassNode),
    Text(String),
    Newline,
}

pub fn process_tag(input: &str) -> IResult<&str, &str> {
    take_till1(|c| c == ' ' || c == '.' || c == '\n')(input)
}

pub fn tag_node(input: &str) -> IResult<&str, TagNode> {
    let (input, tag_name) = process_tag(input)?;

    let mut classes: Vec<ClassNode> = vec![];

    let mut input = input;

    while let Ok((rest, class)) = class_node(input) {
        classes.push(class);
        input = rest;
    }

    let mut text_node: Option<TextNode> = None;

    if let Ok((rest, text)) = process_text(input) {
        text_node = Some(TextNode {
            text: text.to_string(),
        });
        input = rest;
    }

    Ok((
        input,
        TagNode {
            tag: tag_name.to_string(),
            classes: (!classes.is_empty()).then_some(classes),
            text: text_node,
            children: None,
        },
    ))
}

pub fn process_class(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(".")(input)?;
    take_till1(|c| c == ' ' || c == '.' || c == '\n')(input)
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

pub fn process_text(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ")(input)?;
    take_until1("\n")(input)
}

pub fn process_newline(input: &str) -> IResult<&str, &str> {
    line_ending(input)
}

pub fn parse(input: &str) -> IResult<&str, HsmlNode> {
    let (input, tag_node) = tag_node(input)?;

    Ok((input, HsmlNode::Tag(tag_node)))
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
