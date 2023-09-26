use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
    let mut bundler: Bundler = Bundler::new(Path::new("src/main.rs"), Path::new("target/out.rs"));
    bundler.crate_name("cf-rust");
    bundler.run();
}
