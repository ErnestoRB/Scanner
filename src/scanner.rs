pub mod data;
pub mod utils;

use std::{fs::File, io::Read};

use data::*;
use utils::*;

pub fn get_token<'a>(mut text: &'a str, cursor: &mut Cursor) -> (Result<Token, Error>, &'a str) {
    let mut state: State = State::START;
    let mut result = String::new();
    let mut result_token: TokenType = TokenType::EOF;
    let mut char: char = ' ';
    let mut save: bool;
    let mut consume: bool;
    let mut start = cursor.clone();
    while !matches!(state, State::DONE) {
        save = false;
        consume = true;
        match text.chars().next() {
            Some(c) => {
                char = c;
                cursor.col += 1; // always move cursor
                match state {
                    State::START => {
                        if ['\r', '\n', '\t', ' '].contains(&c) {
                            if c == '\n' {
                                cursor.lin += 1;
                                cursor.col = 1;
                            }
                            start = cursor.clone();
                            save = false;
                        } else if c.is_ascii_digit() {
                            state = State::NUM;
                            result_token = TokenType::INT;
                            save = true;
                        } else if c.is_ascii_alphabetic() || c == '_' {
                            state = State::ID;
                            result_token = TokenType::ID;
                            save = true;
                        } else if c == '-' {
                            state = State::SUB;
                            result_token = TokenType::MIN;
                            save = true;
                        } else if c == '+' {
                            state = State::ADD;
                            result_token = TokenType::SUM;
                            save = true;
                        } else if c == '/' {
                            result_token = TokenType::DIV;
                            state = State::SLASH;
                            save = true;
                        } else if c == '!' {
                            state = State::NEG;
                            result_token = TokenType::NEG;
                            save = true;
                        } else if c == '<' {
                            state = State::LT;
                            result_token = TokenType::LT;
                            save = true;
                        } else if c == '>' {
                            state = State::GT;
                            result_token = TokenType::GT;
                            save = true;
                        } else if c == '=' {
                            save = true;
                            state = State::EQ;
                            result_token = TokenType::ASSIGN;
                        } else {
                            state = State::DONE;
                            save = true;
                            if let Some(token) = SYMBOLS.get(c.to_string().as_str()) {
                                result_token = token.clone();
                            } else {
                                let error_cursor = cursor.clone();
                                return (
                                    Err(Error {
                                        message: format!("Simbolo '{}' no permitido", c),
                                        start,
                                        end: error_cursor,
                                        lexemme: c.to_string(),
                                    }),
                                    &text[char.len_utf8()..],
                                );
                            }
                        }
                    }
                    State::EQ => {
                        if c == '=' {
                            save = true;
                            result_token = TokenType::EQ;
                        } else {
                            save = false;
                            consume = false;
                            state = State::DONE;
                            result_token = TokenType::ASSIGN;
                        }
                    }
                    State::SLASH => {
                        if c == '/' {
                            save = true;
                            state = State::LINE_COM;
                            result_token = TokenType::INLINE_COMMENT;
                        } else if c == '*' {
                            save = true;
                            state = State::BLOCK_COM_1;
                        } else {
                            save = false;
                            consume = false;
                            state = State::DONE;
                        }
                    }
                    State::NUM => {
                        if c.is_ascii_digit() {
                            save = true;
                            result_token = TokenType::INT;
                        } else if c == '.' {
                            state = State::FLOAT_DOT;
                            save = true;
                        } else {
                            result_token = TokenType::INT;
                            state = State::DONE;
                            save = false;
                            consume = false;
                        }
                    }
                    State::ID => {
                        if c == '_' || c.is_ascii_alphabetic() || c.is_ascii_digit() {
                            save = true;
                            result_token = TokenType::ID;
                        } else {
                            save = false;
                            consume = false;
                            state = State::DONE;
                            result_token = TokenType::ID;
                        }
                    }
                    State::LT => {
                        if c == '=' {
                            save = true;
                            state = State::DONE;
                            result_token = TokenType::LE;
                        } else {
                            save = false;
                            consume = false;
                            state = State::DONE;
                            result_token = TokenType::LT;
                        }
                    }
                    State::GT => {
                        if c == '=' {
                            save = true;
                            state = State::DONE;
                            result_token = TokenType::GE;
                        } else {
                            consume = false;
                            save = false;
                            state = State::DONE;
                            result_token = TokenType::GT;
                        }
                    }
                    State::NEG => {
                        if c == '=' {
                            save = true;
                            state = State::DONE;
                            result_token = TokenType::NE;
                        } else {
                            save = false;
                            consume = false;
                            state = State::DONE;
                        }
                    }
                    State::LINE_COM => {
                        if c == '\n' {
                            save = false;
                            consume = false;
                            state = State::DONE;
                            result_token = TokenType::INLINE_COMMENT;
                        } else {
                            save = true;
                            result_token = TokenType::INLINE_COMMENT;
                        }
                    }
                    State::BLOCK_COM_1 => {
                        if c == '*' {
                            save = true;
                            state = State::BLOCK_COM_2;
                        } else {
                            save = true;
                            if c == '\n' {
                                cursor.lin += 1;
                                cursor.col = 1;
                            }
                        }
                    }
                    State::BLOCK_COM_2 => {
                        if c == '/' {
                            save = true;
                            state = State::DONE;
                            result_token = TokenType::BLOCK_COMMENT;
                        } else if c == '*' {
                            save = true;
                        } else {
                            if c == '\n' {
                                cursor.lin += 1;
                                cursor.col = 1;
                            }
                            save = true;
                            state = State::BLOCK_COM_1;
                        }
                    }
                    State::FLOAT_DOT => {
                        if c.is_ascii_digit() {
                            save = true;
                            state = State::FLOAT;
                        } else {
                            cursor.col -= 1;
                            let error_cursor = cursor.clone();
                            return (
                                Err(Error {
                                    start,
                                    end: error_cursor,
                                    message:
                                        "Un número flotante debe tener números después del '.'"
                                            .to_string(),
                                    lexemme: result,
                                }),
                                &text[..],
                            );
                        }
                    }
                    State::FLOAT => {
                        if c.is_ascii_digit() {
                            result_token = TokenType::FLOAT;
                            save = true;
                        } else {
                            state = State::DONE;
                            save = false;
                            consume = false;
                        }
                    }
                    State::SUB => {
                        if c.is_ascii_digit() {
                            save = true;
                            state = State::NUM;
                        } else if c == '-' {
                            result_token = TokenType::DEC;
                            state = State::DONE;
                            save = true;
                        } else {
                            save = false;
                            consume = false;
                            state = State::DONE;
                            result_token = TokenType::MIN;
                        }
                    }
                    State::ADD => {
                        if c.is_ascii_digit() {
                            save = true;
                            state = State::NUM;
                        } else if c == '+' {
                            result_token = TokenType::INC;
                            state = State::DONE;
                            save = true;
                        } else {
                            save = false;
                            consume = false;
                            state = State::DONE;
                            result_token = TokenType::SUM;
                        }
                    }
                    State::DONE => {
                        // no deberia pasar
                    }
                }
            }
            None => {
                save = false;
                consume = false;
                if matches!(state, State::FLOAT_DOT) {
                    let new_cursor = cursor.clone();
                    return (
                        Err(Error {
                            start,
                            end: new_cursor,
                            message: "Los numeros flotantes deben ser seguidos de un número después del punto".to_string(),
                            lexemme: result
                        }),
                        text,
                    );
                }
                if matches!(state, State::BLOCK_COM_1) || matches!(state, State::BLOCK_COM_2) {
                    let new_cursor = cursor.clone();
                    return (
                        Err(Error {
                            start,
                            end: new_cursor,
                            message: "El comentario no fue terminado correctamente".to_string(),
                            lexemme: result,
                        }),
                        text,
                    );
                }
                state = State::DONE;
            }
        }
        if save {
            result.push(char);
        }
        if consume {
            text = &text[char.len_utf8()..];
        } else {
            cursor.col -= 1;
        }
    }
    if matches!(state, State::DONE) {
        if matches!(result_token, TokenType::ID) {
            result_token = reserved_lookup(result.as_str());
        }
        return (
            Ok(Token {
                token_type: result_token,
                lexemme: result,
            }),
            text,
        );
    }
    (
        Err(Error {
            message: "Unexpected error".to_string(),
            start,
            end: cursor.clone(),
            lexemme: result,
        }),
        text,
    )
}

