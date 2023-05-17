use std::env;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Target architecture: {}", target_arch);

    if target_arch == "wasm32" {
        println!("cargo:rustc-link-search=./say_hello/build");
        println!("cargo:rustc-link-lib=static=say_hello");    
    }
}
