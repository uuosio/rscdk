use log::warn;

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::panic;

use thrift::protocol::{
    TInputProtocol, TInputProtocolFactory, TOutputProtocol, TOutputProtocolFactory,
};
use thrift::transport::{TIoChannel, TReadTransportFactory, TTcpChannel, TWriteTransportFactory};

use thrift::server::{
    TProcessor
};
use thrift::TransportErrorKind;

use crate::interfaces::{Uint64};
use crate::interfaces::TApplySyncClient;

use crate::client;

extern "Rust" {
	fn native_apply(receiver: u64, first_receiver: u64, action: u64);
}

pub struct IPCServer<PRC, RTF, IPF, WTF, OPF>
where
    PRC: TProcessor + Send + Sync + 'static,
    RTF: TReadTransportFactory + 'static,
    IPF: TInputProtocolFactory + 'static,
    WTF: TWriteTransportFactory + 'static,
    OPF: TOutputProtocolFactory + 'static,
{
    r_trans_factory: RTF,
    i_proto_factory: IPF,
    w_trans_factory: WTF,
    o_proto_factory: OPF,
    processor: Arc<PRC>,
    pub cnn: Option<IncomingConnection<PRC>>,
    // worker_pool: ThreadPool,
}

impl<PRC, RTF, IPF, WTF, OPF> IPCServer<PRC, RTF, IPF, WTF, OPF>
where
    PRC: TProcessor + Send + Sync + 'static,
    RTF: TReadTransportFactory + 'static,
    IPF: TInputProtocolFactory + 'static,
    WTF: TWriteTransportFactory + 'static,
    OPF: TOutputProtocolFactory + 'static,
{
    /// Create a `IPCServer`.
    ///
    /// Each accepted connection has an input and output half, each of which
    /// requires a `TTransport` and `TProtocol`. `IPCServer` uses
    /// `read_transport_factory` and `input_protocol_factory` to create
    /// implementations for the input, and `write_transport_factory` and
    /// `output_protocol_factory` to create implementations for the output.
    pub fn new(
        read_transport_factory: RTF,
        input_protocol_factory: IPF,
        write_transport_factory: WTF,
        output_protocol_factory: OPF,
        processor: PRC,
        num_workers: usize,
    ) -> IPCServer<PRC, RTF, IPF, WTF, OPF> {
        IPCServer {
            r_trans_factory: read_transport_factory,
            i_proto_factory: input_protocol_factory,
            w_trans_factory: write_transport_factory,
            o_proto_factory: output_protocol_factory,
            processor: Arc::new(processor),
            cnn: None,
            // worker_pool: ThreadPool::with_name("Thrift service processor".to_owned(), num_workers),
        }
    }

    /// Listen for incoming connections on `listen_address`.
    ///
    /// `listen_address` should implement `ToSocketAddrs` trait.
    ///
    /// Return `()` if successful.
    ///
    /// Return `Err` when the server cannot bind to `listen_address` or there
    /// is an unrecoverable error.
    pub fn listen<A: ToSocketAddrs>(&mut self, listen_address: A) -> thrift::Result<()> {
        let listener = TcpListener::bind(listen_address)?;
        let stream = listener.accept();

        match stream {
            Ok((s, _addr)) => {
                let (mut i_prot, mut o_prot) = self.new_protocols_for_connection(s)?;
                let processor = self.processor.clone();
                match handle_incoming_connection(processor, &mut i_prot, &mut o_prot) {
                    Ok(()) => {},
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
            Err(e) => {
                warn!("failed to accept remote connection with error {:?}", e);
            }
        }

        Ok(())
    }

    pub fn accept<A: ToSocketAddrs>(&mut self, listen_address: A) -> thrift::Result<()> {
        let listener = TcpListener::bind(listen_address)?;
        let stream = listener.accept();
        match stream {
            Ok((s, _addr)) => {
                let (i_prot, o_prot) = self.new_protocols_for_connection(s)?;
                let processor = self.processor.clone();
                self.cnn = Some(IncomingConnection {
                    processor: processor,
                    i_prot: i_prot,
                    o_prot: o_prot,
                    end_loop: false,
                });
                Ok(())
            }
            Err(e) => {
                warn!("failed to accept remote connection with error {:?}", e);
                Err(
                    thrift::Error::Application(
                      thrift::ApplicationError::new(
                        thrift::ApplicationErrorKind::InternalError,
                        format!("{}", e)
                      )
                    )
                )
            }
        }
    }

    pub fn handle_apply_request(&mut self) -> thrift::Result<()> {
        let mut cnn = self.cnn.as_mut().unwrap();
        handle_incoming_connection_ex(&mut cnn)
    }

    pub fn end_loop(&mut self) {
        self.cnn.as_mut().unwrap().end_loop = true;
    }

    fn new_protocols_for_connection(
        &mut self,
        stream: TcpStream,
    ) -> thrift::Result<(
        Box<dyn TInputProtocol + Send>,
        Box<dyn TOutputProtocol + Send>,
    )> {
        // create the shared tcp stream
        let channel = TTcpChannel::with_stream(stream);

        // split it into two - one to be owned by the
        // input tran/proto and the other by the output
        let (r_chan, w_chan) = channel.split()?;

        // input protocol and transport
        let r_tran = self.r_trans_factory.create(Box::new(r_chan));
        let i_prot = self.i_proto_factory.create(r_tran);

        // output protocol and transport
        let w_tran = self.w_trans_factory.create(Box::new(w_chan));
        let o_prot = self.o_proto_factory.create(w_tran);

        Ok((i_prot, o_prot))
    }
}

pub struct IncomingConnection<PRC>
where
    PRC: TProcessor,
{
    pub processor: Arc<PRC>,
    pub i_prot: Box<dyn TInputProtocol + Send>,
    pub o_prot: Box<dyn TOutputProtocol + Send>,
    pub end_loop: bool,
}

impl<PRC> IncomingConnection<PRC>
where
    PRC: TProcessor,
{
    pub fn end_loop(&mut self) {
        self.end_loop = true;
    }
}

fn handle_incoming_connection<PRC>(
    processor: Arc<PRC>,
    i_prot: &mut Box<dyn TInputProtocol + Send>,
    o_prot: &mut Box<dyn TOutputProtocol + Send>,
) -> thrift::Result<()>
where
    PRC: TProcessor,
{
    let i_prot = i_prot;
    let o_prot = o_prot;
    loop {
        let ret = processor.process(&mut *i_prot, &mut *o_prot);
        match ret {
            Ok(()) => {}
            Err(ref err) => {
                match err {
                    thrift::Error::Transport(ref transport_err)
                        if transport_err.kind == TransportErrorKind::EndOfFile => {
                        }
                    other => {
                        warn!("processor completed with error: {:?}", other);
                    }
                }
                return ret;
            }
        }
        // return ret;
    }
}


fn handle_incoming_connection_ex<PRC>(cnn: &mut IncomingConnection<PRC>) -> thrift::Result<()>
where
    PRC: TProcessor,
{
    cnn.end_loop = false;
    let i_prot = &mut cnn.i_prot;
    let o_prot = &mut cnn.o_prot;
    loop {
        if END_APPLY.lock().unwrap().get_value() {
            END_APPLY.lock().unwrap().set_value(false);
            return Ok(())
        }
        let ret = cnn.processor.clone().process(&mut *i_prot, &mut *o_prot);
        match ret {
            Ok(()) => {}
            Err(ref err) => {
                match err {
                    thrift::Error::Transport(ref transport_err)
                        if transport_err.kind == TransportErrorKind::EndOfFile => {
                        }
                    other => {
                        warn!("processor completed with error: {:?}", other);
                    }
                }
                return ret;
            }
        }
        // return ret;
    }
}

use thrift::protocol::{
    TBinaryInputProtocolFactory,
    TBinaryOutputProtocolFactory,
};


use thrift::transport::{
    TBufferedReadTransportFactory, TBufferedWriteTransportFactory,
};

use crate::interfaces::{ApplyRequestSyncHandler, ApplyRequestSyncProcessor};

use lazy_static::lazy_static; // 1.4.0

use std::sync::{
    Mutex,
    MutexGuard
};


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

lazy_static! {
    static ref APPLY_REQUEST_SERVER: Mutex<ApplyRequestServer> = Mutex::new(ApplyRequestServer::new());
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
        let _receiver = receiver.into();
        let _first_receiver = first_receiver.into();
        let _action = action.into();

        let result = panic::catch_unwind(|| {
            unsafe {
                native_apply(_receiver, _first_receiver, _action);
            }    
        });
        match result {
            Ok(()) => {

            }
            Err(err) => {
                println!("{:?}", err);
                // crate::client::get_vm_api_client().end_apply().unwrap();
                // panic!("{:?}", err);
            }
        }
        println!("native_apply done!");
        // println!("\x1b[92m[({},{})->{}]: CONSOLE OUTPUT END   =====================\x1b[0m", n2s(_receiver), n2s(_action), n2s(_first_receiver));
        Ok(1)
    }

    fn handle_apply_end(&self) -> thrift::Result<i32> {
        END_APPLY.lock().unwrap().set_value(true);
        Ok(1)
    }
}

pub struct ApplyRequestServer {
    server: IPCServer<ApplyRequestSyncProcessor<ApplyRequestHandler>, TBufferedReadTransportFactory, TBinaryInputProtocolFactory, TBufferedWriteTransportFactory, TBinaryOutputProtocolFactory>,
}

impl ApplyRequestServer {
    pub fn new() -> Self {
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

pub fn run_apply_request_server(_port: u16)  -> thrift::Result<()> {
    get_apply_request_server().server.handle_apply_request()
}

pub fn get_apply_request_server() -> MutexGuard<'static, ApplyRequestServer> {
    let mut ret = APPLY_REQUEST_SERVER.lock().unwrap();
    if ret.server.cnn.is_none() {
        println!("++++++++++++apply_request server: waiting for connection");
        ret.server.accept("127.0.0.1:9091").unwrap();
    }
    return ret;
}
