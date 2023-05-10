use hsml::parser::parse;

fn main() {
    let content = "h1 Vite CJS Faker Demo\n
  .card\n
    .card__image\n
      img(:src=\"natureImageUrl\" :alt=\"'Background image for ' + fullName\")\n
    .card__profile\n
      img(:src=\"avatarUrl\" :alt=\"'Avatar image of ' + fullName\")\n
    .card__body {{ fullName }}\n
";

    let (result, tokens) = parse(content).unwrap();

    dbg!(tokens);
}
