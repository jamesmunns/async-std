//! Tools for working with processes.

pub use std::process::{ExitStatus, Output};

pub use child::Child;
pub use command::Command;

mod child;
mod command;
