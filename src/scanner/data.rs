#![allow(non_camel_case_types)]

use phf::phf_map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TokenType {
    INT,
    FLOAT,
    ID,
    INLINE_COMMENT,
    BLOCK_COMMENT,
    // Palabras reservadas
    IF,
    ELSE,
    DO,
    WHILE,
    SWITCH,
    CASE,
    INTEGER,
    DOUBLE,
    // objetos
    STDIN,
    STDOUT,
    MAIN,
    // Operadores aritmeticos
    SUM,
    MIN,
    TIMES,
    DIV,
    MODULUS,
    POWER,
    INC, // ++
    DEC, // --
    // Operadores relacionales
    LT,
    LE,
    GT,
    GE,
    NE,
    EQ,
    NEG, // !
    // Operadores logicos
    AND,
    OR,
    // Simbolos
    COMMA,
    LPAR,
    RPAR,
    LBRA,
    RBRA,
    SCOL,
    // Asignacion
    ASSIGN,
    //EOF
    EOF,
    // Ninguno
    NONE,
}

pub enum State {
    START,
    SLASH,
    LINE_COM,
    BLOCK_COM_1,
    BLOCK_COM_2,
    FLOAT,
    FLOAT_DOT,
    SUB,
    ADD,
    NUM,
    ID,
    LT,
    GT,
    NEG,
    EQ,
    DONE,
}

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "if" => TokenType::IF,
    "else" => TokenType::ELSE,
    "do" => TokenType::DO,
    "while" => TokenType::WHILE,
    "switch" => TokenType::SWITCH,
    "case" => TokenType::CASE,
    "integer" => TokenType::INTEGER,
    "double" => TokenType::DOUBLE,
    "main" => TokenType::MAIN,
    "and" => TokenType::AND,
    "or" => TokenType::OR,
    "stdin" => TokenType::STDIN,
    "stdout" => TokenType::STDOUT
};

pub static SYMBOLS: phf::Map<&'static str, TokenType> = phf_map! {
    // operators
    "*" => TokenType::TIMES,
    "%" => TokenType::MODULUS,
    "^" => TokenType::POWER,
    "," => TokenType::COMMA,
    "(" => TokenType::LPAR,
    ")" => TokenType::RPAR,
    "{" => TokenType::LBRA,
    "}" => TokenType::RBRA,
    ";" => TokenType::SCOL,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexemme: String,
    pub start: Cursor,
    pub end: Cursor,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cursor {
    pub col: i32,
    pub lin: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Error {
    pub start: Cursor,
    pub end: Cursor,
    pub message: String,
    pub lexemme: String,
}
