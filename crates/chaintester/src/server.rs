use log::warn;

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use threadpool::ThreadPool;

use thrift::protocol::{
    TInputProtocol, TInputProtocolFactory, TOutputProtocol, TOutputProtocolFactory,
};
use thrift::transport::{TIoChannel, TReadTransportFactory, TTcpChannel, TWriteTransportFactory};
use thrift::{ApplicationError, ApplicationErrorKind};

use thrift::server::{
    TProcessor
};
use thrift::TransportErrorKind;

#[derive(Debug)]
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
    worker_pool: ThreadPool,
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
            worker_pool: ThreadPool::with_name("Thrift service processor".to_owned(), num_workers),
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
                let (i_prot, o_prot) = self.new_protocols_for_connection(s)?;
                let processor = self.processor.clone();
                handle_incoming_connection(processor, i_prot, o_prot);
            }
            Err(e) => {
                warn!("failed to accept remote connection with error {:?}", e);
            }
        }

        Ok(())
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

fn handle_incoming_connection<PRC>(
    processor: Arc<PRC>,
    i_prot: Box<dyn TInputProtocol>,
    o_prot: Box<dyn TOutputProtocol>,
) where
    PRC: TProcessor,
{
    let mut i_prot = i_prot;
    let mut o_prot = o_prot;
    loop {
        match processor.process(&mut *i_prot, &mut *o_prot) {
            Ok(()) => {}
            Err(err) => {
                match err {
                    thrift::Error::Transport(ref transport_err)
                        if transport_err.kind == TransportErrorKind::EndOfFile => {}
                    other => warn!("processor completed with error: {:?}", other),
                }
                break;
            }
        }
    }
}
