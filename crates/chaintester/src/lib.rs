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

