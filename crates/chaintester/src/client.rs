use std::fmt;

use std::{fs, path::Path};
use std::{thread, time::Duration};
use std::ops::{Deref, DerefMut};

use serde_json::{Value, Map};

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
};

use crate::interfaces::{IPCChainTesterSyncClient, TIPCChainTesterSyncClient, ApplySyncClient, Action};

type ClientInputProtocol = TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>;
type ClientOutputProtocol = TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>;


use std::convert::{From, Into};
use std::default::Default;

use thrift::protocol::{
    TBinaryInputProtocolFactory,
    TBinaryOutputProtocolFactory,
};

use crate::server::IPCServer;

use thrift::transport::{
    TBufferedReadTransportFactory, TBufferedWriteTransportFactory,
};

use crate::interfaces::{ApplyRequestSyncHandler, ApplyRequestSyncProcessor};
use crate::interfaces::{Uint64};

use lazy_static::lazy_static; // 1.4.0
use std::sync::{
    Mutex,
    MutexGuard
};

extern "Rust" {
	fn native_apply(receiver: u64, first_receiver: u64, action: u64);
}

#[derive(Debug)]
pub struct TransactionError {
    json: Option<Map<String, Value>>,
    error_string: Option<String>,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref value) = self.json {
            write!(f, "{}", serde_json::to_string_pretty(&Value::Object(value.clone())).unwrap())
        } else {
            if let Some(ref err) = self.error_string {
                write!(f, "{}", err)
            } else {
                write!(f, "{}", "Unknown error")
            }
        }
    }
}

pub type Result<T> = core::result::Result<T, TransactionError>;

pub struct VMAPIClient {
    vm_api_client: Option<ApplySyncClient<ClientInputProtocol, ClientOutputProtocol>>,
}

pub struct ChainTesterClient {
    client: Option<IPCChainTesterSyncClient<ClientInputProtocol, ClientOutputProtocol>>,
}

pub struct ApplyRequestServer {
    server: IPCServer<ApplyRequestSyncProcessor<ApplyRequestHandler>, TBufferedReadTransportFactory, TBinaryInputProtocolFactory, TBufferedWriteTransportFactory, TBinaryOutputProtocolFactory>,
}

lazy_static! {
    static ref VM_API_CLIENT: Mutex<VMAPIClient> = Mutex::new(VMAPIClient::new());
}

lazy_static! {
    static ref CHAIN_TESTER_CLIENT: Mutex<ChainTesterClient> = Mutex::new(ChainTesterClient::new());
}

lazy_static! {
    static ref APPLY_REQUEST_SERVER: Mutex<ApplyRequestServer> = Mutex::new(ApplyRequestServer::new());
}

pub struct EndApply {
    value: bool
}

impl EndApply {
    pub fn set_value(&mut self, value: bool) {
        self.value = value;
    }

    pub fn get_value(&mut self) -> bool {
        return self.value;
    }
}

lazy_static! {
    pub static ref END_APPLY: Mutex<EndApply> = Mutex::new(EndApply{value: false});
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

pub fn get_apply_request_server() -> MutexGuard<'static, ApplyRequestServer> {
    let mut ret = APPLY_REQUEST_SERVER.lock().unwrap();
    if ret.server.cnn.is_none() {
        println!("++++++++++++apply_request server: waiting for connection");
        ret.server.accept("127.0.0.1:9091").unwrap();
    }
    return ret;
}

impl VMAPIClient {
    fn new() -> Self {
        VMAPIClient{vm_api_client: None}
    }

    pub fn init(&mut self) {
        if self.vm_api_client.is_none() {
            let client = new_vm_api_client("127.0.0.1", 9092).unwrap();
            self.vm_api_client = Some(client);
        }
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
        ChainTesterClient{client: None}
    }

