use std::{thread, time::Duration};
use std::ops::{Deref, DerefMut};

use clap::{clap_app, value_t};

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
};

use crate::interfaces::{IPCChainTesterSyncClient, TIPCChainTesterSyncClient, ApplySyncClient, TApplySyncClient};

type ClientInputProtocol = TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>;
type ClientOutputProtocol = TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>;


use std::convert::{From, Into};
use std::default::Default;

use thrift::protocol::{
    TBinaryInputProtocolFactory,
    TBinaryOutputProtocolFactory,
    TInputProtocolFactory,
    TOutputProtocolFactory,
};

use thrift::server::{
    TProcessor
};

use crate::server::IPCServer;

use thrift::transport::{
    TBufferedReadTransportFactory, TBufferedWriteTransportFactory,
    TReadTransportFactory,
    TWriteTransportFactory,
};

use crate::interfaces::{ApplyRequestSyncHandler, ApplyRequestSyncProcessor};
use crate::interfaces::{Uint64};

use lazy_static::lazy_static; // 1.4.0
use std::sync::{
    Mutex,
    MutexGuard
};

lazy_static! {
    static ref VM_API_CLIENT: Mutex<VMAPIClient> = Mutex::new(VMAPIClient::new());
}

// 

pub fn get_vm_api_client() -> MutexGuard<'static, VMAPIClient> {
    let mut ret = VM_API_CLIENT.lock().unwrap();
    if ret.vm_api_client.is_none() {
        ret.init();
    }
    return ret;
}

pub struct VMAPIClient {
    vm_api_client: Option<ApplySyncClient<ClientInputProtocol, ClientOutputProtocol>>,
}

impl VMAPIClient {
    fn new() -> Self {
        VMAPIClient{vm_api_client: None}
    }

    fn init(&mut self) {
        if self.vm_api_client.is_none() {
            println!("+++++++++++VMAPIClient.init");
            let client = new_vm_api_client("127.0.0.1", 9092).unwrap();
            self.vm_api_client = Some(client);
            println!("+++++++++++VMAPIClient.end");
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

extern "Rust" {
	fn native_apply(receiver: u64, first_receiver: u64, action: u64);
}

pub struct ChainTester {
    id: i32,
    client: IPCChainTesterSyncClient<ClientInputProtocol, ClientOutputProtocol>,
}

impl ChainTester {
    pub fn new() -> Self {
        Self { client: Self::new_client(), id: 0 }
    }

    fn new_client() -> IPCChainTesterSyncClient<ClientInputProtocol, ClientOutputProtocol> {
        let host = "127.0.0.1";
        let port = 9090;

        let mut c = TTcpChannel::new();
    
        // open the underlying TCP stream
        println!("connecting to interfaces server on {}:{}", host, port);
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
    
        IPCChainTesterSyncClient::new(i_prot, o_prot)
    }

    pub fn new_chain(&mut self) {
        self.id = self.client.new_chain().unwrap();
    }

    pub fn free_chain(&mut self) {
        self.client.free_chain(self.id).unwrap();
    }

    pub fn push_action(&mut self, account: &str, action: &str, arguments: &str, permissions: &str) {
        let _account = String::from(account);
        let _action = String::from(action);
        let _arguments = String::from(arguments);
        let _permissions = String::from(permissions);
        self.client.push_action(self.id, _account, _action, _arguments, _permissions).unwrap();
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
    println!("+++++++++go here");

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


pub fn run_apply_request_server(port: u16)  -> thrift::Result<()> {
    let listen_address = format!("127.0.0.1:{}", port);

    println!("binding to {}", listen_address);

    let i_tran_fact = TBufferedReadTransportFactory::new();
    let i_prot_fact = TBinaryInputProtocolFactory::new();

    let o_tran_fact = TBufferedWriteTransportFactory::new();
    let o_prot_fact = TBinaryOutputProtocolFactory::new();

    // demux incoming messages
    let processor = ApplyRequestSyncProcessor::new(ApplyRequestHandler {
        ..Default::default()
    });

    // create the server and start listening
    let mut server = IPCServer::new(
        i_tran_fact,
        i_prot_fact,
        o_tran_fact,
        o_prot_fact,
        processor,
        10,
    );
    server.listen(&listen_address)
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

use std::convert::TryInto;

fn to_fixed_array(v: Vec<u8>) -> [u8; 8] {
    v.try_into()
        .unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 8, v.len()))
}

fn to_u64(value: Uint64) -> u64 {
    if value.raw_value.is_none() {
        panic!("bad raw Uint64 value 1");
    }
    return u64::from_le_bytes(to_fixed_array(value.raw_value.unwrap()));
}

impl ApplyRequestSyncHandler for ApplyRequestHandler {
    fn handle_apply_request(&self, receiver: Uint64, first_receiver: Uint64, action: Uint64) -> thrift::Result<i32> {
        println!("ok here!");
        let _receiver = to_u64(receiver);
        let _first_receiver = to_u64(first_receiver);
        let _action = to_u64(action);
        println!("+++++++++handle_apply_request");
        unsafe {
            native_apply(_receiver, _first_receiver, _action);
        }

        Ok(1)
    }

    fn handle_apply_end(&self) -> thrift::Result<i32> {
        Ok(1)
    }
}
