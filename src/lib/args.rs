use std::process::exit;

pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Args {
    /// Compile with library rules, value could be "static" or "shared"
    #[structopt(long, parse(from_str = parse_arg_lib), default_value = "binary")]
    pub lib: OutType,
    /// Activate release mode
    #[structopt(short, long)]
    pub release: bool,
    /// Specify output file
    #[structopt(short, long, parse(from_str), default_value = "a.out")]
    pub output: String,
}

#[derive(Debug, PartialEq)]
pub enum OutType {
    Binary,
    Static,
    Shared,
}

fn parse_arg_lib(lib: &str) -> OutType {
    match lib {
        "binary" => OutType::Binary,
        "static" => OutType::Static,
        "shared" => OutType::Shared,
        _ => {
            eprintln!("Unable to parse value of argument \"lib\"");
            exit(1);
        }
    }
}
