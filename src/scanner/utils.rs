use core::str;

use crate::data::{Cursor, Error, TokenType, KEYWORDS};

pub fn fake_cursor() -> Cursor {
    Cursor { col: 1, lin: 1 }
}

pub fn init_cursor() -> Cursor {
    Cursor { col: 1, lin: 1 }
}

pub fn fake_error() -> Error {
    Error {
        start: fake_cursor(),
        end: fake_cursor(),
        message: "Fake Error".to_string(),
        lexemme: "fake".to_string(),
    }
}

pub fn reserved_lookup(id: &str) -> TokenType {
    KEYWORDS.get(id).cloned().unwrap_or(TokenType::ID)
}
