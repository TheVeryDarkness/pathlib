use crate::{pure::PathParser, PurePath};
use std::ops::Add;

/// A path parser for Posix systems.
pub struct PosixParser;

impl PathParser for PosixParser {
    const PRIMARY_COMPONENT_SEPARATOR: char = '/';
    const SECONDARY_COMPONENT_SEPARATOR: Option<char> = None;
    const EXTENSION_SEPARATOR: char = '.';
    const DRIVE_SEPARATOR: Option<char> = None;
    const ESCAPE_CHAR: char = '\\';
}

/// A pure path for Posix systems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PurePosixPath {
    path: String,
}

impl PurePath for PurePosixPath {
    type Parser = PosixParser;
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

impl Add for PurePosixPath {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        <Self as PurePath>::join(&self, &rhs)
    }
}
