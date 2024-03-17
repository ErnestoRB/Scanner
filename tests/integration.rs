use std::{
    env::join_paths,
    path::{self, Path},
};

use ::scanner::{
    data::{Cursor, TokenType},
    *,
};

#[test]
fn it_tokenize_correctly() {
    let (tokens, errors) = tokenize(r"int algo = 192");
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens.get(0).unwrap().token_type, TokenType::INTEGER);
    assert_eq!(tokens.get(1).unwrap().token_type, TokenType::ID);
    assert_eq!(tokens.get(2).unwrap().token_type, TokenType::ASSIGN);
    assert_eq!(tokens.get(3).unwrap().token_type, TokenType::INT);
    assert_eq!(errors.len(), 0);
}

#[test]
fn it_tokenize_errors() {
    let (tokens, errors) = tokenize(r"int algo = 192.");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.get(0).unwrap().token_type, TokenType::INTEGER);
    assert_eq!(tokens.get(1).unwrap().token_type, TokenType::ID);
    assert_eq!(tokens.get(2).unwrap().token_type, TokenType::ASSIGN);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors.get(0).unwrap().position, Cursor { col: 15, lin: 1 })
}

#[test]
fn it_tokenize() {
    let text = r"/** Este es un comentario **/    
    float algo = 192.23;
    float _pos_algo = +192.23;
    float _neg_algo = +192.23;
    int algo2 = 123;
    int _res =  algo + algo2;
    // comentario
    if (_res) { // otro comentario

    }
    switch case main do while
    <= >= == != - * ^ . %
    ";
    let (tokens, errors) = tokenize(text);
    assert_eq!(errors.len(), 0);
    assert_eq!(tokens.len(), 1 + 5 + 5 + 5 + 5 + 7 + 1 + 6 + 1 + 5 + 9);
}

#[test]
fn it_tokenize_file() {
    let path = Path::new(".").join("data").join("test.cat");
    let result = tokenize_file(path.to_str().unwrap());
    if let Ok((tokens, errors)) = result {
        assert_eq!(errors.len(), 0);
        assert_eq!(tokens.len(), 5 + 5 + 5 + 9 + 6 + 1 + 1 + 5 + 7 + 6 + 1 + 1);
    } else if let Err(e) = result {
        println!("{}", e)
    }
}

#[test]
fn it_tokenize_file_errors() {
    let path = Path::new(".").join("data").join("test_errors.cat");
    let result = tokenize_file(path.to_str().unwrap());
    if let Ok((tokens, errors)) = result {
        assert_eq!(errors.len(), 3);
        assert_eq!(errors.get(0).unwrap().position, Cursor { col: 9, lin: 2 });
        assert_eq!(errors.get(1).unwrap().position, Cursor { col: 5, lin: 3 });
        assert_eq!(errors.get(2).unwrap().position, Cursor { col: 6, lin: 20 });
        assert_eq!(tokens.len(), 5 + 5 + 5 + 9 + 6 + 1 + 1 + 5 + 7 + 6 + 1 + 1);
    } else if let Err(e) = result {
        println!("{}", e)
    }
}
