use std::fmt;
use std::panic;

use std::{fs};
use std::{thread, time::Duration};
use std::ops::{Deref, DerefMut};

use serde_json::{Value};

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
};

use crate::interfaces::{IPCChainTesterSyncClient, TIPCChainTesterSyncClient, ApplySyncClient, Action};

type ClientInputProtocol = TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>;
type ClientOutputProtocol = TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>;


use std::convert::{From, Into};

use lazy_static::lazy_static; // 1.4.0
use std::sync::{
    Mutex,
    MutexGuard
};


pub struct ChainTesterError {
    pub json: Option<Value>,
    pub error_string: Option<String>,
}

pub enum JsonKeyType {
    ArrayIndex(usize),
    MapKey(String)
}

impl From<usize> for JsonKeyType {
    fn from(value: usize) -> Self {
        JsonKeyType::ArrayIndex(value)
    }
}

impl From<String> for JsonKeyType {
    fn from(value: String) -> Self {
        JsonKeyType::MapKey(value)
    }
}

impl ChainTesterError {
    pub fn get_err(&self) -> Option<String> {
        let value = &self.json.as_ref().unwrap()["except"]["stack"][0]["data"]["s"];
        if let Value::String(s) = value {
            return Some(s.clone())
        }
        return None;
    }

    pub fn check_err(&self, err: &str) {
        let err2 = &self.get_err().unwrap() ;
        if err2 != err {
            panic!("invalid error, expect {}, got {}", err, err2);
        }
    }
}

impl fmt::Display for ChainTesterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref value) = self.json {
            write!(f, "{}", serde_json::to_string_pretty(value).unwrap())
        } else {
            if let Some(ref err) = self.error_string {
                write!(f, "{}", err)
            } else {
                write!(f, "{}", "Unknown error")
            }
        }
    }
}

impl fmt::Debug for ChainTesterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref value) = self.json {
            write!(f, "{}", serde_json::to_string_pretty(&value).unwrap())
        } else {
            if let Some(ref err) = self.error_string {
                write!(f, "{}", err)
            } else {
                write!(f, "{}", "Unknown error")
            }
        }
    }
}

pub struct TransactionReturn {
    pub value: Value
}

impl fmt::Display for TransactionReturn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.value).unwrap())
    }
}

impl fmt::Debug for TransactionReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.value).unwrap())
    }
}

pub type Result<T> = core::result::Result<T, ChainTesterError>;

pub struct GetTableRowsPrams<'a> {
    pub json: bool,
    pub code: &'a str,
    pub scope: &'a str,
    pub table: &'a str,
    pub lower_bound: &'a str,
    pub upper_bound: &'a str,
    pub limit: i64,
    pub key_type: &'a str,
    pub index_position: &'a str,
    pub reverse: bool,
    pub show_payer: bool,
}

impl<'a> Default for GetTableRowsPrams<'a> {
    fn default() -> Self {
        Self {
            json: true,
            code: "",
            scope: "",
            table: "",
            lower_bound: "",
            upper_bound: "",
            limit: 10,
            key_type: "",
            index_position: "",
            reverse: false,
            show_payer: false
        }
    }
}

pub struct VMAPIClient {
    vm_api_client: Option<ApplySyncClient<ClientInputProtocol, ClientOutputProtocol>>,
    apply: Option<fn(u64, u64, u64)>,
}

pub struct ChainTesterClient {
    client: Option<IPCChainTesterSyncClient<ClientInputProtocol, ClientOutputProtocol>>,
}

lazy_static! {
    static ref VM_API_CLIENT: Mutex<VMAPIClient> = Mutex::new(VMAPIClient::new());
}

lazy_static! {
    static ref CHAIN_TESTER_CLIENT: Mutex<ChainTesterClient> = Mutex::new(ChainTesterClient::new());
}

pub fn get_vm_api_client() -> MutexGuard<'static, VMAPIClient> {
    let mut ret = VM_API_CLIENT.lock().unwrap();
    if ret.vm_api_client.is_none() {
        ret.init();
    }
    return ret;
}

pub fn close_vm_api_client() {
    let mut ret = VM_API_CLIENT.lock().unwrap();
    ret.close();
}

impl VMAPIClient {
    fn new() -> Self {
        VMAPIClient{vm_api_client: None, apply: None}
    }

    pub fn init(&mut self) {
        if self.vm_api_client.is_none() {
            let host = crate::get_debugger_config().vm_api_server_address.clone();
            let port = crate::get_debugger_config().vm_api_server_port;
            let client = new_vm_api_client(&host, port).unwrap();
            self.vm_api_client = Some(client);
        }
    }

    pub fn set_apply(&mut self, apply: fn(u64, u64, u64)) {
        self.apply = Some(apply);
    }

