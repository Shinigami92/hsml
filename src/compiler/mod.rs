use crate::parser::{
    attribute::node::AttributeNode, comment::node::CommentNode, tag::node::TagNode, HsmlNode,
    RootNode,
};

#[derive(Default)]
pub struct HsmlCompileOptions {}

fn compile_tag_node(tag_node: &TagNode, _options: &HsmlCompileOptions) -> String {
    let mut html_content = String::new();

    html_content.push('<');
    html_content.push_str(&tag_node.tag);

    if let Some(class_nodes) = &tag_node.classes {
        html_content.push_str(" class=\"");

        let class_names: String = class_nodes
            .iter()
            .map(|class_node| class_node.name.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        html_content.push_str(&class_names);

        html_content.push('\"');
    }

    if let Some(attributes) = &tag_node.attributes {
        attributes.iter().for_each(|AttributeNode { key, value }| {
            html_content.push(' ');
            html_content.push_str(key);

            if let Some(value) = value {
                html_content.push_str("=\"");
                html_content.push_str(value);
                html_content.push('\"');
            }
        });
    }

    let should_auto_close = tag_node.children.is_none() && tag_node.text.is_none();
    if should_auto_close {
        html_content.push_str("/>");
        return html_content;
    } else {
        html_content.push('>');
    }

    if let Some(text) = &tag_node.text {
        html_content.push_str(&text.text);
    }

    if let Some(child_nodes) = &tag_node.children {
        for child_node in child_nodes {
            match child_node {
                HsmlNode::Tag(tag_node) => {
                    html_content.push_str(&compile_tag_node(tag_node, _options))
                }
                HsmlNode::Comment(comment_node) => {
                    if !comment_node.is_dev {
                        html_content.push_str(&compile_comment_node(comment_node, _options))
                    }
                }
                _ => panic!("Unsupported node type"),
            }
        }
    }

    html_content.push_str("</");
    html_content.push_str(&tag_node.tag);
    html_content.push('>');

    html_content
}

fn compile_comment_node(comment_node: &CommentNode, _options: &HsmlCompileOptions) -> String {
    let mut html_content = String::new();

    html_content.push_str("<!--");
    html_content.push_str(&comment_node.text);
    html_content.push_str("-->");

    html_content
}

fn compile_node(node: &HsmlNode, options: &HsmlCompileOptions) -> String {
    match node {
        HsmlNode::Tag(tag_node) => compile_tag_node(tag_node, options),
        HsmlNode::Comment(comment_node) if !comment_node.is_dev => {
            compile_comment_node(comment_node, options)
        }
        HsmlNode::Comment(_) => String::from(""),
        _ => panic!("Unsupported node type"),
    }
}

pub fn compile(hsml_ast: &RootNode, options: &HsmlCompileOptions) -> String {
    let mut html_content = String::new();

    for node in &hsml_ast.nodes {
        html_content.push_str(&compile_node(node, options));
    }

    html_content
}

#[cfg(test)]
mod tests {
    use crate::{
        compiler::{compile, HsmlCompileOptions},
        parser::{parse::parse, tag::node::TagNode, text::node::TextNode, HsmlNode, RootNode},
    };

    #[test]
    fn it_should_compile_empty_ast() {
        let ast = RootNode { nodes: vec![] };

        let html_content = compile(&ast, &HsmlCompileOptions::default());

        assert_eq!(html_content, "");
    }

    #[test]
    fn it_should_compile_simple_tag() {
        let ast = RootNode {
            nodes: vec![HsmlNode::Tag(TagNode {
                tag: String::from("h1"),
                classes: None,
                attributes: None,
                text: Some(TextNode {
                    text: String::from("Hello World"),
                }),
                children: None,
            })],
        };

        let html_content = compile(&ast, &HsmlCompileOptions::default());

        assert_eq!(html_content, "<h1>Hello World</h1>");
    }

    #[test]
    fn it_should_compile_parsed_content() {
        let input = r#"h1.text-red Vite CJS Faker Demo
.card
  .card__image
    img(:src="natureImageUrl" :alt="'Background image for ' + fullName")
  .card__profile
    img(:src="avatarUrl" :alt="'Avatar image of ' + fullName")
  .card__body {{ fullName }}
"#;

        let (rest, ast) = parse(input).unwrap();

        let html_content = compile(&ast, &HsmlCompileOptions::default());

        assert_eq!(
            html_content,
            r#"<h1 class="text-red">Vite CJS Faker Demo</h1><div class="card"><div class="card__image"><img :src="natureImageUrl" :alt="'Background image for ' + fullName"/></div><div class="card__profile"><img :src="avatarUrl" :alt="'Avatar image of ' + fullName"/></div><div class="card__body">{{ fullName }}</div></div>"#
        );
        assert_eq!(rest, "");
    }
}
