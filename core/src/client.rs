use crate::error::ClientError;
use crate::event::ClientEvent;
use atri_executor::Executor;
use dashmap::DashMap;
use futures_util::StreamExt;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct RequestClient {
    uin: AtomicU64,
    seq: AtomicU16,
    seq_packet_recv: DashMap<u16, futures_channel::oneshot::Sender<Vec<u8>>>,
}

impl RequestClient {
    pub fn new() -> Self {
        Self {
            uin: AtomicU64::new(0),
            seq: AtomicU16::new(0),
            seq_packet_recv: DashMap::new(),
        }
    }

    pub fn uin(&self) -> ClientResult<u64> {
        match self.uin.load(Ordering::Relaxed) {
            0 => Err(ClientError::NotInitialized),
            valid => Ok(valid),
        }
    }

    pub async fn handle_packet(&self, packet: Vec<u8>) {}
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Client<F, E> {
    handler: F,
    request_client: Arc<RequestClient>,
    executor: E,
    packet_sender: futures_channel::mpsc::UnboundedSender<Vec<u8>>,
}

type ClientResult<T> = Result<T, ClientError>;

impl Client<(), ()> {
    #[inline]
    pub fn builder() -> ClientBuilder<(), (), ()> {
        ClientBuilder::default()
    }
}

impl<F, E> Client<F, E> {
    pub fn uin(&self) -> ClientResult<u64> {
        self.request_client.uin()
    }

    pub fn handler(&self) -> &F {
        &self.handler
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }
}

impl<F, Fu, E> Client<F, E>
where
    F: Fn(ClientEvent) -> Fu,
    F: Send + 'static,
    Fu: Future<Output = ()>,
    Fu: Send + 'static,
    E: Executor,
{
    pub fn request_client(&self) -> &RequestClient {
        &self.request_client
    }

    pub async fn handle(&self, event: ClientEvent) {
        (self.handler())(event).await;
    }
}

pub struct ClientBuilder<F, E, S> {
    handler: F,
    request_client: RequestClient,
    executor: E,
    packet_send_rx: futures_channel::mpsc::UnboundedReceiver<Vec<u8>>,
    packet_send_tx: futures_channel::mpsc::UnboundedSender<Vec<u8>>,
    stream: S,
}

impl ClientBuilder<(), (), ()> {
    pub fn new() -> Self {
        let (tx, rx) = futures_channel::mpsc::unbounded();

        Self {
            handler: (),
            request_client: RequestClient::new(),
            executor: (),
            packet_send_rx: rx,
            packet_send_tx: tx,
            stream: (),
        }
    }
}

impl Default for ClientBuilder<(), (), ()> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<F, E, S> ClientBuilder<F, E, S> {
    #[inline]
    pub fn with_handler<H, Fu>(self, handler: H) -> ClientBuilder<H, E, S>
    where
        H: Fn(ClientEvent) -> Fu,
        H: Send + 'static,
        Fu: Future<Output = ()>,
        Fu: Send + 'static,
    {
        let Self {
            request_client,
            executor,
            packet_send_rx,
            packet_send_tx,
            stream,
            ..
        } = self;

        ClientBuilder {
            handler,
            request_client,
            executor,
            packet_send_rx,
            packet_send_tx,
            stream,
        }
    }

    #[inline]
    pub fn with_default_handler(self) -> ClientBuilder<fn(ClientEvent) -> NopFuture, E, S> {
        fn _handle(_: ClientEvent) -> NopFuture {
            NopFuture
        }

        self.with_handler(_handle)
    }

    #[inline]
    pub fn with_executor<T: Executor>(self, executor: T) -> ClientBuilder<F, T, S> {
        let Self {
            handler,
            request_client,
            packet_send_rx,
            packet_send_tx,
            stream,
            ..
        } = self;

        ClientBuilder {
            handler,
            request_client,
            executor,
            packet_send_rx,
            packet_send_tx,
            stream,
        }
    }

    #[inline]
    pub fn with_stream<T>(self, stream: T) -> ClientBuilder<F, E, T> {
        let Self {
            handler,
            request_client,
            executor,
            packet_send_rx,
            packet_send_tx,
            ..
        } = self;

        ClientBuilder {
            handler,
            request_client,
            executor,
            packet_send_rx,
            packet_send_tx,
            stream,
        }
    }
}

impl<F, Fu, E, S> ClientBuilder<F, E, S>
where
    F: Fn(ClientEvent) -> Fu,
    F: Send + 'static,
    Fu: Future<Output = ()>,
    Fu: Send + 'static,
    E: Executor,
    S: futures_io::AsyncRead + futures_io::AsyncWrite,
{
     pub fn run(self) -> Client<F, E> {
        let client = Arc::new(self.request_client);

        let pkt_install = client.clone();
        self.executor.spawn(async move {
            let mut pkt_rx = self.packet_send_rx;
            while let Some(pkt) = pkt_rx.next().await {
                pkt_install.handle_packet(pkt).await;
            }
        });

        Client {
            handler: self.handler,
            request_client: client,
            executor: self.executor,
            packet_sender: self.packet_send_tx,
        }
    }
}

pub struct NopFuture;

impl Future for NopFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
