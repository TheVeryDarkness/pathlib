use pathlib::{
    Component::{self, *},
    PosixPath, PurePath, UnifiedPath, WindowsPath,
};
#[cfg(feature = "std")]
use std::{
    ffi::OsStr,
    fs,
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
            let components = path_actual
                .components()
                .map(Component::try_from)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let unified_path_actual = UnifiedPath::from_iter(components);
            assert_eq!(
                unified_path_actual.components(),
                path_actual.components().collect::<Vec<_>>(),
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

        {
            let posix_path_actual = PosixPath::from(path);
            let path_actual = UnifiedPath::from(&posix_path_actual);
            assert_eq!(
                path_actual.components(),
                posix_path_actual.components().collect::<Vec<_>>(),
            );
            let file_name_actual = path_actual.file_name();
            assert_eq!(file_name_actual, file_name, "file_name() of {path:?}",);
        }

        {
            let windows_path_actual = WindowsPath::from(path);
            let path_actual = UnifiedPath::from(&windows_path_actual);
            assert_eq!(
                path_actual.components(),
                windows_path_actual.components().collect::<Vec<_>>(),
            );
            let file_name_actual = path_actual.file_name();
            assert_eq!(file_name_actual, file_name, "file_name() of {path:?}",);
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
            {
                let a = UnifiedPath::from(&a);
                let b = UnifiedPath::from(&b);
                let c = UnifiedPath::from(&c);
                assert_eq!(a.join(&b), c, "{:?}.join({:?})", a, b);
                assert_eq!(&a / &b, c, "{:?}.join({:?})", a, b);
                assert_eq!(a.clone() / b.clone(), c, "{:?}.join({:?})", a, b);
            }
        }
        {
            let a = WindowsPath::from(a);
            let b = WindowsPath::from(b);
            let d = WindowsPath::from(d);
            assert_eq!(a.join(&b), d, "{:?}.join({:?})", a, b);
            assert_eq!(&a / &b, d, "{:?}.join({:?})", a, b);
            assert_eq!(a.clone() / b.clone(), d, "{:?}.join({:?})", a, b);
            {
                let a = UnifiedPath::from(&a);
                let b = UnifiedPath::from(&b);
                let c = UnifiedPath::from(&d);
                assert_eq!(a.join(&b), c, "{:?}.join({:?})", a, b);
                assert_eq!(&a / &b, c, "{:?}.join({:?})", a, b);
                assert_eq!(a.clone() / b.clone(), c, "{:?}.join({:?})", a, b);
            }
        }
    }
}

/// (path, file stem, extension)
const EXTENSION: &[(&str, Option<&str>, Option<&str>)] = &[
    ("/foo/bar.txt", Some("bar"), Some("txt")),
    ("/foo/bar", Some("bar"), None),
    ("/foo/.bar", Some(".bar"), None),
    ("/foo/.bar.txt", Some(".bar"), Some("txt")),
    (".", None, None),
    ("..", None, None),
    ("/.", None, None),
    ("/..", None, None),
    ("foo/.", Some("foo"), None),
    ("foo/..", None, None),
    ("/foo/.", Some("foo"), None),
    ("/foo/..", None, None),
    ("./foo/.", Some("foo"), None),
    ("./foo/..", None, None),
    ("../foo/.", Some("foo"), None),
    ("../foo/..", None, None),
    ("foo", Some("foo"), None),
    ("foo.", Some("foo"), Some("")),
    ("foo..", Some("foo."), Some("")),
    ("foo...", Some("foo.."), Some("")),
    ("foo.txt", Some("foo"), Some("txt")),
    ("foo.txt.", Some("foo.txt"), Some("")),
    ("foo.txt..", Some("foo.txt."), Some("")),
    ("foo.txt...", Some("foo.txt.."), Some("")),
    ("/foo/bar.", Some("bar"), Some("")),
    ("/foo/bar.txt.", Some("bar.txt"), Some("")),
    ("/foo/bar.txt..", Some("bar.txt."), Some("")),
    ("/foo/bar.txt...", Some("bar.txt.."), Some("")),
    ("/foo/bar.txt", Some("bar"), Some("txt")),
];

#[test]
fn extension() {
    for &(path, stem, extension) in EXTENSION {
        #[cfg(feature = "std")]
        {
            let path = PathBuf::from(path);
            assert_eq!(
                (
                    path.file_stem().map(OsStr::new),
                    path.extension().map(OsStr::new)
                ),
                (stem.map(OsStr::new), extension.map(OsStr::new)),
                "file_stem() and extension() of {path:?}",
            );
        }

        {
            let path = PosixPath::from(path);
            assert_eq!(
                (path.file_stem(), path.extension()),
                (stem, extension),
                "file_stem() and extension() of {path:?}",
            );
        }

        {
            let path = WindowsPath::from(path);
            assert_eq!(
                (path.file_stem(), path.extension()),
                (stem, extension),
                "file_stem() and extension() of {path:?}",
            );
        }
    }
}

#[test]
fn fs() {
    #[cfg(feature = "std")]
    {
        fs::create_dir("./tmp").unwrap();
        let dir = PathBuf::from("./tmp");
        assert!(dir.exists());
        let metadata = dir.metadata().unwrap();
        assert!(metadata.is_dir());
        assert!(!metadata.is_file());
        assert!(!metadata.is_symlink());
        assert!(dir.read_link().is_err());
        assert!(dir.symlink_metadata().unwrap().is_dir());
        assert!(dir.canonicalize().is_ok());
        assert!(dir.try_exists().unwrap());
        assert!(dir.read_dir().is_ok());

        let file_1 = PathBuf::from("./tmp/foo.txt");
        fs::write(&file_1, b"Hello, world!").unwrap();
        assert!(file_1.exists());
        let metadata = file_1.metadata().unwrap();
        assert!(!metadata.is_dir());
        assert!(metadata.is_file());
        assert!(!metadata.is_symlink());
        assert!(file_1.read_link().is_err());
        assert!(file_1.symlink_metadata().unwrap().is_file());
        assert!(file_1.canonicalize().is_ok());
        assert!(file_1.try_exists().unwrap());
        assert!(file_1.read_dir().is_err());

        let file_2 = PathBuf::from("./tmp/bar.txt");
        fs::write(&file_2, b"Hello, world!").unwrap();
        assert!(file_2.exists());
        let metadata = file_2.metadata().unwrap();
        assert!(!metadata.is_dir());
        assert!(metadata.is_file());
        assert!(!metadata.is_symlink());
        assert!(file_2.read_link().is_err());
        assert!(file_2.symlink_metadata().unwrap().is_file());
        assert!(file_2.canonicalize().is_ok());
        assert!(file_2.try_exists().unwrap());
        assert!(file_2.read_dir().is_err());

        let files = dir
            .read_dir()
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|entry| entry.path() == file_1));
        assert!(files.iter().any(|entry| entry.path() == file_2));

        fs::remove_file(file_2).unwrap();
        fs::remove_file(file_1).unwrap();

        fs::remove_dir(dir).unwrap();
    }
}
