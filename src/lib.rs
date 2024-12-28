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

mod nt;
#[cfg(feature = "std")]
mod path;
mod posix;
mod pure;

pub use nt::{PureWindowsPath, WindowsParser};
#[cfg(feature = "std")]
pub use path::Path;
pub use posix::PurePosixPath;
pub use pure::{PathParser, PurePath};
