mod lib;
use lib::{args::*, compile::*};

fn main() {
    let args = Args::from_args();
    println!("{:?}", args);

    // traverse the whole project and gather .c files
    let sources = collect_source();

    compile(sources, args);
}