pub fn tokenize(contents: &str) -> (Vec<Token>, Vec<Error>) {
    let mut cursor = init_cursor();
    let mut errors: Vec<Error> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let text = contents.to_owned();
    let mut rem_text = &text[..];
    loop {
        let (result, string) = get_token(&rem_text, &mut cursor);
        rem_text = string;
        match result {
            Ok(tkn) => {
                if matches!(tkn.token_type, TokenType::EOF) {
                    break;
                }
                tokens.push(tkn);
            }
            Err(err) => errors.push(err),
        }
    }
    (tokens, errors)
}

pub fn tokenize_file(file: &str) -> Result<(Vec<Token>, Vec<Error>), String> {
    let f = File::open(file);
    if let Ok(mut handle) = f {
        let mut buffer: String = String::new();
        match handle.read_to_string(&mut buffer) {
            Ok(_) => Ok(tokenize(&mut buffer)),
            Err(_e) => Err("El archivo no está en codificación UTF-8".to_string()),
        }
    } else {
        Err(format!("Could not load file '{}'", file).to_string())
    }
}

#[cfg(test)]
pub mod tests {

    use crate::{
        data::{Token, TokenType},
        get_token,
        scanner::reserved_lookup,
        utils::init_cursor,
    };

    #[test]
    pub fn keyword_lookup_match() {
        assert!(matches!(reserved_lookup("main"), TokenType::MAIN));
        assert!(matches!(reserved_lookup("stdin"), TokenType::STDIN));
        assert!(matches!(reserved_lookup("stdout"), TokenType::STDOUT));
        assert!(matches!(reserved_lookup("integer"), TokenType::INTEGER));
        assert!(matches!(reserved_lookup("double"), TokenType::DOUBLE));
        assert!(matches!(reserved_lookup("and"), TokenType::AND));
        assert!(matches!(reserved_lookup("or"), TokenType::OR));
        assert!(matches!(reserved_lookup("if"), TokenType::IF));
        assert!(matches!(reserved_lookup("else"), TokenType::ELSE));
        assert!(matches!(reserved_lookup("case"), TokenType::CASE));
        assert!(matches!(reserved_lookup("switch"), TokenType::SWITCH));
        assert!(matches!(reserved_lookup("while"), TokenType::WHILE));
        assert!(matches!(reserved_lookup("do"), TokenType::DO));
    }

