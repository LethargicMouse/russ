use crate::source::{Source, pos::Pos, view::View};

impl Source {
    pub fn view_at(&'_ self, pos: Pos) -> View<'_> {
        self.view(pos, pos.next(' '))
    }

    pub fn view(&'_ self, start: Pos, end: Pos) -> View<'_> {
        View {
            src_name: &self.name,
            lines: &self.lines,
            start,
            end,
        }
    }
}
