use std::cell::RefCell;
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
    location: Range<usize>,
    _len: usize,
}

thread_local! {
    static CURRENT_INPUT: RefCell<String> = RefCell::new(String::new())
}

#[macro_export]
macro_rules! error_at {
    ($location: ident, $($arg:tt)*) => {
        $crate::error_at($location, ($location+1), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error_token {
    ($token: ident, $($arg:tt)*) => {
        $crate::error_token($token, format!($($arg)*));
    };
}

fn error_token(token: &Token, message: String) {
    error_at(token.location.start, token.location.end, message);
}

fn error_at(start: usize, end: usize, message: String) {
    let mut current_input = String::new();
    CURRENT_INPUT.with(|text| current_input = text.borrow().to_string());
    println!("{}", current_input);

    let padding_left = (0..start).map(|_| " ").collect::<String>();
    let error_pointer = (start..end).map(|_| "^").collect::<String>();
    println!("{}{} {}", padding_left, error_pointer, message);

    panic!();
}

impl Token {
    fn new(kind: TokenKind, value: String, start: usize, end: usize) -> Token {
        Token {
            kind,
            value,
            location: Range { start, end },
            _len: end - start,
        }
    }
}

fn tokenize(code: String) -> LinkedList<Token> {
    CURRENT_INPUT.with(|text| {
        *text.borrow_mut() = code.clone();
    });
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
                    number.push(c);
                    end += 1;

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

        error_at!(i, "invalid character: {}", c);
    }

    let len = code.len();
    let token = Token::new(TokenKind::EndOfFile, String::new(), len, len);
    list.push_back(token);

    list
}

fn get_number(token: &Token) -> &String {
    if token.kind != TokenKind::Number {
        error_token!(token, "expect a number");
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

        error_token!(token, "invalid token: {:?}", token);
    }

    println!("  ret");
}
