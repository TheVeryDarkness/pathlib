use crate::{Component, PurePath, Vec};
use core::ops::Div;

/// A unified path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnifiedPath<'a> {
    components: Vec<Component<'a>>,
}

impl<'a> FromIterator<Component<'a>> for UnifiedPath<'a> {
    fn from_iter<I: IntoIterator<Item = Component<'a>>>(iter: I) -> Self {
        Self {
            components: iter.into_iter().collect(),
        }
    }
}

impl<'a> UnifiedPath<'a> {
    /// Returns the components.
    pub fn components(&self) -> &[Component<'a>] {
        &self.components
    }
}

impl<'a> PurePath for UnifiedPath<'a> {
    fn parent(&self) -> Option<Self> {
        self.components.split_last().map(|(_, rest)| Self {
            components: rest.to_vec(),
        })
    }

    fn file_name(&self) -> Option<&str> {
        self.components.last().and_then(|c| c.as_file_name())
    }

    fn join_in_place(&mut self, path: &Self) {
        if path.is_absolute() {
            self.components.clear();
        }
        self.components.extend_from_slice(path.components());
    }

    fn join(&self, path: &Self) -> Self {
        if path.is_absolute() {
            return path.clone();
        }
        let mut new = self.clone();
        new.join_in_place(path);
        new
    }

    fn file_stem(&self) -> Option<&str> {
        todo!()
    }

    fn extension(&self) -> Option<&str> {
        todo!()
    }

    fn is_absolute(&self) -> bool {
        self.components.first() == Some(&Component::Root)
    }

    fn components(&self) -> impl DoubleEndedIterator<Item = Component<'_>> {
        self.components.iter().cloned()
    }
}

impl<'a> From<&'a crate::PosixPath> for UnifiedPath<'a> {
    fn from(path: &'a crate::PosixPath) -> Self {
        Self {
            components: path.components().collect(),
        }
    }
}

impl<'a> From<&'a crate::WindowsPath> for UnifiedPath<'a> {
    fn from(path: &'a crate::WindowsPath) -> Self {
        Self {
            components: path.components().collect(),
        }
    }
}

impl Div for UnifiedPath<'_> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        <Self as PurePath>::join_in_place(&mut self, &rhs);
        self
    }
}

impl<'a> Div for &UnifiedPath<'a> {
    type Output = UnifiedPath<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        <UnifiedPath<'a> as PurePath>::join(self, rhs)
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use crate::{Path, UnifiedPath};
    use std::{
        fs::{Metadata, ReadDir},
        io::Result,
    };

    cfg_if::cfg_if! {
        if #[cfg(target_os = "emscripten")] {
            impl Path for UnifiedPath<'_> {
                fn canonicalize(&self) -> Result<Self> {
                    todo!()
                }

                fn try_exists(&self) -> Result<bool> {
                    todo!()
                }

                fn metadata(&self) -> Result<Metadata> {
                    todo!()
                }

                fn read_dir(&self) -> Result<ReadDir> {
                    todo!()
                }

                fn read_link(&self) -> Result<Self> {
                    todo!()
                }

                fn symlink_metadata(&self) -> Result<Metadata> {
                    todo!()
                }
            }
        } else {
            impl Path for UnifiedPath<'_> {
                fn canonicalize(&self) -> Result<Self> {
                    todo!()
                }

                fn try_exists(&self) -> Result<bool> {
                    todo!()
                }

                fn metadata(&self) -> Result<Metadata> {
                    todo!()
                }

                fn read_dir(&self) -> Result<ReadDir> {
                    todo!()
                }

                fn read_link(&self) -> Result<Self> {
                    todo!()
                }

                fn symlink_metadata(&self) -> Result<Metadata> {
                    todo!()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec;

    #[test]
    fn test_unified_path() {
        let path = UnifiedPath::from_iter(vec![
            Component::Root,
            Component::Normal("foo"),
            Component::Normal("bar"),
        ]);
        assert_eq!(
            path.components(),
            &[
                Component::Root,
                Component::Normal("foo"),
                Component::Normal("bar")
            ]
        );
    }
}
