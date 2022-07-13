use std::{thread, time::Duration};
use std::ops::{Deref, DerefMut};

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
};

use crate::interfaces::{IPCChainTesterSyncClient, TIPCChainTesterSyncClient, ApplySyncClient};

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
        let _= get_apply_request_server();

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

    pub fn push_action(&mut self, account: &str, action: &str, arguments: &str, permissions: &str) {
        let _account = String::from(account);
        let _action = String::from(action);
        let _arguments = String::from(arguments);
        let _permissions = String::from(permissions);
        self.client().push_action(self.id, _account, _action, _arguments, _permissions).unwrap();
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
    println!("connecting to interfaces server on {}:{}", host, port);
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

impl ApplyRequestSyncHandler for ApplyRequestHandler {
    fn handle_apply_request(&self, receiver: Uint64, first_receiver: Uint64, action: Uint64) -> thrift::Result<i32> {
        println!("+++++++++handle_apply_request");
        unsafe {
            native_apply(receiver.into(), first_receiver.into(), action.into());
        }
        Ok(1)
    }

    fn handle_apply_end(&self) -> thrift::Result<i32> {
        END_APPLY.lock().unwrap().set_value(true);
        Ok(1)
    }
}
