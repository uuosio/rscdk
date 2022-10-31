// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements. See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership. The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License. You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. See the License for the
// specific language governing permissions and limitations
// under the License.

use lazy_static::lazy_static; // 1.4.0
use std::sync::{
    Mutex,
    MutexGuard
};

pub mod interfaces;
pub mod client;
pub use client::{
    new_vm_api_client,
    ChainTester,
    get_vm_api_client,
    close_vm_api_client,
    get_globals,
    get_test_mutex,
    n2s,
    GetTableRowsPrams,
};

pub mod server;


pub struct DebuggerConfig {
    pub debugger_server_address: String,
    pub debugger_server_port: u16,
    pub vm_api_server_address: String,
    pub vm_api_server_port: u16,
    pub apply_request_server_address: String,
    pub apply_request_server_port: u16,
}

impl DebuggerConfig {
    fn new() -> Self {
        Self { 
            debugger_server_address: "127.0.0.1".into(), 
            debugger_server_port: 9090, 
            vm_api_server_address: "127.0.0.1".into(), 
            vm_api_server_port: 9092,
            apply_request_server_address: "127.0.0.1".into(), 
            apply_request_server_port: 9091,
        }
    }
}

lazy_static! {
    static ref DEBUGGER_CONFIG: Mutex<DebuggerConfig> = Mutex::new(DebuggerConfig::new());
}

pub fn get_debugger_config() -> MutexGuard<'static, DebuggerConfig> {
    return DEBUGGER_CONFIG.lock().unwrap()
}

extern "Rust" {
	pub fn __eosio_generate_abi() -> String;
}

// pub fn generate_abi_file(package_name: &str) {
// 	let abi = unsafe {
// 		__eosio_generate_abi()
// 	};

//     // let package_name = env!("CARGO_PKG_NAME");
//     let abi_file = format!("./target/{}.abi", package_name);
// 	match std::fs::write(std::path::Path::new(&abi_file), abi) {
//         Ok(()) => {

//         }
//         Err(err) => {
//             panic!("{}", err);
//         }
//     }
// }

lazy_static! {
    static ref BUILD_CONTRACT_MUTEX: Mutex<std::collections::HashMap<String, String>> = Mutex::new(std::collections::HashMap::new());
}

pub fn build_contract(package_name: &str, project_dir: &str) {
    println!("++++++building {package_name} at {project_dir}");
    let mut build_contract = BUILD_CONTRACT_MUTEX.lock().unwrap();
    if build_contract.get(package_name).is_some() {
        return;
    }

    build_contract.insert(package_name.into(), project_dir.into());

    // let mut cargo_path: String = which::which("cargo").unwrap().to_str().unwrap().into();
    // if cargo_path.contains(".rustup") {
    //     let pos = cargo_path.find(".rustup").unwrap();
    //     cargo_path = format!("{}/{}", &cargo_path[0..pos], ".cargo/bin/cargo");
    //     println!("++++++++++cargo path:{}", cargo_path);
    // }
    std::env::set_var("RUSTFLAGS", "-C link-arg=-zstack-size=8192  -Clinker-plugin-lto");
    let mut cmd = std::process::Command::new("cargo");
    cmd
        .args([
            "+nightly",
            "build",
            "--target=wasm32-wasi",
            &format!("--target-dir={project_dir}/target"),
            "-Zbuild-std",
            "--no-default-features",
            "--release",
            "-Zbuild-std-features=panic_immediate_abort"
            ]
        );

    let mut child = cmd
        // capture the stdout to return from this function as bytes
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("command failed to start");
    let output = child.wait().unwrap();
    if !output.success() {
        panic!("build failed");
    }

    let in_wasm_file = format!("{project_dir}/target/wasm32-wasi/release/{}.wasm", package_name);
    let out_wasm_file = format!("{project_dir}/target/{}.wasm", package_name);
    let wasm = std::fs::read(in_wasm_file).unwrap();
    std::fs::write(out_wasm_file, wasm).unwrap();

    // generate_abi_file(package_name);
}

