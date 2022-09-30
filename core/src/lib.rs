pub mod error;
pub mod event;

use crate::error::ClientError;
use crate::event::ClientEvent;
use atri_executor::Executor;
use dashmap::DashMap;
use futures_channel::oneshot;
use std::future::Future;
use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};

pub struct RequestClient {
    uin: AtomicU64,
    seq: AtomicU16,
    seq_packet_recv: DashMap<u16, oneshot::Sender<Vec<u8>>>,
    packet_send_rx: futures_channel::mpsc::UnboundedReceiver<Vec<u8>>,
    packet_send_tx: futures_channel::mpsc::UnboundedSender<Vec<u8>>,
}

impl RequestClient {
    pub fn new() -> Self {
        let (pkt_tx, pkt_rx) = futures_channel::mpsc::unbounded();

        Self {
            uin: AtomicU64::new(0),
            seq: AtomicU16::new(0),
            seq_packet_recv: DashMap::new(),
            packet_send_rx: pkt_rx,
            packet_send_tx: pkt_tx,
        }
    }

    pub fn uin(&self) -> ClientResult<u64> {
        match self.uin.load(Ordering::Relaxed) {
            0 => Err(ClientError::NotInitialized),
            valid => Ok(valid),
        }
    }
}

pub struct Client<F, E> {
    handler: F,
    request: RequestClient,
    executor: E,
}

type ClientResult<T> = Result<T, ClientError>;

impl<F, E> Client<F, E> {
    pub fn uin(&self) -> ClientResult<u64> {
        self.request.uin()
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
    pub fn new(handler: F, executor: E) -> Self {
        Self {
            handler,
            request: RequestClient::new(),
            executor,
        }
    }

    pub(crate) async fn handle(&self, event: ClientEvent) {
        (self.handler())(event).await;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use atri_executor::Executor;
    use crate::{Client, ClientEvent};

    #[test]
    fn handler() {
        async fn handle(_: ClientEvent) {
            println!("233");
        }

        let rt = atri_executor::runtime::blocking::Runtime;
        let arc = Arc::new(rt);
        let client = Client::new(handle, arc.clone());
        client.executor().spawn(async {
            println!("Hey!");
        });

        std::thread::sleep(Duration::from_millis(1));
    }
}
