use std::path::Path;

use ::scanner::{
    data::{Cursor, TokenType},
    *,
};

#[test]
fn it_tokenize_correctly() {
    let (tokens, errors) = tokenize(r"integer algo = 192");
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens.get(0).unwrap().token_type, TokenType::INTEGER);
    assert_eq!(tokens.get(1).unwrap().token_type, TokenType::ID);
    assert_eq!(tokens.get(2).unwrap().token_type, TokenType::ASSIGN);
    assert_eq!(tokens.get(3).unwrap().token_type, TokenType::INT);
    assert_eq!(errors.len(), 0);
}

#[test]
fn it_tokenize_errors() {
    let (tokens, errors) = tokenize(r"integer algo = 192.");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.get(0).unwrap().token_type, TokenType::INTEGER);
    assert_eq!(tokens.get(1).unwrap().token_type, TokenType::ID);
    assert_eq!(tokens.get(2).unwrap().token_type, TokenType::ASSIGN);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors.get(0).unwrap().start, Cursor { col: 16, lin: 1 });
    assert_eq!(errors.get(0).unwrap().end, Cursor { col: 20, lin: 1 })
}

#[test]
fn it_tokenize() {
    let text = r"/** Este es un comentario **/ 
    integer a = 12;  // Identificador de una sola letra 
    float algo = 192.23;
    float _pos_algo = +192.23;
    float _neg_algo = +192.23;
    integer algo2 = 123;
    integer _res =  algo + algo2;
    // comentario
    if (_res) { // otro comentario

    }
    switch case main do while
    <= >= == != - * ^ %
    ";
    let (tokens, errors) = tokenize(text);
    assert_eq!(errors.len(), 0);
    assert_eq!(tokens.len(), 5 + 5 + 5 + 5 + 5 + 7 + 5 + 1 + 5 + 8);
}

#[test]
fn it_tokenize_file() {
    let path = Path::new(".").join("data").join("test.cat");
    let result = tokenize_file(path.to_str().unwrap());
    if let Ok((tokens, errors)) = result {
        assert_eq!(errors.len(), 0);
        assert_eq!(tokens.len(), 5 + 5 + 5 + 9 + 6 + 1 + 5 + 7 + 5 + 1 + 1);
    } else if let Err(e) = result {
        println!("{}", e)
    }
}

#[test]
fn it_tokenize_file_errors() {
    let path = Path::new(".").join("data").join("test_errors.cat");
    let result = tokenize_file(path.to_str().unwrap());
    if let Ok((tokens, errors)) = result {
        assert_eq!(errors.len(), 5);
        assert_eq!(errors.get(0).unwrap().start, Cursor { col: 5, lin: 2 });
        assert_eq!(errors.get(0).unwrap().end, Cursor { col: 10, lin: 2 });
        assert_eq!(errors.get(1).unwrap().start, Cursor { col: 5, lin: 3 });
        assert_eq!(errors.get(1).unwrap().end, Cursor { col: 6, lin: 3 });
        assert_eq!(errors.get(2).unwrap().start, Cursor { col: 5, lin: 17 });
        assert_eq!(errors.get(2).unwrap().end, Cursor { col: 6, lin: 17 });
        assert_eq!(errors.get(3).unwrap().start, Cursor { col: 6, lin: 17 });
        assert_eq!(errors.get(3).unwrap().end, Cursor { col: 7, lin: 17 });
        assert_eq!(errors.get(4).unwrap().start, Cursor { col: 1, lin: 20 });
        assert_eq!(errors.get(4).unwrap().end, Cursor { col: 7, lin: 20 });
        assert_eq!(tokens.len(), 5 + 5 + 5 + 9 + 5 + 1 + 5 + 7 + 6 + 1 + 1);
    } else if let Err(e) = result {
        println!("{}", e);
    }
}

#[test]
fn it_tokenize_large_file() {
    let path = Path::new(".").join("data").join("test_large.cat");
    let result = tokenize_file(path.to_str().unwrap());
    if let Ok((tokens, errors)) = result {
        assert_eq!(errors.len(), 3);
        assert_eq!(errors.get(0).unwrap().start, Cursor { col: 9, lin: 4 });
        assert_eq!(errors.get(0).unwrap().end, Cursor { col: 10, lin: 4 });
        assert_eq!(errors.get(1).unwrap().start, Cursor { col: 25, lin: 4 });
        assert_eq!(errors.get(1).unwrap().end, Cursor { col: 28, lin: 4 });
        assert_eq!(errors.get(2).unwrap().start, Cursor { col: 6, lin: 5 });
        assert_eq!(errors.get(2).unwrap().end, Cursor { col: 7, lin: 5 });
        assert_eq!(
            tokens.len(),
            10
                + 2
                + 1
                + 7
                + 7
                + 4
                + 4
                + 4
                + 6
                + 5
                + 5
                + 12
                + 13
                + 11
                + 9
                + 4
                + 7
                + 5
                + 1
                + 8
                + 4
                + 1
                + 4
                + 2
                + 5
                + 2
                + 2
                + 2
                + 3
                + 5
                + 1
                + 10
                + 18
                + 3
                + 6
                + 2
                + 4
                + 1
                + 3
                + 7
                + 3
                + 3
                + 2
                + 1
        );
    } else if let Err(e) = result {
        println!("{}", e)
    }
}
