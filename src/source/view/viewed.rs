use std::fmt::Debug;

use crate::source::view::View;

pub struct Viewed<'a, T> {
    pub view: View<'a>,
    pub un: T,
}

impl<T: Debug> Debug for Viewed<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Viewed").field("un", &self.un).finish()
    }
}
