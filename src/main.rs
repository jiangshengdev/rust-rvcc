use std::collections::LinkedList;
use std::env;
use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
enum TokenKind {
    Punctuation,
    Number,
    EndOfFile,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: String,
    _location: Range<usize>,
    _len: usize,
}

impl Token {
    fn new(kind: TokenKind, value: String, start: usize, end: usize) -> Token {
        Token {
            kind,
            value,
            _location: Range { start, end },
            _len: end - start,
        }
    }
}

fn tokenize(code: String) -> LinkedList<Token> {
    let mut positions = code.char_indices();
    let mut list = LinkedList::new();
    let mut current = positions.next();

    while let Some((i, c)) = current {
        if c.is_ascii_whitespace() {
            current = positions.next();
            continue;
        }

        if c.is_ascii_digit() {
            let mut number = String::new();
            let start = i;
            let mut end = i;

            while let Some((_i, c)) = current {
                if c.is_ascii_digit() {
                    end += 1;
                    number.push(c);

                    current = positions.next();
                    continue;
                } else {
                    break;
                }
            }

            let token = Token::new(TokenKind::Number, number, start, end);
            list.push_back(token);

            continue;
        }

        if c.is_ascii_punctuation() {
            let start = i;
            let end = i + 1;
            let token = Token::new(TokenKind::Punctuation, c.to_string(), start, end);
            list.push_back(token);

            current = positions.next();
            continue;
        }

        panic!("invalid token: {}", c);
    }

    let len = code.len();
    let token = Token::new(TokenKind::EndOfFile, String::new(), len, len);
    list.push_back(token);

    list
}

fn get_number(token: &Token) -> &String {
    if token.kind != TokenKind::Number {
        panic!("expect a number");
    }

    &token.value
}

fn main() {
    let mut args = env::args();
    let path = args.next().unwrap();
    let input = match args.next() {
        Some(s) => s,
        None => panic!("{}: invalid number of arguments", path),
    };

    let tokens = tokenize(input);

    println!("  .global main");
    println!("main:");

    let mut iter = tokens.iter();
    let mut current = iter.next();
    let token = current.unwrap();
    let number = get_number(token);
    println!("  li a0, {}", number);

    current = iter.next();

    while let Some(token) = current {
        if token.kind == TokenKind::EndOfFile {
            break;
        }

        if token.value == "+" {
            let token = iter.next().unwrap();
            println!("  addi a0, a0, {}", get_number(token));

            current = iter.next();
            continue;
        }

        if token.value == "-" {
            let token = iter.next().unwrap();
            println!("  addi a0, a0, -{}", get_number(token));

            current = iter.next();
            continue;
        }

        panic!("invalid token: {:?}", token)
    }

    println!("  ret");
}
