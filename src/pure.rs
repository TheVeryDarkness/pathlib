use std::ops::Add;

/// The current directory.
const CURRENT_DIR: &str = ".";
/// The parent directory.
const PARENT_DIR: &str = "..";

#[inline]
fn rsplit_once_with_delimiter<'i>(
    s: &'i str,
    delimiter: &[char],
) -> Option<(&'i str, &'i str, &'i str)> {
    let i = s.rfind(delimiter)?;
    let (a, b) = s.split_at(i);
    // FIXME: This hardcodes the assumption that the delimiter is a single byte.
    let (b, c) = b.split_at(1);
    Some((a, b, c))
}

/// A path parser.
pub trait PathParser {
    /// The primary component separator.
    ///
    /// For example, `'/'` on Posix systems and `'\\'` on Windows.
    const PRIMARY_COMPONENT_SEPARATOR: char;
    /// The secondary component separator.
    ///
    /// For example, `None` on Posix systems and `Some('/')` on Windows.
    const SECONDARY_COMPONENT_SEPARATOR: Option<char>;
    /// The component separators.
    ///
    /// For example, `&['/', '\\']` on Windows.
    const COMPONENT_SEPARATORS: &'static [char] = match Self::SECONDARY_COMPONENT_SEPARATOR {
        Some(c) => &[Self::PRIMARY_COMPONENT_SEPARATOR, c],
        None => &[Self::PRIMARY_COMPONENT_SEPARATOR],
    };
    /// The extension separator.
    const EXTENSION_SEPARATOR: char;
    /// The drive separator.
    const DRIVE_SEPARATOR: Option<char>;
    /// The escape character. Normally `\`.
    const ESCAPE_CHAR: char;

    /// Returns the parent of the given path and the last component of the path in a lexical way.
    /// That means, `..` and `.` are not resolved or even considered.
    fn split_last(path: &str) -> (Option<(&str, &str)>, &str) {
        // while let Some(p) = path.strip_suffix(Self::PRIMARY_COMPONENT_SEPARATOR) {
        //     path = p;
        // }
        let mut s = path;
        loop {
            match rsplit_once_with_delimiter(s, Self::COMPONENT_SEPARATORS) {
                Some((parent, separator, component)) => {
                    s = parent;
                    if let Some(p) = s.strip_suffix(Self::ESCAPE_CHAR) {
                        s = p;
                        continue;
                    }
                    return (Some((parent, separator)), component);
                }
                None => return (None, path),
            }
        }
    }

    /// Returns the parent of the path and the last component of the path.
    fn split_last_component(mut s: &str) -> (Option<&str>, Option<&str>) {
        loop {
            match Self::split_last(s) {
                (Some((parent, _)), CURRENT_DIR) => {
                    s = parent;
                    continue;
                }
                (None, CURRENT_DIR) => return (Some(""), None),

                (Some(("", separator)), PARENT_DIR) => return (Some(separator), None),
                (Some((parent, _)), PARENT_DIR) => return (Some(parent), None),
                (None, PARENT_DIR) => return (Some(""), None),

                (Some(("", _)), "") => return (None, None),
                (None, "") => return (None, None),

                (Some((parent, _)), "") => {
                    s = parent;
                    continue;
                }
                (Some(("", separator)), file_name) => return (Some(separator), Some(file_name)),
                (Some((parent, _)), file_name) => return (Some(parent), Some(file_name)),
                (None, file_name) => return (Some(""), Some(file_name)),
            }
        }
    }

    /// Returns the parent of the path.
    fn parent(s: &str) -> Option<&str> {
        Self::split_last_component(s).0
        // let (parent, _) = Self::split_last(s);
        // match parent {
        //     Some(("", separator)) => Some(separator),
        //     Some((parent, _)) => Some(parent),
        //     None => Some(""),
        // }
        // if let Some((parent, separator)) = parent {
        //     if parent.is_empty() {
        //         Some(separator)
        //     } else {
        //         Some(parent)
        //     }
        // } else {
        //     None
        // }
    }

    /// Returns the last component of the path, if there is one.
    fn file_name(s: &str) -> Option<&str> {
        Self::split_last_component(s).1
        // loop {
        //     let (parent, file_name) = Self::split_last(s);
        //     match file_name {
        //         CURRENT_DIR => {
        //             if let Some((parent, _)) = parent {
        //                 s = parent;
        //                 continue;
        //             } else {
        //                 return None;
        //             }
        //         }
        //         PARENT_DIR => return None,
        //         "" => {
        //             s = parent?.0;
        //             continue;
        //         }
        //         _ => return Some(file_name),
        //     }
        // }
    }

    // /// Returns the path without the extension and the extension.
    // fn split_extension(path: &str) -> (&str, Option<&str>) {
    //     let s = Self::split_last(path).1;
    //     match s.rsplit_once(Self::EXTENSION_SEPARATOR) {
    //         Some((stem, extension)) => (path, Some(&extension[1..])),
    //         None => (s, None),
    //     }
    // }

    /// Returns the driver of the path and the rest of the path.
    fn split_driver(path: &str) -> (Option<&str>, &str) {
        if let Some(c) = Self::DRIVE_SEPARATOR {
            if let Some((drive, rest)) = path.split_once(c) {
                (Some(drive), rest)
            } else {
                (None, path)
            }
        } else {
            (None, path)
        }
    }

    /// Returns whether the path is absolute.
    fn is_absolute(path: &str) -> bool {
        path.starts_with(Self::PRIMARY_COMPONENT_SEPARATOR)
    }

    /// Append component separator if not already present.
    fn as_dir(path: &mut String) {
        if !path.ends_with(Self::PRIMARY_COMPONENT_SEPARATOR) {
            path.push(Self::PRIMARY_COMPONENT_SEPARATOR);
        }
    }
}

