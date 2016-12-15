#![feature(rustc_private, box_syntax)]

extern crate rustc;
extern crate rustc_driver;
extern crate syntax;
extern crate glob;

use std::path::{PathBuf, Path};
use std::io;
use syntax::codemap::FileLoader;
use glob::glob;

struct CompilerInput(String);

impl FileLoader for CompilerInput {
    fn file_exists(&self, _: &Path) -> bool { true }
    fn abs_path(&self, _: &Path) -> Option<PathBuf> { None }
    fn read_file(&self, _: &Path) -> io::Result<String> { Ok(self.0.clone()) }
}

#[allow(dead_code)]
enum LinkType {
    Sysroot,
    Individually
}

static LINK_TYPE: LinkType = LinkType::Individually;
static SYSROOT: &'static str =
    "/Users/will/.multirust/toolchains/nightly-x86_64-apple-darwin";

fn run_compiler(src: String) -> isize {
    let crate_name = "foobar";

    let linker_args = match LINK_TYPE {
        LinkType::Sysroot => {
            format!("--sysroot {}", SYSROOT)
        },
        LinkType::Individually => {
            let rustlib_path =
                format!("{}/lib/rustlib/x86_64-apple-darwin/lib", SYSROOT);
            let libs = vec!["std", "core"];
            let libflags = libs.into_iter().map(|lib| {
                let matches =
                    glob(&format!("{}/lib{}-*", rustlib_path, lib)).expect("Invalid glob");
                let path = matches.into_iter().next()
                    .expect(&format!("Missing lib {}", lib))
                    .expect("Invalid path");
                format!("--extern {}={}", lib, path.display())})
                .collect::<Vec<String>>()
                .join(" ");
            format!("{} -L {}", libflags, rustlib_path)
        }
    };

    let args: Vec<String> =
        format!(
            "_ {} {} --crate-type dylib --cap-lints allow",
            linker_args,
            crate_name)
        .split(' ').map(|s| s.to_string()).collect();

    rustc_driver::run(
        move ||
            rustc_driver::run_compiler(
                &args,
                &mut rustc_driver::RustcDefaultCalls,
                Some(box CompilerInput(src)),
                None))
}

fn main() {
    let src = r#"
#[no_mangle]
pub fn foobar(){
    println!("foobar");
}"#;

    match run_compiler(src.to_string()) {
        0 => (),
        n => panic!("Compilation failed with error code {}", n)
    };
}
