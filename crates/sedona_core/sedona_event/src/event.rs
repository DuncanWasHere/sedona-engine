use std::hash::Hash;

pub trait Event {
    type Key: Eq + Hash;

    /// Extract key to determine which handlers to run.
    fn key(&self) -> Self::Key;
}