/// A pure path.
pub trait PurePath:
    Clone + Sized + From<String> + for<'a> From<&'a str> + Add<Output = Self> + AsRef<str>
{
    /// The parser type for this path.
    type Parser: PathParser;

    /// Returns a mutable reference to the path as a string.
    fn as_string_mut(&mut self) -> &mut String;

    /// Returns the parent of the path.
    fn parent(&self) -> Option<Self> {
        Self::Parser::parent(self.as_ref()).map(Self::from)
    }

    /// Returns the last component of the path, if there is one.
    fn file_name(&self) -> Option<&str> {
        Self::Parser::file_name(self.as_ref())
    }

    // /// Replace the extension of the path with the given extension.
    // fn with_extension(&self, ext: &str) -> Self {
    //     let (path, _) = Self::Parser::split_extension(self.as_ref());
    //     let mut joined = path.to_string();
    //     joined.push(Self::Parser::EXTENSION_SEPARATOR);
    //     joined.push_str(ext);
    //     Self::from(joined)
    // }

    // /// Returns the extension of the path.
    // fn extension(&self) -> Option<&str> {
    //     let (_, ext) = Self::Parser::split_extension(self.as_ref());
    //     ext
    // }

    /// Joins the given path in place.
    fn join_in_place(&mut self, path: &Self) {
        if path.is_absolute() {
            *self = path.clone();
            return;
        }
        let joined = self.as_string_mut();
        Self::Parser::as_dir(joined);
        joined.push_str(path.as_ref());
    }

    /// Joins the given path.
    fn join(&self, path: &Self) -> Self {
        if path.is_absolute() {
            return path.clone();
        }
        let mut joined = self.as_ref().to_string();
        Self::Parser::as_dir(&mut joined);
        joined.push_str(path.as_ref());
        Self::from(joined)
    }

    /// Returns whether the path is absolute.
    fn is_absolute(&self) -> bool {
        Self::Parser::is_absolute(self.as_ref())
    }
    // fn strip_extension(&self) -> Self;
    // fn strip_prefix(&self, prefix: &str) -> Option<Self>;
    // fn strip_suffix(&self, suffix: &str) -> Option<Self>;
    // fn starts_with(&self, path: &str) -> bool;
    // fn ends_with(&self, path: &str) -> bool;

    // fn components(&self) -> impl Iterator<Item = &str>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{posix::PurePosixPath, PureWindowsPath};
    use std::{
        ffi::OsStr,
        path::{Path, PathBuf},
    };

    const PARENT_AND_FILE_NAME: &[(&str, Option<&str>, Option<&str>)] = &[
        ("/foo/bar", Some("/foo"), Some("bar")),
        ("/foo", Some("/"), Some("foo")),
        ("/", None, None),
        ("foo/bar", Some("foo"), Some("bar")),
        ("foo", Some(""), Some("foo")),
        ("", None, None),
        ("/usr/bin/", Some("/usr"), Some("bin")),
        ("tmp/foo.txt", Some("tmp"), Some("foo.txt")),
        ("foo.txt/.", Some(""), Some("foo.txt")),
        ("foo.txt/.//", Some(""), Some("foo.txt")),
        ("foo.txt/..", Some("foo.txt"), None),
        ("/", None, None),
        ("//", None, None),
        ("/./", None, None),
        (".", Some(""), None),
        ("..", Some(""), None),
        ("/.", None, None),
        ("/..", Some("/"), None),
        ("./..", Some("."), None),
        ("../..", Some(".."), None),
        ("../.", Some(""), None),
        ("./.", Some(""), None),
        ("a/.", Some(""), Some("a")),
        ("a//./", Some(""), Some("a")),
        ("/a/.", Some("/"), Some("a")),
        ("/a/.//.//", Some("/"), Some("a")),
    ];

    #[test]
    fn test_split_last_component() {
        for &(path, parent, file_name) in PARENT_AND_FILE_NAME {
            #[cfg(feature = "std")]
            {
                let path_actual = PathBuf::from(path);
                let (parent_actual, file_name_actual) =
                    (path_actual.parent(), path_actual.file_name());
                assert_eq!(
                    (parent_actual, file_name_actual),
                    (parent.map(Path::new), file_name.map(OsStr::new)),
                    "parent() and file_name() of {path:?}",
                );
            }

            {
                let path_actual = PurePosixPath::from(path);
                let (parent_actual, file_name_actual) =
                    (path_actual.parent(), path_actual.file_name());
                assert_eq!(
                    (parent_actual.as_ref().map(AsRef::as_ref), file_name_actual),
                    (parent, file_name),
                    "parent() and file_name() of {path:?}",
                );
            }

            {
                let path_actual = PureWindowsPath::from(path);
                let (parent_actual, file_name_actual) =
                    (path_actual.parent(), path_actual.file_name());
                assert_eq!(
                    (parent_actual.as_ref().map(AsRef::as_ref), file_name_actual),
                    (parent, file_name),
                    "parent() and file_name() of {path:?}",
                );
            }
        }
    }

    const JOIN: &[(&str, &str, &str)] = &[
        ("/foo", "bar", "/foo/bar"),
        ("/foo", "/bar", "/bar"),
        ("/foo/", "bar", "/foo/bar"),
        ("/foo/", "/bar", "/bar"),
        ("/foo", "/bar/", "/bar/"),
        ("/foo/", "/bar/", "/bar/"),
        ("/foo/", "/bar/baz", "/bar/baz"),
        ("/foo/", "bar/baz", "/foo/bar/baz"),
        ("/foo/", "bar/baz/", "/foo/bar/baz/"),
        ("/foo/", "/bar/baz/", "/bar/baz/"),
    ];
    #[test]
    fn join() {
        for &(a, b, c) in JOIN {
            let a = PurePosixPath::from(a);
            let b = PurePosixPath::from(b);
            let c = PurePosixPath::from(c);
            assert_eq!(a.join(&b), c, "{:?}.join({:?})", a, b);
        }
    }
}