    pub fn get_apply(&self) -> Option<fn(u64, u64, u64)> {
        return self.apply;
    }

    pub fn close(&mut self) {
        if self.vm_api_client.is_some() {
            self.vm_api_client = None;
        }
    }

    // pub fn client(&mut self) -> &mut ApplySyncClient<ClientInputProtocol, ClientOutputProtocol> {
    //     self.vm_api_client.as_mut().unwrap()
    // }
}

impl Deref for VMAPIClient {
    type Target = ApplySyncClient<ClientInputProtocol, ClientOutputProtocol>;

    fn deref(&self) -> &ApplySyncClient<ClientInputProtocol, ClientOutputProtocol>
    {
        self.vm_api_client.as_ref().unwrap()
    }
}

impl DerefMut for VMAPIClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.vm_api_client.as_mut().unwrap()
    }
}
// 

impl ChainTesterClient {
    fn new() -> Self {
        better_panic::install();
        ChainTesterClient{client: None}
    }

    fn init(&mut self) {
        if self.client.is_some() {
            return;
        }

        let host = crate::get_debugger_config().debugger_server_address.clone();
        let port = crate::get_debugger_config().debugger_server_port;

        let mut c = TTcpChannel::new();
    
        // open the underlying TCP stream
        println!("connecting to debugger server on {}:{}", host, port);
        c.open(&format!("{}:{}", host, port)).unwrap();    
        println!("debugger server connected");
        
        // clone the TCP channel into two halves, one which
        // we'll use for reading, the other for writing
        let (i_chan, o_chan) = c.split().unwrap();
    
        // wrap the raw sockets (slow) with a buffered transport of some kind
        let i_tran = TBufferedReadTransport::new(i_chan);
        let o_tran = TBufferedWriteTransport::new(o_chan);
    
        // now create the protocol implementations
        let i_prot = TBinaryInputProtocol::new(i_tran, false);
        let o_prot = TBinaryOutputProtocol::new(o_tran, true);
    
        let mut client = IPCChainTesterSyncClient::new(i_prot, o_prot);
        client.init_vm_api().unwrap();
        let _ = get_vm_api_client(); //init vm api client

        client.init_apply_request().unwrap();
        let _= crate::server::get_apply_request_server(); //init apply request server

        self.client = Some(client);

    }

    pub fn close(&mut self) {
        if self.client.is_some() {
            self.client = None;
        }
    }
}

impl Deref for ChainTesterClient {
    type Target = IPCChainTesterSyncClient<ClientInputProtocol, ClientOutputProtocol>;

    fn deref(&self) -> &IPCChainTesterSyncClient<ClientInputProtocol, ClientOutputProtocol>
    {
        self.client.as_ref().unwrap()
    }
}

impl DerefMut for ChainTesterClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.client.as_mut().unwrap()
    }
}

pub fn get_chain_tester_client() -> MutexGuard<'static, ChainTesterClient> {
    let mut ret = CHAIN_TESTER_CLIENT.lock().unwrap();
    if ret.client.is_none() {
        ret.init();
    }
    return ret;
}

pub fn close_chain_tester_client() {
    let mut ret = CHAIN_TESTER_CLIENT.lock().unwrap();
    ret.close();
}

pub struct ChainTester {
    id: i32,
}

fn parse_ret(ret: &thrift::Result<String>) -> Result<Value> {
    match ret {
        Ok(ret) => {
            println!("+++++++parse_ret:{}", ret);
            let tx: Value = serde_json::from_str(&ret).map_err(|err| {
                ChainTesterError{json: None, error_string: Some(err.to_string())}
            })?;

            if tx.get("except").is_some() {
                Err(ChainTesterError{json: Some(tx), error_string: None})
            } else {
                Ok(tx)
            }
        }
        Err(err) => {
            Err(ChainTesterError{
                json: None, error_string: Some(format!("{:?}", err)),
            })
        }
    }
}

fn parse_ret2(ret: &thrift::Result<Vec<u8>>) -> Result<Value> {
    match ret {
        Ok(ret) => {
            let tx: Value = serde_json::from_slice(ret).map_err(|err| {
                ChainTesterError{json: None, error_string: Some(err.to_string())}
            })?;

            if tx.get("except").is_some() {
                Err(ChainTesterError{json: Some(tx), error_string: None})
            } else {
                Ok(tx)
            }
        }
        Err(err) => {
            Err(ChainTesterError{
                json: None, error_string: Some(format!("{:?}", err)),
            })
        }
    }
}

impl ChainTester {
    pub fn new() -> Self {
        Self { id: get_chain_tester_client().new_chain(true).unwrap() }
    }

