#[cfg(feature = "parking_lot")]
mod parking_lot {
    pub type Mutex<T> = parking_lot::Mutex<T>;

    pub type RwLock<T> = parking_lot::RwLock<T>;
}
#[cfg(feature = "parking_lot")]
pub use self::parking_lot::*;

#[cfg(not(feature = "parking_lot"))]
mod standard {
    pub type Mutex<T> = std::sync::Mutex<T>;

    pub type RwLock<T> = std::sync::RwLock<T>;
}

#[cfg(not(feature = "parking_lot"))]
pub use self::standard::*;
