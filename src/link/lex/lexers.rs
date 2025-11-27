use crate::link::lex::{
    Lex, Token,
    lexeme::{LexList, Lexeme::*},
};

impl<'a> Lex<'a> {
    pub fn int(&mut self) -> Option<Token<'a>> {
        let res_len = self.source.code[self.cursor..]
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .count();
        if res_len == 0 {
            None
        } else {
            let res = str::from_utf8(&self.source.code[self.cursor..self.cursor + res_len])
                .unwrap()
                .parse()
                .unwrap();
            let lexeme = Int(res);
            Some(self.token(lexeme, res_len))
        }
    }

    pub fn name(&mut self) -> Option<Token<'a>> {
        if self.cursor == self.source.code.len()
            || !is_name_first_char(self.source.code[self.cursor])
        {
            return None;
        }
        let res_len = self.source.code[self.cursor..]
            .iter()
            .take_while(|c| is_name_char(**c))
            .count();
        let res = &self.source.code[self.cursor..self.cursor + res_len];
        let lexeme = Name(str::from_utf8(res).unwrap());
        Some(self.token(lexeme, res_len))
    }

    pub fn list(&mut self, list: LexList<'a>) -> Option<Token<'a>> {
        for (s, lexeme) in list {
            if self.source.code[self.cursor..].starts_with(s) {
                return Some(self.token(*lexeme, s.len()));
            }
        }
        None
    }
}

fn is_name_first_char(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_name_char(c: u8) -> bool {
    is_name_first_char(c) || c.is_ascii_digit()
}
