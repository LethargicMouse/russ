use std::{cmp::Ordering, fmt::Display};

use crate::{link::parse::Parse, location::Location};

impl<'a> Parse<'a> {
    pub fn error(self) -> Error<'a> {
        let location = self.tokens[self.err_cursor].location;
        let mut msgs = self.msgs;
        msgs.sort();
        msgs.dedup();
        Error { location, msgs }
    }

    pub fn fail<T>(&mut self, s: &'static str) -> Result<T, Fail> {
        match self.cursor.cmp(&self.err_cursor) {
            Ordering::Less => {}
            Ordering::Equal => self.msgs.push(s),
            Ordering::Greater => {
                self.err_cursor = self.cursor;
                self.msgs.clear();
                self.msgs.push(s)
            }
        }
        Err(Fail)
    }
}

pub struct Fail;

pub struct Error<'a> {
    location: Location<'a>,
    msgs: Vec<&'static str>,
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error parsing {}\n--! expected:", self.location)?;
        for msg in &self.msgs {
            write!(f, "\n    - {msg}")?;
        }
        Ok(())
    }
}