    #[test]
    pub fn keyword_lookup_id() {
        assert!(matches!(reserved_lookup("_main"), TokenType::ID));
        assert!(matches!(reserved_lookup("_while"), TokenType::ID));
        assert!(matches!(reserved_lookup("_do"), TokenType::ID));
        assert!(matches!(reserved_lookup("hola123"), TokenType::ID));
    }
    #[test]
    pub fn get_token_consumes() {
        let text = String::from("int a");
        let (_, result) = get_token(&text[..], &mut init_cursor());
        assert_eq!(result, " a");
    }

    #[test]
    pub fn get_token_delimiters() {
        let text = String::from("\n\r\t");
        assert_eq!(
            get_token(&text, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: "".to_string(), // no se debe guardar esta info
                token_type: TokenType::EOF
            }
        )
    }

    #[test]
    pub fn get_token_float() {
        let mut text1 = String::from("+1289.23");
        let mut text2 = String::from("-1289.23");
        let mut text3 = String::from("1289.23");
        let mut text4 = String::from("1289.");
        let mut text5 = String::from("1289");
        assert_eq!(
            get_token(&mut text1, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text1,
                token_type: TokenType::FLOAT
            }
        );
        assert_eq!(
            get_token(&mut text2, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text2,
                token_type: TokenType::FLOAT
            }
        );
        assert_eq!(
            get_token(&mut text3, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text3,
                token_type: TokenType::FLOAT
            }
        );
        assert!(get_token(&mut text4, &mut init_cursor()).0.is_err());
        assert_ne!(
            get_token(&mut text5, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text5,
                token_type: TokenType::FLOAT
            }
        );
        let mut text = String::from("34.34.34.34");
        assert_ne!(
            get_token(&mut text, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text,
                token_type: TokenType::FLOAT
            }
        )
    }

    #[test]
    pub fn get_token_int() {
        let mut text1 = String::from("+1289");
        let mut text2 = String::from("-1289");
        let mut text3 = String::from("1289");
        let mut text4 = String::from("1289.");
        let mut text5 = String::from("asd");
        assert_eq!(
            get_token(&mut text1, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text1,
                token_type: TokenType::INT
            }
        );
        assert_eq!(
            get_token(&mut text2, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text2,
                token_type: TokenType::INT
            }
        );
        assert_eq!(
            get_token(&mut text3, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text3,
                token_type: TokenType::INT
            }
        );
        assert!(get_token(&mut text4, &mut init_cursor()).0.is_err());
        assert_ne!(
            get_token(&mut text5, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text5,
                token_type: TokenType::INT
            }
        )
    }

