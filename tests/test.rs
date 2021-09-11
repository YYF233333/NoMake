use lib_nomake::{args::*, compile::*};
use std::fs;

#[test]
fn binary() {
    let args = Args {
        lib: OutType::Binary,
        release: false,
        output: String::from("binary.out"),
    };
    let sources = collect_source();
    compile(sources, args);
    fs::remove_file("binary.out").unwrap();
}

#[test]
fn shared_lib() {
    let args = Args {
        lib: OutType::Shared,
        release: false,
        output: String::from("shared.so"),
    };
    let sources = collect_source();
    compile(sources, args);
    fs::remove_file("shared.so").unwrap();
}

#[test]
fn static_lib() {
    let args = Args {
        lib: OutType::Static,
        release: false,
        output: String::from("static.a"),
    };
    let sources = collect_source();
    compile(sources, args);
    fs::remove_file("static.a").unwrap();
}
