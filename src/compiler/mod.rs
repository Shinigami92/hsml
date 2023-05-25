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

    if let Some(id_node) = &tag_node.id {
        html_content.push_str(r#" id=""#);
        html_content.push_str(&id_node.id);
        html_content.push('\"');
    }

    if let Some(class_nodes) = &tag_node.classes {
        html_content.push_str(r#" class=""#);

        let class_names: String = class_nodes
            .iter()
            .map(|class_node| class_node.name.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        html_content.push_str(&class_names);

        html_content.push('\"');
    }

    if let Some(attributes) = &tag_node.attributes {
        attributes.iter().for_each(|node| match node {
            HsmlNode::Attribute(AttributeNode { key, value }) => {
                html_content.push(' ');
                html_content.push_str(key);

                if let Some(value) = value {
                    html_content.push_str(r#"=""#);
                    html_content.push_str(value);
                    html_content.push('\"');
                }
            }
            HsmlNode::Comment(node) if node.is_dev => {
                // do nothing
            }
            _ => panic!("Unsupported node type"),
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
        parser::{
            id::node::IdNode, parse::parse, tag::node::TagNode, text::node::TextNode, HsmlNode,
            RootNode,
        },
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
                id: None,
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
    fn it_should_compile_content_with_id() {
        let ast = RootNode {
            nodes: vec![HsmlNode::Tag(TagNode {
                tag: String::from("h1"),
                id: Some(IdNode {
                    id: String::from("title"),
                }),
                classes: None,
                attributes: None,
                text: Some(TextNode {
                    text: String::from("Hello World"),
                }),
                children: None,
            })],
        };

        let html_content = compile(&ast, &HsmlCompileOptions::default());

        assert_eq!(html_content, r#"<h1 id="title">Hello World</h1>"#);
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
  img(
    // supports attribute inline comments
    src="/fancy-avatar.jpg"
    alt="Fancy Avatar"
    // the size of the image
    width="384"
    height="512"
  )
"#;

        let (rest, ast) = parse(input).unwrap();

        let html_content = compile(&ast, &HsmlCompileOptions::default());

        assert_eq!(
            html_content,
            r#"<h1 class="text-red">Vite CJS Faker Demo</h1><div class="card"><div class="card__image"><img :src="natureImageUrl" :alt="'Background image for ' + fullName"/></div><div class="card__profile"><img :src="avatarUrl" :alt="'Avatar image of ' + fullName"/></div><div class="card__body">{{ fullName }}</div><img src="/fancy-avatar.jpg" alt="Fancy Avatar" width="384" height="512"/></div>"#
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn it_should_compile_parsed_elk_status_content_component() {
        let input = r#".space-y-3(
  :class="{
    'pt2 pb0.5 px3.5 bg-dm rounded-4 me--1': isDM,
    'ms--3.5 mt--1 ms--1': isDM && context !== 'details',
  }"
)
  StatusBody(v-if="(!isFiltered && isSensitiveNonSpoiler) || hideAllMedia" :status="status" :newer="newer" :with-action="!isDetails" :class="isDetails ? 'text-xl' : ''")
  StatusSpoiler(:enabled="hasSpoilerOrSensitiveMedia || isFiltered" :filter="isFiltered" :sensitive-non-spoiler="isSensitiveNonSpoiler || hideAllMedia" :is-d-m="isDM")
    template(v-if="spoilerTextPresent" #spoiler)
      p {{ status.spoilerText }}
    template(v-else-if="filterPhrase" #spoiler)
      p {{ `${$t('status.filter_hidden_phrase')}: ${filterPhrase}` }}
    StatusBody(v-if="!(isSensitiveNonSpoiler || hideAllMedia)" :status="status" :newer="newer" :with-action="!isDetails" :class="isDetails ? 'text-xl' : ''")
    StatusTranslation(:status="status")
    StatusPoll(v-if="status.poll" :status="status")
    StatusMedia(
      v-if="status.mediaAttachments?.length"
      :status="status"
      :is-preview="isPreview"
    )
    StatusPreviewCard(
      v-if="status.card"
      :card="status.card"
      :small-picture-only="status.mediaAttachments?.length > 0"
    )
    StatusCard(
      v-if="status.reblog"
      :status="status.reblog"
      border="~ rounded"
      :actions="false"
    )
    div(v-if="isDM")
"#;

        let (rest, ast) = parse(input).unwrap();

        let html_content = compile(&ast, &HsmlCompileOptions::default());

        assert_eq!(
            html_content,
            r#"<div class="space-y-3" :class="{
    'pt2 pb0.5 px3.5 bg-dm rounded-4 me--1': isDM,
    'ms--3.5 mt--1 ms--1': isDM && context !== 'details',
  }"><StatusBody v-if="(!isFiltered && isSensitiveNonSpoiler) || hideAllMedia" :status="status" :newer="newer" :with-action="!isDetails" :class="isDetails ? 'text-xl' : ''"/><StatusSpoiler :enabled="hasSpoilerOrSensitiveMedia || isFiltered" :filter="isFiltered" :sensitive-non-spoiler="isSensitiveNonSpoiler || hideAllMedia" :is-d-m="isDM"><template v-if="spoilerTextPresent" #spoiler><p>{{ status.spoilerText }}</p></template><template v-else-if="filterPhrase" #spoiler><p>{{ `${$t('status.filter_hidden_phrase')}: ${filterPhrase}` }}</p></template><StatusBody v-if="!(isSensitiveNonSpoiler || hideAllMedia)" :status="status" :newer="newer" :with-action="!isDetails" :class="isDetails ? 'text-xl' : ''"/><StatusTranslation :status="status"/><StatusPoll v-if="status.poll" :status="status"/><StatusMedia v-if="status.mediaAttachments?.length" :status="status" :is-preview="isPreview"/><StatusPreviewCard v-if="status.card" :card="status.card" :small-picture-only="status.mediaAttachments?.length > 0"/><StatusCard v-if="status.reblog" :status="status.reblog" border="~ rounded" :actions="false"/><div v-if="isDM"/></StatusSpoiler></div>"#
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn it_should_compile_parsed_elk_main_content_component() {
        let input = r#"div(ref="container" :class="containerClass")
  .sticky.top-0.z10.backdrop-blur.native:lg:w-[calc(100vw-5rem)].native:xl:w-[calc(135%+(100vw-1200px)/2)](
    pt="[env(safe-area-inset-top,0)]"
    bg="[rgba(var(--rgb-bg-base),0.7)]"
  )
    .flex.justify-between.px5.py2.native:xl:flex(:class="{ 'xl:hidden': $route.name !== 'tag' }" border="b base")
      .flex.gap-3.items-center.py2.w-full(:overflow-hidden="!noOverflowHidden ? '' : false")
        NuxtLink.items-center.btn-text.p-0.xl:hidden(
          v-if="backOnSmallScreen || back"
          flex="~ gap1"
          :aria-label="$t('nav.back')"
          @click="$router.go(-1)"
        )
          .rtl-flip(i-ri:arrow-left-line)
        .flex.w-full.native-mac:justify-center.native-mac:text-center.native-mac:sm:justify-start(
          :truncate="!noOverflowHidden ? '' : false"
          data-tauri-drag-region
        )
          slot(name="title")
        .sm:hidde.nh-7.w-1px
      .flex.items-center.flex-shrink-0.gap-x-2
        slot(name="actions")
        PwaBadge.lg:hidden
        NavUser(v-if="isHydrated")
        NavUserSkeleton(v-else)
    slot(name="header")
      div(hidden)
  PwaInstallPrompt.lg:hidden
  .m-auto(:class="isHydrated && wideLayout ? 'xl:w-full sm:max-w-600px' : 'sm:max-w-600px md:shrink-0'")
    .h-6(hidden :class="{ 'xl:block': $route.name !== 'tag' && !$slots.header }")
    slot
"#;

        let (rest, ast) = parse(input).unwrap();

        let html_content = compile(&ast, &HsmlCompileOptions::default());

        assert_eq!(
            html_content,
            r#"<div ref="container" :class="containerClass"><div class="sticky top-0 z10 backdrop-blur native:lg:w-[calc(100vw-5rem)] native:xl:w-[calc(135%+(100vw-1200px)/2)]" pt="[env(safe-area-inset-top,0)]" bg="[rgba(var(--rgb-bg-base),0.7)]"><div class="flex justify-between px5 py2 native:xl:flex" :class="{ 'xl:hidden': $route.name !== 'tag' }" border="b base"><div class="flex gap-3 items-center py2 w-full" :overflow-hidden="!noOverflowHidden ? '' : false"><NuxtLink class="items-center btn-text p-0 xl:hidden" v-if="backOnSmallScreen || back" flex="~ gap1" :aria-label="$t('nav.back')" @click="$router.go(-1)"><div class="rtl-flip" i-ri:arrow-left-line/></NuxtLink><div class="flex w-full native-mac:justify-center native-mac:text-center native-mac:sm:justify-start" :truncate="!noOverflowHidden ? '' : false" data-tauri-drag-region><slot name="title"/></div><div class="sm:hidde nh-7 w-1px"/></div><div class="flex items-center flex-shrink-0 gap-x-2"><slot name="actions"/><PwaBadge class="lg:hidden"/><NavUser v-if="isHydrated"/><NavUserSkeleton v-else/></div></div><slot name="header"><div hidden/></slot></div><PwaInstallPrompt class="lg:hidden"/><div class="m-auto" :class="isHydrated && wideLayout ? 'xl:w-full sm:max-w-600px' : 'sm:max-w-600px md:shrink-0'"><div class="h-6" hidden :class="{ 'xl:block': $route.name !== 'tag' && !$slots.header }"/><slot/></div></div>"#
        );
        assert_eq!(rest, "");
    }
}
