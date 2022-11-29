#[cfg(feature = "parking_lot")]
mod parking_lot {
    use parking_lot::lock_api::MutexGuard;
    use parking_lot::{Mutex as PLMutex, RawMutex};
    use std::fmt;

    pub struct Mutex<T: ?Sized> {
        inner: PLMutex<T>,
    }

    impl<T> Mutex<T> {
        #[inline]
        pub fn new(t: T) -> Self {
            Self {
                inner: PLMutex::new(t),
            }
        }

        #[inline]
        pub const fn const_new(t: T) -> Self {
            Self {
                inner: parking_lot::const_mutex(t),
            }
        }

        #[inline]
        pub fn lock(&self) -> MutexGuard<RawMutex, T> {
            self.inner.lock()
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Mutex<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.inner.fmt(f)
        }
    }
}
#[cfg(feature = "parking_lot")]
pub use self::parking_lot::*;

#[cfg(not(feature = "parking_lot"))]
mod standard {
    use std::fmt;
    use std::sync::Mutex as StdMutex;
    use std::sync::MutexGuard;

    pub struct Mutex<T: ?Sized> {
        inner: StdMutex<T>,
    }

    impl<T> Mutex<T> {
        #[inline]
        pub const fn new(t: T) -> Self {
            Self::const_new(t)
        }

        #[inline]
        pub const fn const_new(t: T) -> Self {
            Self {
                inner: StdMutex::new(t),
            }
        }

        pub fn lock(&self) -> MutexGuard<T> {
            match self.inner.lock() {
                Ok(g) => g,
                Err(e) => e.into_inner(),
            }
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Mutex<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.inner.fmt(f)
        }
    }
}

#[cfg(not(feature = "parking_lot"))]
pub use self::standard::*;

#[cfg(test)]
mod tests {
    use super::Mutex;
    use std::ops::Deref;

    #[test]
    fn lock() {
        let m = Mutex::const_new(1);
        *m.lock() = 12;

        assert_eq!(12, *m.lock().deref());
    }
}
