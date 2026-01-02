use core::ops::Div;

use crate::pure::ParsablePath;
use crate::{Component, PosixPath, PurePath, String, ToOwned, WindowsPath};

/// A path for Posix systems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnifiedPath {
    path: String,
}

impl ParsablePath for UnifiedPath {
    const PRIMARY_COMPONENT_SEPARATOR: char = '/';
    const SECONDARY_COMPONENT_SEPARATOR: Option<char> = None;
    const EXTENSION_SEPARATOR: char = '.';
    const DRIVE_SEPARATOR: Option<char> = Some(':');
    const CURRENT_DIR: &'static str = ".";
    const PARENT_DIR: &'static str = "..";

    fn as_string_mut(&mut self) -> &mut String {
        &mut self.path
    }
}

impl From<String> for UnifiedPath {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl<'a> From<&'a str> for UnifiedPath {
    fn from(path: &'a str) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}

impl<'a> FromIterator<Component<'a>> for UnifiedPath {
    fn from_iter<T: IntoIterator<Item = Component<'a>>>(iter: T) -> Self {
        const COMPONENT_SEPARATOR: char = UnifiedPath::PRIMARY_COMPONENT_SEPARATOR;
        const DRIVE_SEPARATOR: char = UnifiedPath::DRIVE_SEPARATOR.unwrap();
        const CURRENT_DIR: &str = UnifiedPath::CURRENT_DIR;
        const PARENT_DIR: &str = UnifiedPath::PARENT_DIR;
        let mut path = String::new();
        for component in iter {
            match component {
                Component::Prefix(s) => {
                    path.push_str(s);
                    path.push(DRIVE_SEPARATOR);
                }
                Component::Root => {
                    path.push(COMPONENT_SEPARATOR);
                }
                Component::CurDir => {
                    if !path.ends_with(COMPONENT_SEPARATOR) && !path.is_empty() {
                        path.push(COMPONENT_SEPARATOR);
                    }
                    path.push_str(CURRENT_DIR);
                }
                Component::ParentDir => {
                    if !path.ends_with(COMPONENT_SEPARATOR) && !path.is_empty() {
                        path.push(COMPONENT_SEPARATOR);
                    }
                    path.push_str(PARENT_DIR);
                }
                Component::Normal(s) => {
                    if !path.ends_with(COMPONENT_SEPARATOR) && !path.is_empty() {
                        path.push(COMPONENT_SEPARATOR);
                    }
                    path.push_str(s);
                }
            }
        }
        Self { path }
    }
}

impl From<PosixPath> for UnifiedPath {
    fn from(path: PosixPath) -> Self {
        path.components().collect()
    }
}

impl From<WindowsPath> for UnifiedPath {
    fn from(path: WindowsPath) -> Self {
        path.components().collect()
    }
}

impl AsRef<str> for UnifiedPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

impl Div for UnifiedPath {
    type Output = Self;

    fn div(mut self, rhs: Self) -> <Self as Div>::Output {
        <Self as PurePath>::join_in_place(&mut self, &rhs);
        self
    }
}

impl Div for &UnifiedPath {
    type Output = UnifiedPath;

    fn div(self, rhs: Self) -> <Self as Div>::Output {
        <UnifiedPath as PurePath>::join(self, rhs)
    }
}
