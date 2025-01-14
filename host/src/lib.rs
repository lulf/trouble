//! Trouble is a Bluetooth Low Energy (BLE) Host implementation that communicates
//! with a controller over any transport implementing the traits from the `bt-hci`
//! crate.
//!
//! Trouble can run on embedded devices (`no_std`) and be configured to consume
//! as little resources are needed depending on your required configuration.
#![no_std]
#![allow(async_fn_in_trait)]
#![allow(dead_code)]
#![allow(unused_variables)]

use advertise::AdvertisementDataError;
pub use bt_hci::param::{AddrKind, BdAddr, LeConnRole as Role};
use bt_hci::FromHciBytesError;

mod fmt;

mod att;
mod channel_manager;
mod codec;
mod connection_manager;
mod cursor;
mod packet_pool;
mod pdu;
pub mod types;

pub use packet_pool::Qos as PacketQos;

pub mod adapter;
pub mod advertise;
pub mod connection;
pub mod l2cap;
pub mod scan;

#[cfg(feature = "gatt")]
pub mod attribute;
#[cfg(feature = "gatt")]
mod attribute_server;
#[cfg(feature = "gatt")]
pub mod gatt;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Address {
    pub kind: AddrKind,
    pub addr: BdAddr,
}

impl Address {
    pub fn random(val: [u8; 6]) -> Self {
        Self {
            kind: AddrKind::RANDOM,
            addr: BdAddr::new(val),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdapterError<E> {
    Controller(E),
    Adapter(Error),
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    HciEncode(bt_hci::param::Error),
    HciDecode(FromHciBytesError),
    InsufficientSpace,
    InvalidValue,
    Advertisement(AdvertisementDataError),
    InvalidChannelId,
    NoChannelAvailable,
    NotFound,
    InvalidState,
    OutOfMemory,
    NotSupported,
    ChannelClosed,
    Timeout,
    Busy,
    NoPermits,
    Disconnected,
    Other,
}

impl<E> From<Error> for AdapterError<E> {
    fn from(value: Error) -> Self {
        Self::Adapter(value)
    }
}

impl From<FromHciBytesError> for Error {
    fn from(error: FromHciBytesError) -> Self {
        Self::HciDecode(error)
    }
}

impl<E> From<bt_hci::controller::CmdError<E>> for AdapterError<E> {
    fn from(error: bt_hci::controller::CmdError<E>) -> Self {
        match error {
            bt_hci::controller::CmdError::Hci(p) => Self::Adapter(Error::HciEncode(p)),
            bt_hci::controller::CmdError::Io(p) => Self::Controller(p),
        }
    }
}

impl<E> From<bt_hci::param::Error> for AdapterError<E> {
    fn from(error: bt_hci::param::Error) -> Self {
        Self::Adapter(Error::HciEncode(error))
    }
}

impl From<codec::Error> for Error {
    fn from(error: codec::Error) -> Self {
        match error {
            codec::Error::InsufficientSpace => Error::InsufficientSpace,
            codec::Error::InvalidValue => Error::InvalidValue,
        }
    }
}

impl<E> From<codec::Error> for AdapterError<E> {
    fn from(error: codec::Error) -> Self {
        match error {
            codec::Error::InsufficientSpace => AdapterError::Adapter(Error::InsufficientSpace),
            codec::Error::InvalidValue => AdapterError::Adapter(Error::InvalidValue),
        }
    }
}
