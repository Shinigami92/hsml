mod cli;

use std::process;

use cli::{
    cli, exec_check::exec_check, exec_compile::exec_compile, exec_format::exec_format,
    exec_parse::exec_parse,
};

fn main() -> Result<(), &'static str> {
    let matches = cli().get_matches();

    let result = match matches.subcommand() {
        Some(("compile", sub_matches)) => exec_compile(sub_matches),
        Some(("parse", sub_matches)) => exec_parse(sub_matches),
        Some(("fmt", sub_matches)) => exec_format(sub_matches),
        Some(("check", sub_matches)) => exec_check(sub_matches),
        Some((ext, _)) => {
            panic!("Unknown subcommand: {}", ext);
        }
        _ => unreachable!("Subcommand required"),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(1);
    } else {
        process::exit(0);
    }
}
