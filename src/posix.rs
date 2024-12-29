use crate::{pure::ParsablePath, PurePath};
use core::ops::Div;

/// A path for Posix systems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosixPath {
    path: String,
}

impl ParsablePath for PosixPath {
    const PRIMARY_COMPONENT_SEPARATOR: char = '/';
    const SECONDARY_COMPONENT_SEPARATOR: Option<char> = None;
    const EXTENSION_SEPARATOR: char = '.';
    const DRIVE_SEPARATOR: Option<char> = None;

    fn as_string_mut(&mut self) -> &mut String {
        &mut self.path
    }
}

impl From<String> for PosixPath {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl<'a> From<&'a str> for PosixPath {
    fn from(path: &'a str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl AsRef<str> for PosixPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

impl Div for PosixPath {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        <Self as PurePath>::join(&self, &rhs)
    }
}
