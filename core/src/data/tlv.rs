use crate::data::protocol::ProtocolInfo;
use bytes::buf::UninitSlice;
use bytes::{BufMut, BytesMut};

pub struct Writer {
    buf: BytesMut,
}

impl Writer {
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: BytesMut::with_capacity(cap),
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.put_u16(bytes.len() as u16);
        self.put_slice(bytes);
    }

    #[inline]
    pub fn write<B: AsRef<[u8]>>(&mut self, b: B) {
        self.write_bytes(b.as_ref())
    }
}

unsafe impl BufMut for Writer {
    fn remaining_mut(&self) -> usize {
        self.buf.remaining_mut()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.buf.advance_mut(cnt)
    }

    fn chunk_mut(&mut self) -> &mut UninitSlice {
        self.buf.chunk_mut()
    }
}

pub fn tlv16(device: &ProtocolInfo, guid: &[u8]) {
    let mut w = Writer::new();

    w.put_u32(7);
    w.put_u32(device.appid);
    w.put_u32(device.subid);
    w.put_slice(guid);
    w.write(device.id);
    w.write(device.version);
    w.write(device.sign);
}
