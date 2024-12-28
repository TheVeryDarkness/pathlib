use crate::PurePath;

/// A path trait.
pub trait Path: PurePath {
    /// Returns whether the path exists.
    fn exists(&self) -> bool;
    /// Returns whether the path is a directory.
    fn is_dir(&self) -> bool;
    /// Returns whether the path is a file.
    fn is_file(&self) -> bool;
    /// Returns the directory entries.
    fn read_dir(&self) -> Vec<Self>;
    /// Reads the file.
    fn read_file(&self) -> Vec<u8>;
    /// Writes the file.
    fn write_file(&self, data: &[u8]);
}
