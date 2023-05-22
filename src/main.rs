use std::{fs, path::PathBuf};

use clap::{arg, command, value_parser};
use hsml::{
    compiler::{compile, HsmlCompileOptions},
    parser::parse::parse,
};

fn main() {
    let matches = command!()
        .about("HSML command line tool")
        .arg(arg!(<FILE>).value_parser(value_parser!(PathBuf)))
        .get_matches();

    let file = matches
        .get_one::<PathBuf>("FILE")
        .expect("No file provided");

    // check that file ends with .hsml
    if file.extension().unwrap() != "hsml" {
        panic!("File must have .hsml extension");
    }

    // check that file exists and read it
    let content = fs::read_to_string(file).expect("Unable to read file");

    // parse the file
    let (_, hsml_ast) = parse(&content).expect("Unable to parse file");

    // compile the AST
    let html_content = compile(&hsml_ast, &HsmlCompileOptions::default());

    // write the compiled HTML to a file with the same name as the input file but with .html extension
    let file = file.with_extension("html");
    fs::write(&file, html_content).expect("Unable to write file");

    println!("Compiled HTML written to {} successfully", file.display());
}
