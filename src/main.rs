#![feature(rustc_private, box_syntax)]

extern crate rustc_driver;
extern crate syntax;

use syntax::codemap::FileLoader;
use std::path::{PathBuf, Path};
use std::io;

struct CompilerInput(String);

impl FileLoader for CompilerInput {
    fn file_exists(&self, _: &Path) -> bool { true }
    fn abs_path(&self, _: &Path) -> Option<PathBuf> { None }
    fn read_file(&self, _: &Path) -> io::Result<String> { Ok(self.0.clone()) }
}

fn run_compiler(src: String) {
    let crate_name = "foobar";
    let args: Vec<String> =
        format!(
            "_ {} --sysroot {} --crate-type dylib --cap-lints allow",
            crate_name,
            "/Users/will/.multirust/toolchains/nightly-x86_64-apple-darwin")
        .split(' ').map(|s| s.to_string()).collect();
    let result = rustc_driver::run(
        move ||
            rustc_driver::run_compiler(
                &args,
                &mut rustc_driver::RustcDefaultCalls,
                Some(box CompilerInput(src)),
                None));
}

fn main() {
    let src = r#"
#[no_mangle]
pub fn foobar(){
    println!("foobar");
}"#;
    run_compiler(src.to_string());
}
