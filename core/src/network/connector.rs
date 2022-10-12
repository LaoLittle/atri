use std::future::poll_fn;
use std::io;
use std::task::{Context, Poll};

pub trait Connector
where
    Self: Sized + Unpin,
{
    fn poll_recv(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>>;

    fn poll_send(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>>;
}

#[cfg(feature = "net-tokio")]
mod net_tokio {
    use crate::network::connector::Connector;
    use std::io;
    use std::pin::Pin;
    use std::task::{ready, Context, Poll};
    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    impl Connector for tokio::net::TcpStream {
        fn poll_recv(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
            let pin = Pin::new(self);
            let mut buffer = ReadBuf::new(buf);
            ready!(pin.poll_read(cx, &mut buffer))?;
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_send(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
            let pin = Pin::new(self);
            pin.poll_write(cx, buf)
        }
    }
}

#[cfg(feature = "net-std")]
mod io_std {
    use crate::network::connector::Connector;
    use std::io;
    use std::io::{Read, Write};
    use std::task::{Context, Poll};

    impl Connector for std::net::TcpStream {
        fn poll_recv(&mut self, _: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
            Poll::Ready(self.read(buf))
        }

        fn poll_send(&mut self, _: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
            Poll::Ready(self.write(buf))
        }
    }
}

#[cfg(feature = "smol")]
mod smol {
    use crate::network::connector::Connector;
    use smol::io::{AsyncRead, AsyncWrite};
    use std::pin::Pin;
    use std::task::{Context, Poll};

    impl Connector for smol::net::TcpStream {
        fn poll_recv(
            &mut self,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            let pin = Pin::new(self);
            pin.poll_read(cx, buf)
        }

        fn poll_send(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
            let pin = Pin::new(self);
            pin.poll_write(cx, buf)
        }
    }
}

pub async fn send<C: Connector>(connector: &mut C, buf: &[u8]) -> io::Result<usize> {
    poll_fn(|cx| connector.poll_send(cx, buf)).await
}

pub async fn send_all<C: Connector>(connector: &mut C, buf: &[u8]) -> io::Result<()> {
    let size = send(connector, buf).await?;
    if size == buf.len() {
        Ok(())
    } else {
        Err(io::ErrorKind::WriteZero.into())
    }
}

pub async fn recv<C: Connector>(connector: &mut C, buf: &mut [u8]) -> io::Result<usize> {
    poll_fn(|cx| connector.poll_recv(cx, buf)).await
}

pub async fn recv_all<C: Connector>(connector: &mut C, buf: &mut [u8]) -> io::Result<()> {
    let size = recv(connector, buf).await?;
    if size == buf.len() {
        Ok(())
    } else {
        Err(io::ErrorKind::UnexpectedEof.into())
    }
}
