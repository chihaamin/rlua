use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Identifiers and literals
    Ident(String),
    StringLiteral(String),
    NumberLiteral(f64),

    // Keywords
    And,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Global,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // Type keywords
    Int,
    Long,
    Float,
    Double,
    String,
    Table,
    Bool,
    Char,
    Void,
    Vec,

    // Operators and punctuation
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
    Len,
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    Assign,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Colon,
    DoubleColon,
    Comma,
    Dot,
    Concat,
    Ellipsis,

    // Comments and whitespace (filtered out)
    Comment(String),

    // End of input
    Eof,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            match token {
                Token::Comment(_) => continue,
                _ => tokens.push(token),
            }
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let c = self.input.next()?;

        match c {
            // Single-character tokens
            '+' => Some(Token::Plus),
            '-' => {
                if self.match_char('-') {
                    self.read_comment()
                } else {
                    Some(Token::Minus)
                }
            }
            '*' => Some(Token::Mul),
            '/' => Some(Token::Div),
            '%' => Some(Token::Mod),
            '^' => Some(Token::Pow),
            '#' => Some(Token::Len),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            ';' => Some(Token::Semicolon),
            ',' => Some(Token::Comma),
            '.' => {
                if self.match_char('.') {
                    if self.match_char('.') {
                        Some(Token::Ellipsis)
                    } else {
                        Some(Token::Concat)
                    }
                } else {
                    Some(Token::Dot)
                }
            }
            ':' => {
                if self.match_char(':') {
                    Some(Token::DoubleColon)
                } else {
                    Some(Token::Colon)
                }
            }
            '=' => {
                if self.match_char('=') {
                    Some(Token::Eq)
                } else {
                    Some(Token::Assign)
                }
            }
            '<' => {
                if self.match_char('=') {
                    Some(Token::Le)
                } else {
                    Some(Token::Lt)
                }
            }
            '>' => {
                if self.match_char('=') {
                    Some(Token::Ge)
                } else {
                    Some(Token::Gt)
                }
            }
            '~' => {
                if self.match_char('=') {
                    Some(Token::Ne)
                } else {
                    panic!("Unexpected character '~'");
                }
            }
            '"' | '\'' => self.read_string(c),

            // Identifiers and keywords
            c if c.is_alphabetic() || c == '_' => self.read_identifier(c),

            // Numbers
            c if c.is_ascii_digit() => self.read_number(c),

            _ => None,
        }
    }

    fn read_comment(&mut self) -> Option<Token> {
        if self.match_char('[') {
            self.read_long_comment()
        } else {
            self.read_line_comment()
        }
    }

    fn read_line_comment(&mut self) -> Option<Token> {
        let mut content = String::new();
        while let Some(&c) = self.input.peek() {
            if c == '\n' {
                break;
            }
            content.push(c);
            self.input.next();
        }
        Some(Token::Comment(content))
    }

    fn read_long_comment(&mut self) -> Option<Token> {
        let mut content = String::new();
        let mut level = 0;

        // Check for opening long bracket
        while self.match_char('=') {
            level += 1;
        }

        if !self.match_char('[') {
            // Fallback to line comment
            content.push('[');
            return Some(Token::Comment(content));
        }

        // Read until matching closing bracket
        while let Some(c) = self.input.next() {
            if c == ']' {
                let mut closing_level = 0;
                while self.match_char('=') && closing_level < level {
                    closing_level += 1;
                }
                if closing_level == level && self.match_char(']') {
                    break;
                } else {
                    content.push(']');
                    for _ in 0..closing_level {
                        content.push('=');
                    }
                }
            } else {
                content.push(c);
            }
        }

        Some(Token::Comment(content))
    }

    fn read_string(&mut self, quote: char) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.input.next() {
            match c {
                '\\' => {
                    if let Some(esc) = self.input.next() {
                        match esc {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            '\'' => string.push('\''),
                            _ => string.push(esc),
                        }
                    }
                }
                c if c == quote => break,
                _ => string.push(c),
            }
        }
        Some(Token::StringLiteral(string))
    }

    fn read_identifier(&mut self, first: char) -> Option<Token> {
        let mut ident = String::from(first);
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        match ident.as_str() {
            // Standard keywords
            "and" => Some(Token::And),
            "break" => Some(Token::Break),
            "do" => Some(Token::Do),
            "else" => Some(Token::Else),
            "elseif" => Some(Token::ElseIf),
            "end" => Some(Token::End),
            "false" => Some(Token::False),
            "for" => Some(Token::For),
            "function" => Some(Token::Function),
            "goto" => Some(Token::Goto),
            "if" => Some(Token::If),
            "in" => Some(Token::In),
            "local" => Some(Token::Local),
            "global" => Some(Token::Global),
            "nil" => Some(Token::Nil),
            "not" => Some(Token::Not),
            "or" => Some(Token::Or),
            "repeat" => Some(Token::Repeat),
            "return" => Some(Token::Return),
            "then" => Some(Token::Then),
            "true" => Some(Token::True),
            "until" => Some(Token::Until),
            "while" => Some(Token::While),

            // Type keywords
            "int" => Some(Token::Int),
            "long" => Some(Token::Long),
            "float" => Some(Token::Float),
            "double" => Some(Token::Double),
            "string" => Some(Token::String),
            "table" => Some(Token::Table),
            "bool" => Some(Token::Bool),
            "char" => Some(Token::Char),
            "void" => Some(Token::Void),
            "vec" => Some(Token::Vec),

            _ => Some(Token::Ident(ident)),
        }
    }

    fn read_number(&mut self, first: char) -> Option<Token> {
        let mut num_str = String::from(first);
        let mut has_dot = false;
        let mut has_exp = false;

        while let Some(&c) = self.input.peek() {
            match c {
                '.' if !has_dot && !has_exp => {
                    has_dot = true;
                    num_str.push('.');
                    self.input.next();
                }
                'e' | 'E' if !has_exp => {
                    has_exp = true;
                    num_str.push(c);
                    self.input.next();

                    if let Some(&sign) = self.input.peek() {
                        if sign == '+' || sign == '-' {
                            num_str.push(sign);
                            self.input.next();
                        }
                    }
                }
                '_' => {
                    self.input.next(); // Skip underscores
                }
                c if c.is_ascii_digit() => {
                    num_str.push(c);
                    self.input.next();
                }
                _ => break,
            }
        }

        num_str.parse::<f64>().map(Token::NumberLiteral).ok()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.input.peek() == Some(&expected) {
            self.input.next();
            true
        } else {
            false
        }
    }
}
