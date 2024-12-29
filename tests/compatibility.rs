use pathlib::{Component, Component::*, PosixPath, PurePath, WindowsPath};
#[cfg(feature = "std")]
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
            let (parent_actual, file_name_actual) = (path_actual.parent(), path_actual.file_name());
            assert_eq!(
                (parent_actual, file_name_actual),
                (parent.map(Path::new), file_name.map(OsStr::new)),
                "parent() and file_name() of {path:?}",
            );
        }

        {
            let path_actual = PosixPath::from(path);
            let (parent_actual, file_name_actual) = (path_actual.parent(), path_actual.file_name());
            assert_eq!(
                (parent_actual.as_ref().map(AsRef::as_ref), file_name_actual),
                (parent, file_name),
                "parent() and file_name() of {path:?}",
            );
        }

        {
            let path_actual = WindowsPath::from(path);
            let (parent_actual, file_name_actual) = (path_actual.parent(), path_actual.file_name());
            assert_eq!(
                (parent_actual.as_ref().map(AsRef::as_ref), file_name_actual),
                (parent, file_name),
                "parent() and file_name() of {path:?}",
            );
        }
    }
}

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
    ("/./.", &[Root]),
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
            let path_actual = PosixPath::from(path);
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
            let path_actual = WindowsPath::from(path);
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

const JOIN: &[(&str, &str, &str, &str)] = &[
    ("/foo", "bar", "/foo/bar", "/foo\\bar"),
    ("/foo", "/bar", "/bar", "/bar"),
    ("/foo/", "bar", "/foo/bar", "/foo/bar"),
    ("/foo/", "/bar", "/bar", "/bar"),
    ("/foo", "/bar/", "/bar/", "/bar/"),
    ("/foo/", "/bar/", "/bar/", "/bar/"),
    ("/foo/", "/bar/baz", "/bar/baz", "/bar/baz"),
    ("/foo/", "bar/baz", "/foo/bar/baz", "/foo/bar/baz"),
    ("/foo/", "bar/baz/", "/foo/bar/baz/", "/foo/bar/baz/"),
    ("/foo/", "/bar/baz/", "/bar/baz/", "/bar/baz/"),
];
#[test]
fn join() {
    for &(a, b, c, d) in JOIN {
        #[cfg(feature = "std")]
        {
            let a = PathBuf::from(a);
            let b = PathBuf::from(b);
            let c = PathBuf::from(c);
            assert_eq!(a.join(&b), c, "{:?}.join({:?})", a, b);
        }
        {
            let a = PosixPath::from(a);
            let b = PosixPath::from(b);
            let c = PosixPath::from(c);
            assert_eq!(a.join(&b), c, "{:?}.join({:?})", a, b);
            assert_eq!(&a / &b, c, "{:?}.join({:?})", a, b);
            assert_eq!(a.clone() / b.clone(), c, "{:?}.join({:?})", a, b);
        }
        {
            let a = WindowsPath::from(a);
            let b = WindowsPath::from(b);
            let c = WindowsPath::from(d);
            assert_eq!(a.join(&b), c, "{:?}.join({:?})", a, b);
            assert_eq!(&a / &b, c, "{:?}.join({:?})", a, b);
            assert_eq!(a.clone() / b.clone(), c, "{:?}.join({:?})", a, b);
        }
    }
}
