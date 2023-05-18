use hsml::parser::parse;

fn main() {
    let content = "h1.text-red Vite CJS Faker Demo
  .card
    .card__image
      img(:src=\"natureImageUrl\" :alt=\"'Background image for ' + fullName\")
    .card__profile
      img(:src=\"avatarUrl\" :alt=\"'Avatar image of ' + fullName\")
    .card__body {{ fullName }}
";

    let (_, hsml_ast) = parse(content).unwrap();

    dbg!(hsml_ast);
}
