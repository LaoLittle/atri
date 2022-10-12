use std::future::poll_fn;
use std::io;
use std::task::{Context, Poll};

pub trait Connector
where
    Self: Sized + Unpin,
{
    fn aread(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>>;

    fn awrite(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>>;
}

#[cfg(feature = "net-tokio")]
mod net_tokio {
    use std::io;
    use std::pin::Pin;
    use std::task::{Context, Poll, ready};
    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use crate::network::connector::Connector;

    impl Connector for tokio::net::TcpStream {
        fn aread(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
            let pin = Pin::new(self);
            let mut buffer = ReadBuf::new(buf);
            ready!(pin.poll_read(cx, &mut buffer))?;
            Poll::Ready(Ok(buf.len()))
        }

        fn awrite(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
            let pin = Pin::new(self);
            pin.poll_write(cx, buf)
        }
    }
}

#[cfg(feature = "io-std")]
mod io_std {
    use std::task::{Context, Poll};
    use std::io;
    use std::io::{Read, Write};
    use crate::network::connector::Connector;

    impl Connector for std::net::TcpStream {
        fn aread(&mut self, _: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
            Poll::Ready(self.read(buf))
        }

        fn awrite(&mut self, _: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
            Poll::Ready(self.write(buf))
        }
    }
}

pub async fn send<C: Connector>(connector: &mut C, buf: &[u8]) -> io::Result<usize> {
    poll_fn(|cx| {
        connector.awrite(cx, buf)
    }).await
}

pub async fn recv<C: Connector>(connector: &mut C, buf: &mut [u8]) -> io::Result<usize> {
    poll_fn(|cx| {
        connector.aread(cx, buf)
    }).await
}