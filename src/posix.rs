use crate::{pure::ParsablePath, PurePath};
use core::ops::Div;

/// A pure path for Posix systems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PurePosixPath {
    path: String,
}

impl ParsablePath for PurePosixPath {
    const PRIMARY_COMPONENT_SEPARATOR: char = '/';
    const SECONDARY_COMPONENT_SEPARATOR: Option<char> = None;
    const EXTENSION_SEPARATOR: char = '.';
    const DRIVE_SEPARATOR: Option<char> = None;

    fn as_string_mut(&mut self) -> &mut String {
        &mut self.path
    }
}

impl From<String> for PurePosixPath {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl<'a> From<&'a str> for PurePosixPath {
    fn from(path: &'a str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl AsRef<str> for PurePosixPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

impl Div for PurePosixPath {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        <Self as PurePath>::join(&self, &rhs)
    }
}
