#![forbid(rust_2021_compatibility, rust_2018_idioms, future_incompatible)]
#![deny(
    unused_imports,
    unused_qualifications,
    unused_results,
    unused_comparisons,
    unconditional_panic,
    unconditional_recursion,
    unreachable_pub
)]
#![forbid(missing_docs, rustdoc::all)]
#![forbid(
    clippy::correctness,
    clippy::complexity,
    clippy::suspicious,
    clippy::perf,
    clippy::style,
    clippy::cargo,
    clippy::should_panic_without_expect,
    clippy::incompatible_msrv,
    clippy::expect_used,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason
)]
#![cfg_attr(not(feature = "std"), no_std)]
//! A simple path manipulation library.

#[cfg(not(feature = "std"))]
extern crate alloc;

mod comp;
mod nt;
#[cfg(feature = "std")]
mod path;
mod posix;
mod pure;
mod unified;

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};
#[cfg(feature = "std")]
use std::{borrow::ToOwned, string::String, vec::Vec};

pub use comp::{Component, Components};
pub use nt::WindowsPath;
#[cfg(feature = "std")]
pub use path::Path;
pub use posix::PosixPath;
pub use pure::{ParsablePath, PurePath};
pub use unified::UnifiedPath;
