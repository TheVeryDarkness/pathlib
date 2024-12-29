use crate::{vec, Component, PurePath, Vec};

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

impl<'a> IntoIterator for UnifiedPath<'a> {
    type Item = Component<'a>;
    type IntoIter = vec::IntoIter<Component<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
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
        self.components.last().map(|c| c.as_str())
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
