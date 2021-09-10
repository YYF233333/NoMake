use nomake::*;
use std::fs;

#[test]
fn binary() {
    let sources = collect_source();
    compile(sources, "binary.out", OutType::Binary);
    fs::remove_file("binary.out").unwrap();
}

#[test]
fn shared_lib() {
    let sources = collect_source();
    compile(sources, "shared.so", OutType::Shared);
    fs::remove_file("shared.so").unwrap();
}

#[test]
fn static_lib() {
    let sources = collect_source();
    compile(sources, "static.a", OutType::Static);
    fs::remove_file("static.a").unwrap();
}
