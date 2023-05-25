use std::env;
use std::process::Command;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Target architecture: {}", target_arch);

    if target_arch == "wasm32" {
        let output = Command::new("cdt-get-root-dir")
        .output()
        .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("{}", stdout);

        println!("cargo:rustc-link-search=./say_hello/build");
        println!("cargo:rustc-link-lib=static=say_hello");

        println!("cargo:rustc-link-search={}/{}", stdout.trim(), "lib");
        println!("cargo:rustc-link-lib=static=c++");
    }
}
