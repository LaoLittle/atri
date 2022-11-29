use bytes::{BufMut, Bytes, BytesMut};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

static DELTAS: [u32; 16] = [
    0x9e3779b9, 0x3c6ef372, 0xdaa66d2b, 0x78dde6e4, 0x1715609d, 0xb54cda56, 0x5384540f, 0xf1bbcdc8,
    0x8ff34781, 0x2e2ac13a, 0xcc623af3, 0x6a99b4ac, 0x08d12e65, 0xa708a81e, 0x454021d7, 0xe3779b90,
]; // cache

const FIRST_RNG: Range<usize> = 0..4;
const SECOND_RNG: Range<usize> = 4..8;

pub struct Tea {
    key: [u32; 4],
}

impl Tea {
    #[inline]
    pub fn from_key(key: [u32; 4]) -> Self {
        Self { key }
    }

    pub fn encrypt(&self, data: &[u8]) -> Bytes {
        let n = 6usize.wrapping_sub(data.len());
        let n = (n % 8) + 2;

        let mut bytes = BytesMut::new();
        bytes.put_u8(((n - 2) | 0xF8) as u8);
        bytes.put_bytes(0, n);
        bytes.extend_from_slice(data);
        bytes.put_bytes(0, 7);

        let k0 = self.key[0];
        let k1 = self.key[1];
        let k2 = self.key[2];
        let k3 = self.key[3];

        let mut r1 = 0;
        let mut r2 = 0;
        let mut t1 = 0;
        let mut t2 = 0;

        for chunk in bytes.chunks_exact_mut(8) {
            let mut buf = [0; 4];
            buf.copy_from_slice(&chunk[FIRST_RNG]);
            let a1 = u32::from_be_bytes(buf);
            buf.copy_from_slice(&chunk[SECOND_RNG]);
            let a2 = u32::from_be_bytes(buf);

            let b1 = a1 ^ r1;
            let b2 = a2 ^ r2;

            let (x, y) = _encrypt(b1, b2, k0, k1, k2, k3);

            r1 = x ^ t1;
            r2 = y ^ t2;
            t1 = b1;
            t2 = b2;

            chunk[FIRST_RNG].copy_from_slice(&r1.to_be_bytes());
            chunk[SECOND_RNG].copy_from_slice(&r2.to_be_bytes());
        }

        bytes.freeze()
    }

    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Bytes, DecryptError> {
        let len = encrypted.len();
        if len % 8 != 0 {
            return Err(DecryptError);
        }

        let k0 = self.key[0];
        let k1 = self.key[1];
        let k2 = self.key[2];
        let k3 = self.key[3];

        let mut r1 = 0;
        let mut r2 = 0;
        let mut t1 = 0;
        let mut t2 = 0;

        let (mut x, mut y) = (0, 0);

        let mut dec = BytesMut::from(encrypted);

        for chunk in dec.chunks_exact_mut(8) {
            let mut buf = [0; 4];
            buf.copy_from_slice(&chunk[FIRST_RNG]);
            let a1 = u32::from_be_bytes(buf);
            buf.copy_from_slice(&chunk[SECOND_RNG]);
            let a2 = u32::from_be_bytes(buf);

            let b1 = a1 ^ x;
            let b2 = a2 ^ y;

            (x, y) = _decrypt(b1, b2, k0, k1, k2, k3);

            r1 = x ^ t1;
            r2 = y ^ t2;
            t1 = a1;
            t2 = a2;

            chunk[FIRST_RNG].copy_from_slice(&r1.to_be_bytes());
            chunk[SECOND_RNG].copy_from_slice(&r2.to_be_bytes());
        }

        let start = (dec[0] as usize & 0x07) + 3;

        if dec.len() < 7 + start {
            return Err(DecryptError);
        }

        let mut dec = dec.freeze().split_to(len - 7);

        drop(dec.split_to(start));
        Ok(dec)
    }

    #[inline]
    pub fn key(&self) -> &[u32; 4] {
        &self.key
    }
}

const fn calc(x: u32, v0: u32, v1: u32, delta: u32) -> u32 {
    ((v0.wrapping_add(x << 4)) ^ (x.wrapping_add(delta))) ^ (v1.wrapping_add(x / 32))
}

fn _encrypt(mut x: u32, mut y: u32, k0: u32, k1: u32, k2: u32, k3: u32) -> (u32, u32) {
    for delta in DELTAS {
        let a = calc(y, k0, k1, delta);
        x = x.wrapping_add(a);

        let b = calc(x, k2, k3, delta);
        y = y.wrapping_add(b);
    }

    return (x, y);
}

fn _decrypt(mut x: u32, mut y: u32, k0: u32, k1: u32, k2: u32, k3: u32) -> (u32, u32) {
    for delta in DELTAS.into_iter().rev() {
        let a = calc(x, k2, k3, delta);
        y = y.wrapping_sub(a);

        let b = calc(y, k0, k1, delta);
        x = x.wrapping_sub(b);
    }

    return (x, y);
}

#[derive(Debug)]
pub struct DecryptError;

impl Display for DecryptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid encrypted data")
    }
}

impl Error for DecryptError {}

#[cfg(test)]
mod tests {
    use crate::crypto::tea::Tea;
    use std::time::SystemTime;

    const TEST_KEY: [u32; 4] = [114, 514, 1919, 810];
    static TEST_TEXT: &str = "1145141919810";

    #[test]
    fn crypto() {
        let now = SystemTime::now();
        let tea = Tea { key: TEST_KEY };

        let b = tea.encrypt(TEST_TEXT.as_bytes());
        let d = tea.decrypt(&b);

        println!("{:?}", now.elapsed());

        assert!(d.is_ok());
        assert_eq!(Ok(TEST_TEXT), std::str::from_utf8(&d.unwrap()));
    }
}
