// build.rs

use std::env;
use std::process::Command;

fn main() {
    let git_hash = match env::var("GIT_HASH") {
        Ok(value) => value,
        _ => {
            let output = Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .unwrap();
            String::from_utf8(output.stdout).unwrap()
        }
    };
    println!("cargo:rustc-env=GIT_HASH={git_hash}");
}
