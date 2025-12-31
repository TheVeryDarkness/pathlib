use crate::pure::ParsablePath;
use core::marker::PhantomData;

/// A path component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Component<'a> {
    /// A path prefix.
    Prefix(&'a str),
    /// A root component.
    Root,
    /// A current directory component.
    CurDir,
    /// A parent directory component.
    ParentDir,
    /// A normal component.
    Normal(&'a str),
}

impl Component<'_> {
    /// Returns the component as file name.
    pub fn as_file_name(&self) -> Option<&str> {
        match self {
            Component::Normal(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::*;
    use std::path::Component as StdComponent;

    impl<'a> TryFrom<StdComponent<'a>> for Component<'a> {
        type Error = ();

        fn try_from(c: StdComponent<'a>) -> Result<Self, Self::Error> {
            match c {
                StdComponent::Prefix(p) => Ok(Component::Prefix(p.as_os_str().to_str().ok_or(())?)),
                StdComponent::RootDir => Ok(Component::Root),
                StdComponent::CurDir => Ok(Component::CurDir),
                StdComponent::ParentDir => Ok(Component::ParentDir),
                StdComponent::Normal(p) => Ok(Component::Normal(p.to_str().ok_or(())?)),
            }
        }
    }

    impl PartialEq<StdComponent<'_>> for Component<'_> {
        fn eq(&self, other: &StdComponent<'_>) -> bool {
            match (self, other) {
                (Component::Prefix(a), StdComponent::Prefix(b)) => {
                    Some(*a) == b.as_os_str().to_str()
                }
                (Component::Root, StdComponent::RootDir) => true,
                (Component::CurDir, StdComponent::CurDir) => true,
                (Component::ParentDir, StdComponent::ParentDir) => true,
                (Component::Normal(a), StdComponent::Normal(b)) => a == b,
                _ => false,
            }
        }
    }

    impl<'a> PartialEq<Component<'a>> for StdComponent<'a> {
        fn eq(&self, other: &Component<'a>) -> bool {
            other == self
        }
    }
}

/// An iterator over the [Component]s of a path.
pub struct Components<'a, P> {
    s: &'a str,
    p: PhantomData<P>,
    progressed_front: bool,
    progressed_back: bool,
}

impl<'a, P> Components<'a, P> {
    /// Creates a new [Components] iterator.
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            p: PhantomData,
            progressed_front: false,
            progressed_back: false,
        }
    }
}

impl<'a, P: ParsablePath> Iterator for Components<'a, P> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = P::split_first_component(self.s, self.progressed_front);
        self.progressed_front = true;
        self.s = rest.unwrap_or("");
        first
    }
}

impl<P: ParsablePath> DoubleEndedIterator for Components<'_, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (rest, last) = P::split_last_component(self.s, self.progressed_back);
        self.progressed_back = true;
        self.s = rest.unwrap_or("");
        last
    }
}
