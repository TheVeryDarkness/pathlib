use crate::{Component, Components};

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

#[inline]
fn split_once_with_delimiter<'i>(
    s: &'i str,
    delimiter: &[char],
) -> Option<(&'i str, &'i str, &'i str)> {
    let i = s.find(delimiter)?;
    let (a, b) = s.split_at(i);
    // FIXME: This hardcodes the assumption that the delimiter is a single byte.
    let (b, c) = b.split_at(1);
    Some((a, b, c))
}

/// A path parser.
pub trait ParsablePath {
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

    /// Returns a mutable reference to the path as a string.
    fn as_string_mut(&mut self) -> &mut String;

    /// Returns the first component of the path and the rest of the path in a lexical way.
    /// That means, `..` and `.` are not resolved or even considered.
    fn split_first_lexical(path: &str) -> (&str, Option<(&str, &str)>) {
        match split_once_with_delimiter(path, Self::COMPONENT_SEPARATORS) {
            Some((prefix, separator, component)) => (prefix, Some((separator, component))),
            None => (path, None),
        }
    }

    /// Returns the first component of the path and the rest of the path.
    #[inline]
    fn split_first_component<'i>(
        mut s: &'i str,
        progressed: bool,
    ) -> (Option<Component<'i>>, Option<&'i str>) {
        loop {
            match Self::split_first_lexical(s) {
                (CURRENT_DIR, Some((_, suffix))) => match progressed {
                    true => {
                        s = suffix;
                        continue;
                    }
                    false => return (Some(Component::CurDir), Some(suffix)),
                },
                (CURRENT_DIR, None) => match progressed {
                    true => return (None, None),
                    false => return (Some(Component::CurDir), None),
                },
                (PARENT_DIR, Some((_, suffix))) => {
                    return (Some(Component::ParentDir), Some(suffix))
                }
                (PARENT_DIR, None) => return (Some(Component::ParentDir), None),
                ("", Some((_, suffix))) => match progressed {
                    true => return (None, Some(suffix)),
                    false => return (Some(Component::Root), Some(suffix)),
                },
                ("", None) => {
                    return (None, None);
                }
                (prefix, Some((_, file_name))) => {
                    return (Some(Component::Normal(prefix)), Some(file_name))
                }
                (prefix, None) => return (Some(Component::Normal(prefix)), None),
            }
        }
    }

    //
    // fn split_first_component(mut s: &str) -> (Option<Component<'_>>, Option<&str>) {
    //     loop {
    //         match Self::split_first_lexical(s) {
    //             (Some((prefix, _)), CURRENT_DIR) => {
    //                 s = prefix;
    //                 continue;
    //             }
    //             (None, CURRENT_DIR) => return (Some(Component::CurDir), None),

    //             (Some((prefix, _)), PARENT_DIR) => {
    //                 s = prefix;
    //                 continue;
    //             }
    //             (None, PARENT_DIR) => return (Some(Component::ParentDir), None),

    //             (Some((prefix, separator)), file_name) => {
    //                 return (Some(Component::Normal(prefix)), Some(file_name))
    //             }
    //             (None, file_name) => return (Some(Component::Normal("")), Some(file_name)),
    //         }
    //     }
    // }

    /// Returns the parent of the given path and the last component of the path in a lexical way.
    /// That means, `..` and `.` are not resolved or even considered.
    fn split_last_lexical(path: &str) -> (Option<(&str, &str)>, &str) {
        // while let Some(p) = path.strip_suffix(Self::PRIMARY_COMPONENT_SEPARATOR) {
        //     path = p;
        // }
        let s = path;
        match rsplit_once_with_delimiter(s, Self::COMPONENT_SEPARATORS) {
            Some((parent, separator, component)) => (Some((parent, separator)), component),
            None => (None, path),
        }
    }

    /// Returns the parent of the path and the last component of the path.
    fn split_last_component(
        mut s: &str,
        progressed: bool,
    ) -> (Option<&str>, Option<Component<'_>>) {
        loop {
            match Self::split_last_lexical(s) {
                (Some(("", _)), CURRENT_DIR) => match progressed {
                    true => return (Some(""), None),
                    false => return (Some(""), Some(Component::Root)),
                },
                (Some((parent, _)), CURRENT_DIR) => {
                    s = parent;
                    continue;
                }
                (None, CURRENT_DIR) => return (Some(""), Some(Component::CurDir)),

                (Some(("", separator)), PARENT_DIR) => {
                    return (Some(separator), Some(Component::ParentDir))
                }
                (Some((parent, _)), PARENT_DIR) => {
                    return (Some(parent), Some(Component::ParentDir))
                }
                (None, PARENT_DIR) => return (Some(""), Some(Component::ParentDir)),

                (Some(("", _)), "") => return (None, Some(Component::Root)),
                (None, "") => return (None, None),

                (Some((parent, _)), "") => {
                    s = parent;
                    continue;
                }
                (Some(("", separator)), file_name) => {
                    return (Some(separator), Some(Component::Normal(file_name)))
                }
                (Some((parent, _)), file_name) => {
                    return (Some(parent), Some(Component::Normal(file_name)))
                }
                (None, file_name) => return (Some(""), Some(Component::Normal(file_name))),
            }
        }
    }

    /// Returns the parent of the path and the last component of the path.
    fn split_last(mut s: &str) -> (Option<&str>, Option<&str>) {
        loop {
            match Self::split_last_lexical(s) {
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
        Self::split_last(s).0
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
        Self::split_last(s).1
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

    /// Joins the given path with the parent in place.
    fn join_in_place(parent: &mut String, child: &str) {
        if Self::is_absolute(child) {
            parent.clear();
            parent.push_str(child);
            return;
        }
        Self::as_dir(parent);
        parent.push_str(child.as_ref());
        // if child.is_empty() {
        //     return;
        // }
        // if child.starts_with(Self::PRIMARY_COMPONENT_SEPARATOR) {
        //     *parent = child.to_string();
        //     return;
        // }
        // if parent.ends_with(Self::PRIMARY_COMPONENT_SEPARATOR) {
        //     parent.push_str(child);
        // } else {
        //     parent.push(Self::PRIMARY_COMPONENT_SEPARATOR);
        //     parent.push_str(child);
        // }
    }

    /// Joins the given path.
    fn join(parent: &str, child: &str) -> String {
        if Self::is_absolute(child) {
            return child.to_string();
        }
        let mut joined = parent.to_string();
        Self::as_dir(&mut joined);
        joined.push_str(child.as_ref());
        joined
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
pub trait PurePath: Sized {
    /// Returns the parent of the path.
    fn parent(&self) -> Option<Self>;

    /// Returns the last component of the path, if there is one.
    fn file_name(&self) -> Option<&str>;

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
    fn join_in_place(&mut self, path: &Self);

    /// Joins the given path.
    fn join(&self, path: &Self) -> Self;

    /// Returns whether the path is absolute.
    fn is_absolute(&self) -> bool;

    /// Returns the components of the path.
    fn components(&self) -> impl DoubleEndedIterator<Item = Component<'_>>;

    // fn strip_extension(&self) -> Self;
    // fn strip_prefix(&self, prefix: &str) -> Option<Self>;
    // fn strip_suffix(&self, suffix: &str) -> Option<Self>;
    // fn starts_with(&self, path: &str) -> bool;
    // fn ends_with(&self, path: &str) -> bool;
}

impl<P: ParsablePath + Sized + AsRef<str> + for<'a> From<&'a str> + From<String>> PurePath for P {
    fn parent(&self) -> Option<Self> {
        let parent = Self::parent(self.as_ref());
        parent.map(Self::from)
    }

    fn file_name(&self) -> Option<&str> {
        Self::file_name(self.as_ref())
    }

    fn join_in_place(&mut self, path: &Self) {
        Self::join_in_place(self.as_string_mut(), path.as_ref());
    }

    fn join(&self, path: &Self) -> Self {
        let joined = Self::join(self.as_ref(), path.as_ref());
        Self::from(joined)
    }

    fn is_absolute(&self) -> bool {
        Self::is_absolute(self.as_ref())
    }

    fn components(&self) -> impl DoubleEndedIterator<Item = Component<'_>> {
        <Components<'_, Self>>::new(self.as_ref())
    }
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

    use Component::*;

    const COMPONENTS: &[(&str, &[Component<'static>])] = &[
        ("/foo/bar", &[Root, Normal("foo"), Normal("bar")]),
        ("/foo", &[Root, Normal("foo")]),
        ("/", &[Root]),
        ("foo/bar", &[Normal("foo"), Normal("bar")]),
        ("foo", &[Normal("foo")]),
        ("", &[]),
        ("/usr/bin/", &[Root, Normal("usr"), Normal("bin")]),
        ("tmp/foo.txt", &[Normal("tmp"), Normal("foo.txt")]),
        ("foo.txt/.", &[Normal("foo.txt")]),
        ("foo.txt/.//", &[Normal("foo.txt")]),
        ("foo.txt/..", &[Normal("foo.txt"), ParentDir]),
        ("/", &[Root]),
        ("//", &[Root]),
        ("/./", &[Root]),
        (".", &[CurDir]),
        ("..", &[ParentDir]),
        ("/.", &[Root]),
        ("/..", &[Root, ParentDir]),
        ("./..", &[CurDir, ParentDir]),
        ("../..", &[ParentDir, ParentDir]),
        ("../.", &[ParentDir]),
        ("./.", &[CurDir]),
        ("a/.", &[Normal("a")]),
        ("a//./", &[Normal("a")]),
        ("/a/.", &[Root, Normal("a")]),
        ("/a/.//.//", &[Root, Normal("a")]),
        ("/a/..//.//", &[Root, Normal("a"), ParentDir]),
    ];

    #[test]
    fn test_components() {
        for &(path, components) in COMPONENTS {
            #[cfg(feature = "std")]
            {
                let path_actual = PathBuf::from(path);
                let components_actual: Vec<_> = path_actual.components().collect();
                assert_eq!(components_actual, components, "components() of {path:?}");
                let components_actual: Vec<_> = path_actual
                    .components()
                    .rev()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                assert_eq!(
                    components_actual, components,
                    "components() of {path:?} in reverse",
                );
            }

            {
                let path_actual = PurePosixPath::from(path);
                let components_actual: Vec<_> = path_actual.components().collect();
                assert_eq!(components_actual, components, "components() of {path:?}");
                let components_actual: Vec<_> = path_actual
                    .components()
                    .rev()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                assert_eq!(
                    components_actual, components,
                    "components() of {path:?} in reverse",
                );
            }

            {
                let path_actual = PureWindowsPath::from(path);
                let components_actual: Vec<_> = path_actual.components().collect();
                assert_eq!(components_actual, components, "components() of {path:?}");
                let components_actual: Vec<_> = path_actual
                    .components()
                    .rev()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                assert_eq!(
                    components_actual, components,
                    "components() of {path:?} in reverse",
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
