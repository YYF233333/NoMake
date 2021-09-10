use std::{env, process::exit};

#[macro_use]
extern crate clap;

use clap::{App, Arg};

use nomake::*;

fn main() {
    let argparse = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .help("specify output file"),
        )
        .arg(
            Arg::with_name("lib")
                .short("lib")
                .takes_value(true)
                .help("compile with library rules, value could be \"static\" or \"shared\""),
        )
        .get_matches();

    let outfile = match argparse.value_of("output") {
        Some(s) => s,
        // default outfile name
        None => "a.out",
    };

    let lib = match argparse.value_of("lib") {
        None => OutType::Binary,
        Some("static") => OutType::Static,
        Some("shared") => OutType::Shared,
        _ => {
            eprintln!("Unable to parse value of argument \"lib\"");
            exit(1);
        }
    };
    // traverse the whole project and gather .c files
    let sources = collect_source();

    compile(sources, outfile, lib);
}
