use std::{
    env, fs,
    path::Path,
    process::{exit, Child, Command},
};
use walkdir::{DirEntry, WalkDir};

use super::args::*;

pub fn collect_source() -> Vec<DirEntry> {
    collect_file(env::current_dir().unwrap(), |name| name.ends_with(".c"))
}

pub fn compile(sources: Vec<DirEntry>, args: Args) {
    fs::create_dir_all("bin").unwrap();

    let objects: Vec<String> = sources.into_iter().map(to_object).collect();

    let mut command = match args.lib {
        OutType::Binary => link_binary(objects, &args.output),
        OutType::Shared => link_shared(objects, &args.output),
        OutType::Static => link_static(objects, &args.output),
    };

    if !command.wait().unwrap().success() {
        eprintln!("Failed to link object files");
        eprintln!("Exiting...");
        exit(1);
    }
}

// traverse directory recursively, collecting all files meet the custom requirements
fn collect_file<P: AsRef<Path>>(root: P, filter: fn(file_name: &str) -> bool) -> Vec<DirEntry> {
    let mut sources = Vec::new();

    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        if let Some(name) = entry.file_name().to_str() {
            if filter(name) {
                sources.push(entry);
            }
        };
    }
    sources
}

// compile a .c source file, place the .o object file in folder "bin"
fn to_object(source: DirEntry) -> String {
    let name = source.file_name().to_str().unwrap();
    let name = name.strip_suffix(".c").unwrap();
    println!("compiling {}", name);
    let mut command = Command::new("gcc")
        .arg("-c")
        .arg("-o")
        .arg(format!("./bin/{}.o", name))
        .arg(source.path())
        .spawn()
        .expect("Command failed to start");

    if !command.wait().unwrap().success() {
        eprintln!("Failed to compile {}", name);
        eprintln!("Exiting...");
        exit(1);
    }
    format!("./bin/{}.o", name)
}

// link rules for binary
fn link_binary(objects: Vec<String>, filename: &str) -> Child {
    Command::new("gcc")
        .args(["-o", filename])
        .args(objects)
        .spawn()
        .expect("Command failed to start")
}

// link rules for shared lib
fn link_shared(objects: Vec<String>, filename: &str) -> Child {
    Command::new("gcc")
        .arg("-shared")
        .args(["-o", filename])
        .args(objects)
        .spawn()
        .expect("Command failed to start")
}

// link rules for static lib
fn link_static(objects: Vec<String>, filename: &str) -> Child {
    Command::new("ar")
        .arg("rcs")
        .arg(filename)
        .args(objects)
        .spawn()
        .expect("Command failed to start")
}
