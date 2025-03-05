pub mod compiler;
pub mod parser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_content(source: &str) -> String {
    let (_, ast) = parser::parse::parse(source).unwrap();

    compiler::compile(&ast, &compiler::HsmlCompileOptions::default())
}
