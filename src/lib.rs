//! # `AcceptXMR`: Accept Monero in Your Application
//!
//! This library aims to provide a simple, reliable, and efficient means to track monero payments.
//!
//! To track payments, the [`PaymentGateway`] generates subaddresses using your private view key and
//! primary address. It then watches for monero sent to that subaddress using a monero daemon of
//! your choosing, your private view key and your primary address.
//!
//! Use this library at your own risk, it is young and unproven.
//!
//! ## Key Features
//! * View pair only, no hot wallet.
//! * Subaddress based.
//! * Pending invoices stored persistently, enabling recovery from power loss.
//! * Number of confirmations is configurable per-invoice.
//! * Ignores transactions with non-zero timelocks.
//!
//! ## Security
//!
//! `AcceptXMR` is non-custodial, and does not require a hot wallet. However, it does require your
//! private view key and primary address for scanning outputs. If keeping these private is important
//! to you, please take appropriate precautions to secure the platform you run your application on
//! _and keep your private view key out of your git repository!_.
//!
//! Also note that anonymity networks like TOR are not currently supported for RPC calls. This
//! means that your network traffic will reveal that you are interacting with the monero network.
//!
//! ## Reliability
//!
//! This library strives for reliability, but that attempt may not be successful. `AcceptXMR` is
//! young and unproven, and relies on several crates which are undergoing rapid changes themselves
//! (for example, the database used ([Sled](sled)) is still in beta).
//!
//! That said, this payment gateway should survive unexpected power loss thanks to pending invoices
//! being flushed to disk each time new blocks/transactions are scanned. A best effort is made to
//! keep the scanning thread free any of potential panics, and RPC calls in the scanning thread are
//! logged on failure and repeated next scan. In the event that an error does occur, the liberal use
//! of logging within this library will hopefully facilitate a speedy diagnosis an correction.
//!
//! Use this library at your own risk.
//!
//! ## Performance
//!
//! It is strongly recommended that you host your own monero daemon on the same local network.
//! Network and daemon slowness are the primary cause of high invoice update latency in the majority
//! of use cases.
//!
//! To reduce the average latency before receiving invoice updates, you may also consider lowering
//! the [`PaymentGateway`]'s `scan_interval` below the default of 1 second:
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use tempfile::Builder;
//! use acceptxmr::PaymentGateway;
//! use std::time::Duration;
//!
//! # let temp_dir = Builder::new()
//! #   .prefix("temp_db_")
//! #   .rand_bytes(16)
//! #   .tempdir()?;
//!
//! let private_view_key = "ad2093a5705b9f33e6f0f0c1bc1f5f639c756cdfc168c8f2ac6127ccbdab3a03";
//! let primary_address = "4613YiHLM6JMH4zejMB2zJY5TwQCxL8p65ufw8kBP5yxX9itmuGLqp1dS4tkVoTxjyH3aYhYNrtGHbQzJQP5bFus3KHVdmf";
//!
//! let payment_gateway = PaymentGateway::builder(private_view_key.to_string(), primary_address.to_string())
//!     .scan_interval(Duration::from_millis(100)) // Scan for invoice updates every 100 ms.
//! #   .db_path(temp_dir.path().to_str().expect("Failed to get temporary directory path").to_string())
//!     .build()?;
//! #   Ok(())
//! # }
//! ```
//!
//! Please note that `scan_interval` is the minimum time between scanning for updates. If your
//! daemon's response time is already greater than your `scan_interval`, or if your CPU is unable to
//! scan new transactions fast enough, reducing your `scan_interval` will do nothing.

#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::module_name_repetitions)]

mod caching;
mod invoice;
mod invoices_db;
mod payment_gateway;
mod rpc;
mod scanner;
mod subscriber;

use std::error::Error;
use std::fmt;

pub use invoice::{Invoice, InvoiceId, SubIndex};
use invoices_db::InvoiceStorageError;
pub use payment_gateway::{PaymentGateway, PaymentGatewayBuilder};
use rpc::RpcError;
pub use subscriber::{Subscriber, SubscriberError};

/// Library's custom error type.
#[derive(Debug)]
pub enum AcceptXmrError {
    /// An error originating from a daemon RPC call.
    Rpc(RpcError),
    /// An error storing/retrieving [`Invoice`](crate::Invoice)s.
    InvoiceStorage(InvoiceStorageError),
    /// [`Subscriber`](crate::Subscriber) failed to retrieve update.
    Subscriber(SubscriberError),
    /// Failure to unblind the amount of an owned output.
    Unblind(SubIndex),
    /// Failure to parse private view key.
    Parse {
        /// Type to parse.
        datatype: &'static str,
        /// Input to parse.
        input: String,
        /// Error encountered.
        error: String,
    },
    /// Failure to check if output is owned.
    OwnedOutputCheck(monero::blockdata::transaction::Error),
    /// Failed to start scanning thread.
    ScanningThread(std::io::Error),
}

impl From<RpcError> for AcceptXmrError {
    fn from(e: RpcError) -> Self {
        Self::Rpc(e)
    }
}

impl From<InvoiceStorageError> for AcceptXmrError {
    fn from(e: InvoiceStorageError) -> Self {
        Self::InvoiceStorage(e)
    }
}

impl From<SubscriberError> for AcceptXmrError {
    fn from(e: SubscriberError) -> Self {
        Self::Subscriber(e)
    }
}

impl From<monero::blockdata::transaction::Error> for AcceptXmrError {
    fn from(e: monero::blockdata::transaction::Error) -> Self {
        Self::OwnedOutputCheck(e)
    }
}

impl From<std::io::Error> for AcceptXmrError {
    fn from(e: std::io::Error) -> Self {
        Self::ScanningThread(e)
    }
}

impl fmt::Display for AcceptXmrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcceptXmrError::Rpc(e) => {
                write!(f, "RPC error: {}", e)
            }
            AcceptXmrError::InvoiceStorage(e) => {
                write!(f, "invoice storage error: {}", e)
            }
            AcceptXmrError::Subscriber(e) => {
                write!(f, "subscriber failed to receive update: {}", e)
            }
            AcceptXmrError::Unblind(index) => write!(
                f,
                "unable to unblind amount of owned output sent to subaddress index {}",
                index
            ),
            AcceptXmrError::Parse {
                datatype,
                input,
                error,
            } => {
                write!(
                    f,
                    "failed to parse {} from \"{}\": {}",
                    datatype, input, error
                )
            }
            AcceptXmrError::OwnedOutputCheck(e) => {
                write!(f, "failed to check if output is owned: {}", e)
            }
            AcceptXmrError::ScanningThread(e) => {
                write!(f, "error starting scanning thread: {}", e)
            }
        }
    }
}

impl Error for AcceptXmrError {}
