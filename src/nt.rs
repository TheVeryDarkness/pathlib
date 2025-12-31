use core::ops::Div;

use crate::pure::ParsablePath;
use crate::{PurePath, String, ToOwned};

/// A path for Windows systems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WindowsPath {
    path: String,
}

impl ParsablePath for WindowsPath {
    const PRIMARY_COMPONENT_SEPARATOR: char = '\\';
    const SECONDARY_COMPONENT_SEPARATOR: Option<char> = Some('/');
    const EXTENSION_SEPARATOR: char = '.';
    const DRIVE_SEPARATOR: Option<char> = Some(':');
    const CURRENT_DIR: &'static str = ".";
    const PARENT_DIR: &'static str = "..";

    fn as_string_mut(&mut self) -> &mut String {
        &mut self.path
    }
}

impl From<String> for WindowsPath {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl<'a> From<&'a str> for WindowsPath {
    fn from(path: &'a str) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}

impl AsRef<str> for WindowsPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

impl Div for WindowsPath {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        <Self as PurePath>::join_in_place(&mut self, &rhs);
        self
    }
}

impl Div for &WindowsPath {
    type Output = WindowsPath;

    fn div(self, rhs: Self) -> Self::Output {
        <WindowsPath as PurePath>::join(self, rhs)
    }
}
