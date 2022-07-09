use std::{thread, time::Duration};

use clap::{clap_app, value_t};

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
};

use crate::chaintester::{IPCChainTesterSyncClient, TIPCChainTesterSyncClient, ApplySyncClient, TApplySyncClient};

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

use crate::chaintester::{ApplyRequestSyncHandler, ApplyRequestSyncProcessor};
use crate::chaintester::{Uint64};


fn new_client(
    host: &str,
    port: u16,
) -> thrift::Result<IPCChainTesterSyncClient<ClientInputProtocol, ClientOutputProtocol>> {
    let mut c = TTcpChannel::new();

    // open the underlying TCP stream
    println!("connecting to chaintester server on {}:{}", host, port);
    c.open(&format!("{}:{}", host, port))?;

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
    Ok(IPCChainTesterSyncClient::new(i_prot, o_prot))
}

pub fn run(port: u16, i: usize) -> thrift::Result<()> {
    // build our client and connect to the host:port
    let mut client = new_client("127.0.0.1", port)?;

    println!("push_action return: {}", client.push_action(1, String::from("hello") + &i.to_string())?);

    Ok(())
}

fn new_vm_api_client(
    host: &str,
    port: u16,
) -> thrift::Result<ApplySyncClient<ClientInputProtocol, ClientOutputProtocol>> {
    let mut c = TTcpChannel::new();

    // open the underlying TCP stream
    println!("connecting to chaintester server on {}:{}", host, port);
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

pub fn run_vm_api() -> thrift::Result<()> {
    let mut client = new_vm_api_client("127.0.0.1", 9092)?;
    println!("ok here!");
    client.prints(String::from("hello, worlddddd"))?;
    Ok(())
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

impl ApplyRequestSyncHandler for ApplyRequestHandler {
    fn handle_apply_request(&self, receiver: Uint64, first_receiver: Uint64, action: Uint64) -> thrift::Result<i32> {
        match run_vm_api() {
            Ok(()) => {}
            Err(err) => {
                panic!("{}", err);
            }
        }
        Ok(1)
    }

    fn handle_apply_end(&self) -> thrift::Result<i32> {
        Ok(1)
    }
}
