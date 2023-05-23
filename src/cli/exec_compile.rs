use std::{env, fs, path::PathBuf};

use clap::ArgMatches;
use hsml::{
    compiler::{compile, HsmlCompileOptions},
    parser::parse::parse,
};

pub fn exec_compile(matches: &ArgMatches) -> Result<(), &str> {
    println!("Compiling...");
    let path = matches.get_one::<PathBuf>("path");
    let out = matches.get_one::<PathBuf>("output");

    let fallback_path = env::current_dir().expect("Unable to get current directory");
    let path = path.unwrap_or(&fallback_path);

    if path.is_dir() {
        todo!("Directory compilation")
    } else if path.is_file() {
        let file = path;
        // check that file ends with .hsml
        if file.extension().unwrap() != "hsml" {
            return Err("File must have .hsml extension");
        }

        // check that file exists and read it
        let content = fs::read_to_string(file).expect("Unable to read file");

        // parse the file
        let (_, hsml_ast) = parse(&content).expect("Unable to parse file");

        // compile the AST
        let html_content = compile(&hsml_ast, &HsmlCompileOptions::default());

        if out.is_some() {
            todo!("Write to out file");
        }

        // write the compiled HTML to a file with the same name as the input file but with .html extension
        let file = file.with_extension("html");
        fs::write(&file, html_content).expect("Unable to write file");

        println!("Compiled HTML written to {} successfully", file.display());
    } else {
        return Err("Path must be a file or directory");
    }

    Ok(())
}
