use std::cmp::Ordering;

use crate::source::parser::Parser;

pub struct ParseFailed;
pub type PF = ParseFailed;

impl<'a> Parser<'a> {
    pub fn fail<T>(&mut self, msg: &'static str) -> Result<T, PF> {
        match self.cursor.cmp(&self.err_cursor) {
            Ordering::Less => {}
            Ordering::Equal => {
                self.err_msgs.insert(msg);
            }
            Ordering::Greater => {
                self.err_cursor = self.cursor;
                self.err_msgs.clear();
                self.err_msgs.insert(msg);
            }
        }
        Err(ParseFailed)
    }
}
