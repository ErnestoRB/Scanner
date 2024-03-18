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
    MAIN,
    // Operadores aritmeticos
    SUM,
    MIN,
    TIMES,
    DIV,
    MODULUS,
    POWER,
    // Operadores relacionales
    LT,
    LE,
    GT,
    GE,
    NE,
    EQ,
    // Operadores logicos
    AND,
    OR,
    // Simbolos
    DOT,
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
    "int" => TokenType::INTEGER,
    "double" => TokenType::DOUBLE,
    "main" => TokenType::MAIN,
    "and" => TokenType::AND,
    "or" => TokenType::OR
};

pub static SYMBOLS: phf::Map<&'static str, TokenType> = phf_map! {
    // operators
    "*" => TokenType::TIMES,
    "%" => TokenType::MODULUS,
    "^" => TokenType::POWER,
    "." => TokenType::DOT,
    "," => TokenType::COMMA,
    "(" => TokenType::LPAR,
    ")" => TokenType::RPAR,
    "{" => TokenType::LBRA,
    "}" => TokenType::RBRA,
    ";" => TokenType::SCOL
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexemme: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cursor {
    pub col: i32,
    pub lin: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub position: Cursor,
    pub message: String,
    pub lexemme: String,
}
