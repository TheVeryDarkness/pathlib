use crate::{pure::ParsablePath, PurePath};
use core::ops::Div;

/// A pure path for Windows systems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PureWindowsPath {
    path: String,
}

impl ParsablePath for PureWindowsPath {
    const PRIMARY_COMPONENT_SEPARATOR: char = '\\';
    const SECONDARY_COMPONENT_SEPARATOR: Option<char> = Some('/');
    const EXTENSION_SEPARATOR: char = '.';
    const DRIVE_SEPARATOR: Option<char> = Some(':');

    fn as_string_mut(&mut self) -> &mut String {
        &mut self.path
    }
}

impl From<String> for PureWindowsPath {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl<'a> From<&'a str> for PureWindowsPath {
    fn from(path: &'a str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl AsRef<str> for PureWindowsPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

impl Div for PureWindowsPath {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        <Self as PurePath>::join(&self, &rhs)
    }
}
