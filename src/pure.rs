use crate::{Component, Components, String, ToOwned};

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
pub(crate) trait ParsablePath {
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
    /// The current directory.
    const CURRENT_DIR: &'static str;
    /// The parent directory.
    const PARENT_DIR: &'static str;

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
    fn split_first_component(
        mut s: &str,
        progressed: bool,
    ) -> (Option<Component<'_>>, Option<&str>) {
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

    /// Returns the parent of the given path and the last component of the path in a lexical way.
    /// That means, `..` and `.` are not resolved or even considered.
    fn split_last_lexical(path: &str) -> (Option<(&str, &str)>, &str) {
        let s = path;
        match rsplit_once_with_delimiter(s, Self::COMPONENT_SEPARATORS) {
            Some((parent, separator, component)) => (Some((parent, separator)), component),
            None => (None, path),
        }
    }

    /// Returns the parent of the path and the last component of the path.
    #[inline]
    fn split_last_component(mut s: &str, _: bool) -> (Option<&str>, Option<Component<'_>>) {
        loop {
            match Self::split_last_lexical(s) {
                (Some(("", _)), CURRENT_DIR) => return (Some(""), Some(Component::Root)),
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
    }

    /// Returns the last component of the path, if there is one.
    fn file_name(s: &str) -> Option<&str> {
        Self::split_last(s).1
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
    }

    /// Joins the given path.
    fn join(parent: &str, child: &str) -> String {
        if Self::is_absolute(child) {
            return child.to_owned();
        }
        let mut joined = parent.to_owned();
        Self::as_dir(&mut joined);
        joined.push_str(child.as_ref());
        joined
    }

    /// Returns the file stem and extension of the path.
    fn split_extension(s: &str) -> (Option<&str>, Option<&str>) {
        if let Some(s) = Self::file_name(s) {
            match rsplit_once_with_delimiter(s, &[Self::EXTENSION_SEPARATOR]) {
                Some(("", _, _)) => (Some(s), None),
                Some((stem, _, extension)) => (Some(stem), Some(extension)),
                None => (Some(s), None),
            }
        } else {
            (None, None)
        }
    }

    /// Returns the file stem of the path.
    fn file_stem(s: &str) -> Option<&str> {
        let (stem, _) = Self::split_extension(s);
        stem
    }

    /// Returns the extension of the path.
    fn extension(s: &str) -> Option<&str> {
        let (_, ext) = Self::split_extension(s);
        ext
    }

    /// Replace the extension of the path with the given extension in place.
    fn with_extension(path: &str, ext: &str) -> String {
        let (path, _) = Self::split_extension(path);
        let mut new = path.unwrap_or("").to_owned();
        if !ext.is_empty() {
            new.push(Self::EXTENSION_SEPARATOR);
            new.push_str(ext);
        }
        new
    }

    /// Returns the driver of the path and the rest of the path.
    #[expect(dead_code, reason = "reserved for future use")]
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
        path.starts_with(Self::COMPONENT_SEPARATORS)
    }

    /// Returns whether the path is relative.
    #[expect(dead_code, reason = "reserved for future use")]
    fn is_relative(path: &str) -> bool {
        !Self::is_absolute(path)
    }

    /// Append component separator if not already present.
    fn as_dir(path: &mut String) {
        if !path.ends_with(Self::COMPONENT_SEPARATORS) {
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

    /// Joins the given path in place.
    fn join_in_place(&mut self, path: &Self);

    /// Joins the given path.
    fn join(&self, path: &Self) -> Self;

    /// Returns the file stem of the path.
    fn file_stem(&self) -> Option<&str>;

    /// Returns the extension of the path.
    fn extension(&self) -> Option<&str>;

    /// Replace the extension of the path with the given extension.
    fn with_extension(&mut self, ext: &str) -> Self;

    /// Returns whether the path is absolute.
    fn is_absolute(&self) -> bool;

    /// Returns whether the path is relative.
    fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

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

    fn file_stem(&self) -> Option<&str> {
        <Self as ParsablePath>::file_stem(self.as_ref())
    }

    fn extension(&self) -> Option<&str> {
        <Self as ParsablePath>::extension(self.as_ref())
    }

    fn with_extension(&mut self, ext: &str) -> Self {
        let new = <Self as ParsablePath>::with_extension(self.as_ref(), ext);
        Self::from(new)
    }

    fn is_absolute(&self) -> bool {
        Self::is_absolute(self.as_ref())
    }

    fn components(&self) -> impl DoubleEndedIterator<Item = Component<'_>> {
        <Components<'_, Self>>::new(self.as_ref())
    }
}
