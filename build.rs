use std::env;

fn main() {
    let cli_name = env::var("CARGO_PKG_NAME").unwrap();
    let cli_version = env::var("CARGO_PKG_VERSION").unwrap();

    println!("cargo:rustc-env=CLI_NAME={}", cli_name);

    println!("cargo:rustc-env=CLI_VERSION={}", cli_version);   
}