    #[test]
    pub fn get_token_id() {
        let mut text0 = String::from("a");
        let mut text1 = String::from("identificador");
        let mut text2 = String::from("_hola");
        let mut text3 = String::from("_var23");
        let mut text4 = String::from("_12var");
        let mut text5 = String::from("123");
        assert_eq!(
            get_token(&mut text0, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text0,
                token_type: TokenType::ID
            }
        );
        assert_eq!(
            get_token(&mut text1, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text1,
                token_type: TokenType::ID
            }
        );
        assert_eq!(
            get_token(&mut text2, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text2,
                token_type: TokenType::ID
            }
        );
        assert_eq!(
            get_token(&mut text3, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text3,
                token_type: TokenType::ID
            }
        );
        assert_eq!(
            get_token(&mut text4, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text4,
                token_type: TokenType::ID
            }
        );
        assert_ne!(
            get_token(&mut text5, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text5,
                token_type: TokenType::ID
            }
        )
    }

    #[test]
    pub fn get_token_comments() {
        let mut text1 = String::from("//");
        let mut text2 = String::from("//\n");
        let mut text3 = String::from("//Hola\n");
        let mut text4 = String::from("/ / Hola");
        let mut text5 = String::from("/**/");
        let mut text6 = String::from("/* asd asd asd 123 1_ */");
        let mut text7 = String::from("/**adasd/");
        let mut text8 = String::from("/*/");
        let mut text9 = String::from("/*");
        let mut text10 = String::from("/**");
        assert_eq!(
            get_token(&mut text1, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text1,
                token_type: TokenType::INLINE_COMMENT
            }
        );
        assert_eq!(
            get_token(&mut text2, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: "//".to_string(),
                token_type: TokenType::INLINE_COMMENT
            }
        );
        assert_eq!(
            get_token(&mut text3, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: "//Hola".to_string(),
                token_type: TokenType::INLINE_COMMENT
            }
        );
        assert_ne!(
            get_token(&mut text4, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text4,
                token_type: TokenType::INLINE_COMMENT
            }
        );
        assert_eq!(
            get_token(&mut text5, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text5,
                token_type: TokenType::BLOCK_COMMENT
            }
        );
        assert_eq!(
            get_token(&mut text6, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: text6,
                token_type: TokenType::BLOCK_COMMENT
            }
        );
        assert!(get_token(&mut text7, &mut init_cursor()).0.is_err());
        assert!(get_token(&mut text8, &mut init_cursor()).0.is_err());
        assert!(get_token(&mut text9, &mut init_cursor()).0.is_err());
        assert!(get_token(&mut text10, &mut init_cursor()).0.is_err());
    }

    #[test]
    pub fn get_token_operators() {
        let mut operator = String::from("+");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::SUM
            }
        );
        let mut operator = String::from("-");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::MIN
            }
        );
        let mut operator = String::from("*");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::TIMES
            }
        );
        let mut operator = String::from("/");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::DIV
            }
        );
        let mut operator = String::from("%");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::MODULUS
            }
        );
        let mut operator = String::from("^");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::POWER
            }
        );
        let mut operator = String::from("++");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::INC
            }
        );
        let mut operator = String::from("--");
        assert_eq!(
            get_token(&mut operator, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: operator,
                token_type: TokenType::DEC
            }
        );
    }

    #[test]
    pub fn get_token_symbols() {
        let mut symbol = String::from(",");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::COMMA
            }
        );
        let mut symbol = String::from(".");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::DOT
            }
        );
        let mut symbol = String::from(";");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::SCOL
            }
        );
        let mut symbol = String::from("(");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::LPAR
            }
        );
        let mut symbol = String::from(")");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::RPAR
            }
        );
        let mut symbol = String::from("{");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::LBRA
            }
        );
        let mut symbol = String::from("}");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::RBRA
            }
        );
    }

    #[test]
    pub fn get_token_rel_op() {
        let mut symbol = String::from("!");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::NEG
            }
        );
        let mut symbol = String::from("!=");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::NE
            }
        );
        let mut symbol = String::from("==");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::EQ
            }
        );
        let mut symbol = String::from("<");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::LT
            }
        );
        let mut symbol = String::from("<=");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::LE
            }
        );
        let mut symbol = String::from(">");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::GT
            }
        );
        let mut symbol = String::from(">=");
        assert_eq!(
            get_token(&mut symbol, &mut init_cursor()).0.unwrap(),
            Token {
                lexemme: symbol,
                token_type: TokenType::GE
            }
        );
    }
}
