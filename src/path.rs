use crate::PurePath;
use std::{
    fs::{Metadata, ReadDir},
    io::Result,
};

/// A path trait.
pub trait Path: PurePath {
    /// Returns the canonical path.
    fn canonicalize(&self) -> Result<Self>;

    // /// Returns whether the path exists.
    // fn exists(&self) -> bool;

    /// Tries to check whether the path exists.
    fn try_exists(&self) -> Result<bool>;

    // /// Returns whether the path is a directory.
    // fn is_dir(&self) -> bool;

    // /// Returns whether the path is a file.
    // fn is_file(&self) -> bool;

    // /// Returns whether the path is a symlink.
    // fn is_symlink(&self) -> bool;

    /// Returns the metadata.
    fn metadata(&self) -> Result<Metadata>;

    /// Returns the directory entries.
    fn read_dir(&self) -> Result<ReadDir>;

    /// Reads the symlink.
    fn read_link(&self) -> Result<Self>;

    // /// Reads the file.
    // fn read_file(&self) -> Vec<u8>;

    /// Returns the symlink metadata.
    fn symlink_metadata(&self) -> Result<Metadata>;

    // /// Writes the file.
    // fn write_file(&self, data: &[u8]);
}
