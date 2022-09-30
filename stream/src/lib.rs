pub trait AsyncStream {

}

#[cfg(feature = "futures")]
impl<T: futures_io::AsyncRead + futures_io::AsyncWrite> AsyncStream for T {

}

#[cfg(feature = "tokio")]
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite> AsyncStream for T {

}