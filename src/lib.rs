

use std::path::PathBuf;

pub type Config = (&'static str, &'static [&'static str]);
pub type Configs = &'static [(&'static str, Config)];

pub const CONFIGS: Configs = &[
    ("clyde", ("./clyde", &[])),
    ("deno.json", ("deno", &["task"])),
    ("package.json", ("npm", &["run"])),
    ("Makefile", ("make", &[])),
    ("Cargo.toml", ("cargo", &["run"])),
    ("binding.gyp", ("node-gyp", &[])),
    ("gradlew", ("./gradlew", &["run"])),
    ("stack.yaml", ("stack", &[])),
];

pub fn check(p: &PathBuf) -> Option<&'static Config> {
    for (name, c) in CONFIGS {
        if p.join(name).exists() {
            return Some(c);
        }
    }
    None
}

pub fn run(c: &Config) {
    let output = std::process::Command::new(c.0)
        .args(c.1)
        .args(std::env::args().skip(1))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    if let Some(s) = output.status.code() {
        std::process::exit(s);
    }
    if !output.status.success() {
        std::process::exit(1);
    }
}