    pub fn new_ex(initialize: bool) -> Self {
        Self { id: get_chain_tester_client().new_chain(initialize).unwrap() }
    }

    fn client(&mut self) -> MutexGuard<'static, ChainTesterClient> {
        get_chain_tester_client()
    }

    pub fn free(&mut self) {
        self.client().free_chain(self.id).unwrap();
    }

    pub fn produce_block(&mut self) {
        self.client().produce_block(self.id, 0).unwrap()
    }

    pub fn produce_block_ex(&mut self, next_block_skip_seconds: i64) {
        self.client().produce_block(self.id, next_block_skip_seconds).unwrap()
    }

    pub fn enable_debug_contract(&mut self, contract: &str, enable: bool) -> thrift::Result<()> {
        self.client().enable_debug_contract(self.id, contract.into(), enable)
    }

    pub fn is_debug_contract_enabled(&mut self, contract: &str) -> thrift::Result<bool> {
        self.client().is_debug_contract_enabled(self.id, contract.into())
    }

    pub fn import_key(&mut self, pub_key: &str, priv_key: &str) -> bool {
        self.client().import_key(self.id, pub_key.into(), priv_key.into()).unwrap()
    }

    pub fn get_info(&mut self) -> Result<Value> {
        let ret = self.client().get_info(self.id);
        parse_ret(&ret)
    }

    pub fn create_key(&mut self) -> Result<Value> {
        let ret = self.client().create_key("K1".into());
        parse_ret(&ret)
    }

    pub fn create_key_ex(&mut self, key_type: &str) -> Result<Value> {
        let ret = self.client().create_key(key_type.into());
        parse_ret(&ret)
    }

    pub fn get_account(&mut self, account: &str) -> Result<Value> {
        let ret = self.client().get_account(self.id, account.into());
        parse_ret(&ret)
    }

    pub fn create_account(&mut self, creator: &str, account: &str, owner_key: &str, active_key: &str, ram_bytes: i64, stake_net: i64, stake_cpu: i64) -> Result<Value> {
        let ret = self.client().create_account(self.id, creator.into(), account.into(), owner_key.into(), active_key.into(), ram_bytes, stake_net, stake_cpu);
        parse_ret(&ret)
    }

    pub fn push_action(&mut self, account: &str, action: &str, arguments: ActionArguments, permissions: &str) -> Result<TransactionReturn> {
        let _account = String::from(account);
        let _action = String::from(action);

        let _arguments;
        match &arguments {
            ActionArguments::String(s) => {
                _arguments = s.clone();
            }
            ActionArguments::Binary(b) => {
                _arguments = hex::encode(b);
            }
        }
        let _permissions = String::from(permissions);
        match self.client().push_action(self.id, _account, _action, _arguments, _permissions) {
            Ok(ret) => {
                let tx: Value = serde_json::from_slice(&ret).map_err(|err| {
                    ChainTesterError{json: None, error_string: Some(err.to_string())}
                })?;
        
                if tx.get("except").is_some() {
                    Err(ChainTesterError{json: Some(tx), error_string: None})
                } else {
                    Ok(TransactionReturn{value: tx})
                }
            }
            Err(err) => {
                Err(ChainTesterError{
                    json: None, error_string: Some(format!("{:?}", err)),
                })
            }
        }
    }

    pub fn deploy_contract(&mut self, account: &str, wasm_file: &str, abi_file: &str) -> Result<Value> {
        // abi_file.is_empty()
        let wasm = fs::read(wasm_file).unwrap();        
        let hex_wasm = hex::encode(wasm);

        let set_code_args = format!(
            r#"
            {{
                "account": "{}",
                "vmtype": 0,
                "vmversion": 0,
                "code": "{}"
             }}
            "#,
            account,
            hex_wasm
        );

        let permissions = format!(
            r#"
            {{
                "{}": "active"
            }}
            "#,
            account,
        );

        let raw_set_code_args = self.client().pack_action_args(self.id, "eosio".into(), "setcode".into(), set_code_args).unwrap();
        let mut actions: Vec<Box<Action>> = Vec::new();
        let setcode = Action{
            account: Some("eosio".into()),
            action: Some("setcode".into()),
            permissions: Some(permissions.clone()),
            arguments: Some(hex::encode(raw_set_code_args)),
        };
        actions.push(Box::new(setcode));

        if !abi_file.is_empty() {
            // let abi = fs::read(Path::new(abi_file)).unwrap();
            let abi = fs::read_to_string(abi_file).unwrap();
            let raw_abi = self.client().pack_abi(abi).unwrap();
            let hex_raw_abi = hex::encode(raw_abi);
            let set_abi_args = format!(
                r#"
                {{
                    "account": "{}",
                    "abi": "{}"
                 }}
                "#,
                account,
                hex_raw_abi
            );

            let raw_setabi = self.client().pack_action_args(self.id, "eosio".into(), "setabi".into(), set_abi_args).unwrap();
            let setabi = Action{
                account: Some("eosio".into()),
                action: Some("setabi".into()),
                permissions: Some(permissions.clone()),
                arguments: Some(hex::encode(raw_setabi)),
            };

            actions.push(Box::new(setabi));    
        }

        self.push_actions(actions)
    }

    pub fn push_actions(&mut self, actions: Vec<Box<Action>>) -> Result<Value> {
        let ret = self.client().push_actions(self.id, actions);
        parse_ret2(&ret)
    }

    pub fn get_table_rows<'a>(&mut self, json: bool, code: &'a str, scope: &'a str, table: &'a str, lower_bound: &'a str, upper_bound: &'a str, limit: i64) -> Result<Value> {
        let param = GetTableRowsPrams {
            json: json,
            code: code,
            scope: scope,
            table: table,
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            limit: limit,
            key_type: "",
            index_position: "",
            reverse: false,
            show_payer: false,
        };
        return self.get_table_rows_ex(&param);

    }

    pub fn get_table_rows_ex(&mut self, params: &GetTableRowsPrams) -> Result<Value> {
        let ret = self.client().get_table_rows(self.id,
            params.json,
            params.code.into(),
            params.scope.into(),
            params.table.into(),
            params.lower_bound.into(),
            params.upper_bound.into(),
            params.limit,
            params.key_type.into(),
            params.index_position.into(),
            params.reverse,
            params.show_payer,
        );
        parse_ret(&ret)
    }
}

