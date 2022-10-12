use crate::error::ClientError;
use crate::event::ClientEvent;
use crate::crypto::packet::{Packet, PacketDetail};
use atri_executor::Executor;
use dashmap::DashMap;
use futures::StreamExt;
use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::network::connector;
use crate::network::connector::Connector;

pub struct RequestClient {
    uin: u64,
    seq: AtomicU16,
    seq_packet_receiver: DashMap<u16, futures::channel::oneshot::Sender<Packet>>,
}

impl RequestClient {
    pub fn new() -> Self {
        Self {
            uin: 0,
            seq: AtomicU16::new(0),
            seq_packet_receiver: DashMap::new(),
        }
    }

    pub fn uin(&self) -> ClientResult<u64> {
        match self.uin {
            0 => Err(ClientError::NotInitialized),
            valid => Ok(valid),
        }
    }

    pub fn next_seq(&self) -> u16 {
        self.seq.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn decode_packet(&self, payload: &[u8]) {}
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Client {
    base: Arc<RequestClient>,
    request_sender: futures::channel::mpsc::UnboundedSender<Packet>,
}

type ClientResult<T> = Result<T, ClientError>;

impl Client {
    #[inline]
    pub fn builder() -> ClientBuilder<(), (), ()> {
        ClientBuilder::default()
    }
}

impl Client {
    pub fn uin(&self) -> ClientResult<u64> {
        self.base.uin()
    }
}

pub struct ClientBuilder<F, E, C> {
    handler: F,
    base: RequestClient,
    executor: E,
    packet_send_rx: futures::channel::mpsc::UnboundedReceiver<Packet>,
    packet_send_tx: futures::channel::mpsc::UnboundedSender<Packet>,
    connector: C,
}

impl ClientBuilder<(), (), ()> {
    pub fn new() -> Self {
        let (tx, rx) = futures::channel::mpsc::unbounded();

        Self {
            handler: (),
            base: RequestClient::new(),
            executor: (),
            packet_send_rx: rx,
            packet_send_tx: tx,
            connector: (),
        }
    }
}

impl Default for ClientBuilder<(), (), ()> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<F, E, C> ClientBuilder<F, E, C> {
    #[inline]
    pub fn with_handler<H, Fu>(self, handler: H) -> ClientBuilder<H, E, C>
    where
        H: Fn(ClientEvent) -> Fu,
        H: Send + 'static,
        Fu: Future<Output = ()>,
        Fu: Send + 'static,
    {
        let Self {
            base,
            executor,
            packet_send_rx,
            packet_send_tx,
            connector: stream,
            ..
        } = self;

        ClientBuilder {
            handler,
            base,
            executor,
            packet_send_rx,
            packet_send_tx,
            connector: stream,
        }
    }

    #[inline]
    pub fn with_default_handler(self) -> ClientBuilder<fn(ClientEvent) -> NopFuture, E, C> {
        fn _handle(_: ClientEvent) -> NopFuture {
            NopFuture
        }

        self.with_handler(_handle)
    }

    #[inline]
    pub fn with_executor<T: Executor>(self, executor: T) -> ClientBuilder<F, T, C> {
        let Self {
            handler,
            base,
            packet_send_rx,
            packet_send_tx,
            connector: stream,
            ..
        } = self;

        ClientBuilder {
            handler,
            base,
            executor,
            packet_send_rx,
            packet_send_tx,
            connector: stream,
        }
    }

    #[inline]
    pub fn with_connector<T>(self, connector: T) -> ClientBuilder<F, E, T> {
        let Self {
            handler,
            base,
            executor,
            packet_send_rx,
            packet_send_tx,
            ..
        } = self;

        ClientBuilder {
            handler,
            base,
            executor,
            packet_send_rx,
            packet_send_tx,
            connector,
        }
    }
}

impl<F, Fu, E, C> ClientBuilder<F, E, C>
where
    F: Fn(ClientEvent) -> Fu,
    F: Send + 'static,
    Fu: Future<Output = ()>,
    Fu: Send + 'static,
    E: Executor,
    C: Connector,
{
    pub async fn login(self) {
        
    }

    pub fn run(self) -> Client {
        let client = Arc::new(self.base);

        let install = client.clone();
        self.executor.spawn(async move {
            let mut pkt_rx = self.packet_send_rx;
            while let Some(pkt) = pkt_rx.next().await {
                let len = pkt.command.len() + 20;
                let mut out_pkt = Vec::<u8>::with_capacity(len);
                out_pkt.extend_from_slice(&len.to_be_bytes());
                out_pkt.extend_from_slice(&0x0Bu32.to_be_bytes());

                match pkt.packet_detail {
                    PacketDetail::Uin {
                        seq
                    } => {

                    },
                    PacketDetail::Login => {

                    }
                }

                out_pkt.extend_from_slice(&pkt.seq.to_be_bytes());
                out_pkt.push(0);
            }
        });

        Client {
            base: client,
            request_sender: self.packet_send_tx,
        }
    }
}

struct NeedLogin;

pub struct NopFuture;

impl Future for NopFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