    fn init(&mut self) {
        if self.client.is_some() {
            return;
        }

        let host = "127.0.0.1";
        let port = 9090;

        let mut c = TTcpChannel::new();
    
        // open the underlying TCP stream
        println!("connecting to ChainTester server on {}:{}", host, port);
        c.open(&format!("{}:{}", host, port)).unwrap();    
        
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
        let _= get_apply_request_server(); //init apply request server

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

impl ChainTester {
    pub fn new() -> Self {
        Self { id: get_chain_tester_client().new_chain().unwrap() }
    }

    fn client(&mut self) -> MutexGuard<'static, ChainTesterClient> {
        get_chain_tester_client()
    }

    pub fn free(&mut self) {
        self.client().free_chain(self.id).unwrap();
    }

    pub fn produce_block(&mut self) {
        self.client().produce_block(self.id).unwrap()
    }

    pub fn push_action(&mut self, account: &str, action: &str, arguments: ActionArguments, permissions: &str) -> Result<Map<String, Value>> {
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
        let ret = self.client().push_action(self.id, _account, _action, _arguments, _permissions).unwrap();

        let tx: Value = serde_json::from_slice(&ret).map_err(|err| {
            TransactionError{json: None, error_string: Some(err.to_string())}
        })?;

        let obj: &Map<String, Value> = tx.as_object().unwrap();
        if obj.get("except").is_some() {
            Err(TransactionError{json: Some(obj.clone()), error_string: None})
        } else {
            Ok(obj.clone())
        }
    }

    pub fn deploy_contract(&mut self, account: &str, wasm_file: &str, abi_file: &str) -> Result<Map<String, Value>> {
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

        let ret = self.client().push_actions(self.id, actions).unwrap();
        let tx: Value = serde_json::from_slice(&ret).map_err(|err| {
            TransactionError{json: None, error_string: Some(err.to_string())}
        })?;

        let obj: &Map<String, Value> = tx.as_object().unwrap();
        if obj.get("except").is_some() {
            Err(TransactionError{json: Some(obj.clone()), error_string: None})
        } else {
            Ok(obj.clone())
        }

    }

    pub fn push_actions(&mut self, actions: Vec<Box<Action>>) -> Result<Map<String, Value>> {
        let ret = self.client().push_actions(self.id, actions).unwrap();
        let tx: Value = serde_json::from_slice(&ret).map_err(|err| {
            TransactionError{json: None, error_string: Some(err.to_string())}
        })?;

        let obj: &Map<String, Value> = tx.as_object().unwrap();
        if obj.get("except").is_some() {
            Err(TransactionError{json: Some(obj.clone()), error_string: None})
        } else {
            Ok(obj.clone())
        }
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
    for _ in 0..10 {
        match c.open(&remote_address) {
            Ok(()) => {
                break;
            }
            Err(err) => {
                println!("+++++++vm_api_client error: {}", err);
                thread::sleep(Duration::from_micros(200));
            }
        }
    }
    println!("+++++++++connect to vm_api server successfull!");

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


impl ApplyRequestServer {
    pub fn new() -> Self {
        let listen_address = format!("127.0.0.1:{}", "9092");

        // println!("binding to {}", listen_address);
    
        let i_tran_fact = TBufferedReadTransportFactory::new();
        let i_prot_fact = TBinaryInputProtocolFactory::new();
    
        let o_tran_fact = TBufferedWriteTransportFactory::new();
        let o_prot_fact = TBinaryOutputProtocolFactory::new();
    
        // demux incoming messages
        let processor = ApplyRequestSyncProcessor::new(ApplyRequestHandler {
            ..Default::default()
        });
    
        // create the server and start listening
        Self {
            server: IPCServer::new(
                i_tran_fact,
                i_prot_fact,
                o_tran_fact,
                o_prot_fact,
                processor,
                10,
        )}
    }
}

pub fn run_apply_request_server(port: u16)  -> thrift::Result<()> {
    get_apply_request_server().server.handle_apply_request()
}

/// Handles incoming ChainTester service calls.
struct ApplyRequestHandler {
}

impl Default for ApplyRequestHandler {
    fn default() -> ApplyRequestHandler {
        ApplyRequestHandler {
        }
    }
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

impl ApplyRequestSyncHandler for ApplyRequestHandler {
    fn handle_apply_request(&self, receiver: Uint64, first_receiver: Uint64, action: Uint64) -> thrift::Result<i32> {
        let _receiver = receiver.into();
        let _first_receiver = first_receiver.into();
        let _action = action.into();
        // println!("\x1b[92m[({},{})->{}]: CONSOLE OUTPUT BEGIN =====================\x1b[0m", n2s(_receiver), n2s(_action), n2s(_first_receiver));
        unsafe {
            native_apply(_receiver, _first_receiver, _action);
        }
        // println!("\x1b[92m[({},{})->{}]: CONSOLE OUTPUT END   =====================\x1b[0m", n2s(_receiver), n2s(_action), n2s(_first_receiver));
        Ok(1)
    }

    fn handle_apply_end(&self) -> thrift::Result<i32> {
        END_APPLY.lock().unwrap().set_value(true);
        Ok(1)
    }
}
