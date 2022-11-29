use crate::data::protocol::ProtocolInfo;
use bytes::buf::UninitSlice;
use bytes::{BufMut, Bytes, BytesMut};

pub struct TlvWriter {
    buf: BytesMut,
}

impl TlvWriter {
    #[inline]
    pub fn new(cmd: u16) -> Self {
        Self::with_capacity(cmd, 0)
    }

    pub fn with_capacity(cmd: u16, cap: usize) -> Self {
        let mut buf = BytesMut::with_capacity(cap + 4);
        buf.put_u16(cmd);
        buf.put_u16(0); // payload length

        Self { buf }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.put_u16(bytes.len() as u16);
        self.put_slice(bytes);
    }

    #[inline]
    pub fn write<B: AsRef<[u8]>>(&mut self, b: B) {
        self.write_bytes(b.as_ref())
    }

    pub fn into_bytes_mut(mut self) -> BytesMut {
        let len = self.buf.len() - 4;
        self.buf[2..4].copy_from_slice(&(len as u16).to_be_bytes());
        self.buf
    }

    #[inline]
    pub fn into_bytes(mut self) -> Bytes {
        self.into_bytes_mut().freeze()
    }

    #[inline]
    pub fn complete(mut self) -> Bytes {
        self.into_bytes()
    }
}

unsafe impl BufMut for TlvWriter {
    #[inline]
    fn remaining_mut(&self) -> usize {
        self.buf.remaining_mut()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.buf.advance_mut(cnt)
    }

    #[inline]
    fn chunk_mut(&mut self) -> &mut UninitSlice {
        self.buf.chunk_mut()
    }
}

macro_rules! tlv_write {
    ($writer:expr; $($op:ident $arg:expr),* $(,)?) => {
        let w = &mut $writer;

        $(w.$op($arg);)*
    };
}

pub fn tlv16(protocol: &ProtocolInfo, guid: &[u8; 16]) -> Bytes {
    let mut w = TlvWriter::new(0x16);

    tlv_write!(
        w;
        put_u32 protocol.sso_version,
        put_u32 protocol.appid,
        put_u32 protocol.subid,
        put_slice guid,
        write protocol.id,
        write protocol.version,
        write protocol.sign,
    );

    w.into_bytes()
}

pub fn tlv1b() -> Bytes {
    let mut w = TlvWriter::new(0x1b);

    tlv_write!(
        w;
        put_u32 0,
        put_u32 0,
        put_u32 3,
        put_u32 4,
        put_u32 72,
        put_u32 2,
        put_u32 2,
        put_u16 0,
    );

    w.into_bytes()
}
