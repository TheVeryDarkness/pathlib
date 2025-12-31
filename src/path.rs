use std::fs::{Metadata, ReadDir};
use std::io::Result;
use std::path::Path as StdPath;

use crate::PurePath;

/// A path trait.
pub trait Path: PurePath {
    /// Returns the canonical path.
    fn canonicalize(&self) -> Result<Self>;

    // /// Returns whether the path exists.
    // fn exists(&self) -> bool;

    /// Tries to check whether the path exists.
    fn try_exists(&self) -> Result<bool>;

    /// Returns whether the path is a directory.
    fn is_dir(&self) -> bool {
        self.metadata()
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    /// Returns whether the path is a file.
    fn is_file(&self) -> bool {
        self.metadata()
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    /// Returns whether the path is a symlink.
    fn is_symlink(&self) -> bool {
        self.symlink_metadata()
            .map(|metadata| metadata.file_type().is_symlink())
            .unwrap_or(false)
    }

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

impl<P: PurePath + AsRef<str> + for<'a> From<&'a str>> Path for P
where
    Self: Sized,
{
    fn canonicalize(&self) -> Result<Self> {
        let std_path = StdPath::new(self.as_ref());
        let canonical_path = std_path.canonicalize()?;
        Ok(Self::from(canonical_path.to_str().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to convert path to string",
            )
        })?))
    }

    fn try_exists(&self) -> Result<bool> {
        let std_path = StdPath::new(self.as_ref());
        Ok(std_path.exists())
    }

    fn metadata(&self) -> Result<Metadata> {
        let std_path = StdPath::new(self.as_ref());
        std_path.metadata()
    }

    fn read_dir(&self) -> Result<ReadDir> {
        let std_path = StdPath::new(self.as_ref());
        std_path.read_dir()
    }

    fn read_link(&self) -> Result<Self> {
        let std_path = StdPath::new(self.as_ref());
        let target_path = std_path.read_link()?;
        Ok(Self::from(target_path.to_str().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to convert path to string",
            )
        })?))
    }

    fn symlink_metadata(&self) -> Result<Metadata> {
        let std_path = StdPath::new(self.as_ref());
        std_path.symlink_metadata()
    }
}