pub enum ActionArguments {
    String(String),
    Binary(Vec<u8>),
}

impl From<String> for ActionArguments {
    fn from(value: String) -> Self {
        ActionArguments::String(value)
    }
}

impl From<&str> for ActionArguments {
    fn from(value: &str) -> Self {
        ActionArguments::String(String::from(value))
    }
}

impl From<Vec<u8>> for ActionArguments {
    fn from(value: Vec<u8>) -> Self {
        ActionArguments::Binary(value)
    }
}

impl Drop for ChainTester {
    fn drop(&mut self) {
        self.free();
    }
}

pub fn new_vm_api_client(
    host: &str,
    port: u16,
) -> thrift::Result<ApplySyncClient<ClientInputProtocol, ClientOutputProtocol>> {
    let mut c = TTcpChannel::new();

    // open the underlying TCP stream
    println!("connecting to VM API server on {}:{}", host, port);
    //wait for vm api server to start
    thread::sleep(Duration::from_micros(10));
    let remote_address = format!("{}:{}", host, port);
    for i in 0..=10 {
        match c.open(&remote_address) {
            Ok(()) => {
                break;
            }
            Err(err) => {
                if i == 10 {
                    panic!("{}", err)
                } else {
                    println!("+++++++vm_api_client error: {}", err);
                    thread::sleep(Duration::from_micros(200));    
                }
            }
        }
    }

    println!("VM API server connected!");

    // clone the TCP channel into two halves, one which
    // we'll use for reading, the other for writing
    let (i_chan, o_chan) = c.split()?;

    // wrap the raw sockets (slow) with a buffered transport of some kind
    let i_tran = TBufferedReadTransport::new(i_chan);
    let o_tran = TBufferedWriteTransport::new(o_chan);

    // now create the protocol implementations
    let i_prot = TBinaryInputProtocol::new(i_tran, false);
    let o_prot = TBinaryOutputProtocol::new(o_tran, true);
    // we're done!
    Ok(ApplySyncClient::new(i_prot, o_prot))
}

///
pub fn n2s(value: u64) -> String {
	let charmap = ".12345abcdefghijklmnopqrstuvwxyz".as_bytes();
	// 13 dots
	let mut s: [u8; 13] = ['.' as u8, '.'  as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8];
	let mut tmp = value;
	for i in 0..13 {
		let c: u8;
		if i == 0 {
			c = charmap[(tmp&0x0f) as usize];
		} else {
			c = charmap[(tmp&0x1f) as usize];
		}
		s[12-i] = c;
		if i == 0 {
			tmp >>= 4
		} else {
			tmp >>= 5
		}
	}

	let mut i = s.len() - 1;
	while i != 0 {
		if s[i] != '.' as u8 {
			break
		}
        i -= 1;
	}

    let r = match String::from_utf8(s[0..i+1].to_vec()) {
        Ok(v) => v,
        Err(_) => String::from(""),
    };
    return r;
}

pub fn set_apply(apply: fn(u64, u64, u64)) {
    get_vm_api_client().set_apply(apply);
}
