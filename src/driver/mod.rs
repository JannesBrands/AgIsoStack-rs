// Copyright 2023 Raven Industries inc.

//! CAN Driver layer
//!
//! This module defines:
//! 1. An abstract `Driver` trait for different CAN drivers to implement
//! 2. `Frame`, `Pgn`, `Address`, et al types

mod address;
mod can_id;
mod driver;
mod frame;
mod pgn;

pub use address::Address;
pub use can_id::{CanId, Priority, Type};
pub use driver::{Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError};
pub use frame::Frame;
pub use pgn::Pgn;
