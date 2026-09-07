pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number,
    Ident,
    Keyword,
    Comment,
    String,
    Char,
    TextBlock,

    // 算术
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,

    // 比较
    Eq,
    EqEq,
    Ne,
    Not,
    Lt,
    Gt,
    Le,
    Ge,

    // 逻辑 / 位运算
    And,
    AndAnd,
    AndEq,
    Or,
    OrOr,
    OrEq,
    Xor,
    XorEq,
    Tilde,

    // 移位
    Shl,
    Shr,
    Ursh,
    ShlEq,
    ShrEq,
    UrshEq,

    // 其他符号
    Arrow,
    Question,
    Colon,
    ColonColon,
    At,

    // 分隔符
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Dot,
    Ellipsis,

    Eof,
    Error,
}

#[allow(clippy::match_like_matches_macro)]
fn is_keyword(text: &str) -> bool {
    match text {
        "package" | "import" | "class" | "interface" | "enum" | "extends" | "implements"
        | "throws" | "throw" | "new" | "public" | "protected" | "private" | "static" | "final"
        | "abstract" | "synchronized" | "native" | "transient" | "volatile" | "strictfp"
        | "void" | "int" | "boolean" | "char" | "byte" | "short" | "long" | "float" | "double"
        | "if" | "else" | "while" | "do" | "for" | "switch" | "case" | "default" | "break"
        | "continue" | "return" | "yield" | "try" | "catch" | "finally" | "assert" | "instanceof"
        | "this" | "super" | "null" | "true" | "false" => true,
        _ => false,
    }
}

impl Lexer {
    // 返回当前字节，已到末尾则返回 None
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    // 返回当前 +1 的字符
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn peek_next2(&self) -> Option<char> {
        self.chars.get(self.pos + 2).copied()
    }

    // 返回当前字符，并移动
    fn bump(&mut self) -> Option<char> {
        let c: Option<char> = self.peek();
        if let Some(c) = c {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        c
    }

    pub fn new(source: &str) -> Lexer {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn is_at_closing_delimiter(&self, delimiter_len: usize) -> bool {
        match delimiter_len {
            1 => self.peek() == Some('"'),
            3 => {
                self.peek() == Some('"')
                    && self.peek_next() == Some('"')
                    && self.peek_next2() == Some('"')
            }
            _ => false,
        }
    }

    fn read_quoted(&mut self, kind: TokenKind, delimiter_len: usize) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut result: String = String::new();

        for _ in 0..delimiter_len {
            self.bump();
        }

        loop {
            // 结束分隔符
            if self.is_at_closing_delimiter(delimiter_len) {
                for _ in 0..delimiter_len {
                    self.bump();
                }

                return Token {
                    kind,
                    text: result,
                    line: start_line,
                    column: start_column,
                };
            }

            match self.peek() {
                // 转义
                Some('\\') => {
                    if self.peek_next() == Some('u') {
                        self.bump();
                        while self.peek() == Some('u') {
                            self.bump();
                        }
                        let hex_start = self.pos;
                        let hex_end = (self.pos + 4).min(self.chars.len());
                        let hex: String = self.chars[hex_start..hex_end].iter().collect();
                        let code = u32::from_str_radix(&hex, 16).unwrap_or(0);
                        let ch = char::from_u32(code).unwrap_or('\u{FFFD}');
                        result.push(ch);
                        for _ in 0..(hex_end - hex_start) {
                            self.bump();
                        }
                    } else {
                        self.bump();
                        if let Some(c) = self.peek() {
                            result.push(c);
                            self.bump();
                        }
                    }
                }

                // 普通字符
                Some(c) => {
                    result.push(c);
                    self.bump();
                }

                // EOF
                None => {
                    return Token {
                        kind: TokenKind::Error,
                        text: String::new(),
                        line: start_line,
                        column: start_column,
                    };
                }
            }
        }
    }

    fn read_char(&mut self) -> Token {
        let start = self.pos;
        let start_line = self.line;
        let start_column = self.column;
        self.bump();

        loop {
            match self.peek() {
                Some('\'') => {
                    self.bump();
                    let text: String = self.chars[start..self.pos].iter().collect();
                    return Token {
                        kind: TokenKind::Char,
                        text,
                        line: start_line,
                        column: start_column,
                    };
                }

                Some('\\') => {
                    self.bump();
                    if self.peek().is_some() {
                        self.bump();
                    }
                }

                Some(_) => {
                    self.bump();
                }

                None => {
                    return Token {
                        kind: TokenKind::Error,
                        text: String::new(),
                        line: start_line,
                        column: start_column,
                    };
                }
            }
        }
    }

    // fn read_blocking(&mut self) -> Token {
    //     let start:usize = self.pos;
    //     self.bump();
    //     self.bump();
    //     self.bump();
    //     loop {
    //         match self.peek() {
    //             Some('\"') => {
    //                 if self.peek_next() == Some('\"') && self.peek_next2() == Some('\"') {
    //                     self.bump();
    //                     self.bump();
    //                     self.bump();
    //                     break;
    //                 }
    //             }
    //             Some(_) => {
    //                 self.bump();
    //             }
    //             None => {
    //                 return Token {kind:TokenKind::Eof,text: String::new(),}
    //             }
    //         }
    //     }
    //     let text: String = self.chars[start..self.pos].iter().collect();
    //     return Token {kind:TokenKind::TextBlock, text};
    // }

    fn read_string(&mut self) -> Token {
        self.read_quoted(TokenKind::String, 1)
    }

    fn read_blocking(&mut self) -> Token {
        self.read_quoted(TokenKind::TextBlock, 3)
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start = self.pos;
        let start_line = self.line;
        let start_column = self.column;

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '$' {
                self.bump();
            } else {
                break;
            }
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        let kind = if is_keyword(&text) {
            TokenKind::Keyword
        } else {
            TokenKind::Ident
        };
        Token {
            kind,
            text,
            line: start_line,
            column: start_column,
        }
    }

    fn read_comment_block(&mut self) -> Token {
        let start = self.pos;
        let start_line = self.line;
        let start_column = self.column;
        self.bump();
        self.bump();

        loop {
            let c = self.peek();
            let next = self.peek_next();

            match (c, next) {
                (Some('*'), Some('/')) => {
                    self.bump(); // *
                    self.bump(); // /
                    break;
                }
                (Some(_), _) => {
                    self.bump();
                }
                (None, _) => {
                    break;
                }
            }
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        Token {
            kind: TokenKind::Comment,
            text,
            line: start_line,
            column: start_column,
        }
    }

    fn read_comment_line(&mut self) -> Token {
        let start: usize = self.pos;
        let start_line = self.line;
        let start_column = self.column;
        self.bump();
        self.bump();

        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.bump();
        }

        let text: String = self.chars[start..self.pos].iter().collect();
        Token {
            kind: TokenKind::Comment,
            text,
            line: start_line,
            column: start_column,
        }
    }

    fn read_operator(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let c = self.bump().unwrap();

        match c {
            '+' => match self.peek() {
                Some('+') => {
                    self.bump();
                    Token {
                        kind: TokenKind::PlusPlus,
                        text: "++".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::PlusEq,
                        text: "+=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Plus,
                    text: "+".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '-' => match self.peek() {
                Some('-') => {
                    self.bump();
                    Token {
                        kind: TokenKind::MinusMinus,
                        text: "--".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::MinusEq,
                        text: "-=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                Some('>') => {
                    self.bump();
                    Token {
                        kind: TokenKind::Arrow,
                        text: "->".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Minus,
                    text: "-".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '*' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::StarEq,
                        text: "*=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Star,
                    text: "*".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '/' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::SlashEq,
                        text: "/=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Slash,
                    text: "/".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '%' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::PercentEq,
                        text: "%=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Percent,
                    text: "%".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '=' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::EqEq,
                        text: "==".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Eq,
                    text: "=".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '!' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::Ne,
                        text: "!=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Not,
                    text: "!".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '<' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::Le,
                        text: "<=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                Some('<') => {
                    self.bump();
                    match self.peek() {
                        Some('=') => {
                            self.bump();
                            Token {
                                kind: TokenKind::ShlEq,
                                text: "<<=".to_string(),
                                line: start_line,
                                column: start_column,
                            }
                        }
                        _ => Token {
                            kind: TokenKind::Shl,
                            text: "<<".to_string(),
                            line: start_line,
                            column: start_column,
                        },
                    }
                }
                _ => Token {
                    kind: TokenKind::Lt,
                    text: "<".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '>' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::Ge,
                        text: ">=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                Some('>') => {
                    self.bump();
                    match self.peek() {
                        Some('=') => {
                            self.bump();
                            Token {
                                kind: TokenKind::ShrEq,
                                text: ">>=".to_string(),
                                line: start_line,
                                column: start_column,
                            }
                        }
                        Some('>') => {
                            self.bump();
                            match self.peek() {
                                Some('=') => {
                                    self.bump();
                                    Token {
                                        kind: TokenKind::UrshEq,
                                        text: ">>>=".to_string(),
                                        line: start_line,
                                        column: start_column,
                                    }
                                }
                                _ => Token {
                                    kind: TokenKind::Ursh,
                                    text: ">>>".to_string(),
                                    line: start_line,
                                    column: start_column,
                                },
                            }
                        }
                        _ => Token {
                            kind: TokenKind::Shr,
                            text: ">>".to_string(),
                            line: start_line,
                            column: start_column,
                        },
                    }
                }
                _ => Token {
                    kind: TokenKind::Gt,
                    text: ">".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '&' => match self.peek() {
                Some('&') => {
                    self.bump();
                    Token {
                        kind: TokenKind::AndAnd,
                        text: "&&".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::AndEq,
                        text: "&=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::And,
                    text: "&".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '|' => match self.peek() {
                Some('|') => {
                    self.bump();
                    Token {
                        kind: TokenKind::OrOr,
                        text: "||".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::OrEq,
                        text: "|=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Or,
                    text: "|".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '^' => match self.peek() {
                Some('=') => {
                    self.bump();
                    Token {
                        kind: TokenKind::XorEq,
                        text: "^=".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
                _ => Token {
                    kind: TokenKind::Xor,
                    text: "^".to_string(),
                    line: start_line,
                    column: start_column,
                },
            },

            '~' => Token {
                kind: TokenKind::Tilde,
                text: "~".to_string(),
                line: start_line,
                column: start_column,
            },
            '?' => Token {
                kind: TokenKind::Question,
                text: "?".to_string(),
                line: start_line,
                column: start_column,
            },
            ':' => {
                if self.peek() == Some(':') {
                    self.bump();
                    Token {
                        kind: TokenKind::ColonColon,
                        text: "::".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                } else {
                    Token {
                        kind: TokenKind::Colon,
                        text: ":".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
            }

            '.' => {
                if self.peek() == Some('.') && self.peek_next() == Some('.') {
                    // 可能有问题，需要再看看
                    self.bump();
                    self.bump();
                    Token {
                        kind: TokenKind::Ellipsis,
                        text: "...".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                } else {
                    Token {
                        kind: TokenKind::Dot,
                        text: ".".to_string(),
                        line: start_line,
                        column: start_column,
                    }
                }
            }

            ',' => Token {
                kind: TokenKind::Comma,
                text: ",".to_string(),
                line: start_line,
                column: start_column,
            },
            ';' => Token {
                kind: TokenKind::Semicolon,
                text: ";".to_string(),
                line: start_line,
                column: start_column,
            },

            '(' => Token {
                kind: TokenKind::LParen,
                text: "(".to_string(),
                line: start_line,
                column: start_column,
            },
            ')' => Token {
                kind: TokenKind::RParen,
                text: ")".to_string(),
                line: start_line,
                column: start_column,
            },
            '{' => Token {
                kind: TokenKind::LBrace,
                text: "{".to_string(),
                line: start_line,
                column: start_column,
            },
            '}' => Token {
                kind: TokenKind::RBrace,
                text: "}".to_string(),
                line: start_line,
                column: start_column,
            },
            '[' => Token {
                kind: TokenKind::LBracket,
                text: "[".to_string(),
                line: start_line,
                column: start_column,
            },
            ']' => Token {
                kind: TokenKind::RBracket,
                text: "]".to_string(),
                line: start_line,
                column: start_column,
            },

            '@' => Token {
                kind: TokenKind::At,
                text: "@".to_string(),
                line: start_line,
                column: start_column,
            },

            _ => Token {
                kind: TokenKind::Error,
                text: c.to_string(),
                line: start_line,
                column: start_column,
            },
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        let start_line = self.line;
        let start_column = self.column;

        if self.peek() == Some('0') {
            if let Some(n) = self.peek_next() {
                if n == 'x' || n == 'X' {
                    self.bump();
                    self.bump();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_hexdigit() || c == '_' {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                    if self.peek() == Some('.') {
                        self.bump();
                        while let Some(c) = self.peek() {
                            if c.is_ascii_hexdigit() || c == '_' {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    if let Some(c) = self.peek() {
                        if c == 'p' || c == 'P' {
                            self.bump();
                            if self.peek() == Some('+') || self.peek() == Some('-') {
                                self.bump();
                            }
                            while let Some(c) = self.peek() {
                                if c.is_ascii_digit() || c == '_' {
                                    self.bump();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    if let Some(c) = self.peek() {
                        if c == 'l' || c == 'L' || c == 'f' || c == 'F' || c == 'd' || c == 'D' {
                            self.bump();
                        }
                    }
                    let text: String = self.chars[start..self.pos].iter().collect();
                    return Token {
                        kind: TokenKind::Number,
                        text,
                        line: start_line,
                        column: start_column,
                    };
                } else if n == 'b' || n == 'B' {
                    self.bump();
                    self.bump();
                    while let Some(c) = self.peek() {
                        if c == '0' || c == '1' || c == '_' {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                    if let Some(c) = self.peek() {
                        if c == 'l' || c == 'L' {
                            self.bump();
                        }
                    }
                    let text: String = self.chars[start..self.pos].iter().collect();
                    return Token {
                        kind: TokenKind::Number,
                        text,
                        line: start_line,
                        column: start_column,
                    };
                }
            }
        }

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                self.bump();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') && self.peek_next() != Some('.') {
            self.bump();
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() || c == '_' {
                    self.bump();
                } else {
                    break;
                }
            }
        }

        if let Some(c) = self.peek() {
            if c == 'e' || c == 'E' {
                self.bump();
                if self.peek() == Some('+') || self.peek() == Some('-') {
                    self.bump();
                }
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
        }

        if let Some(c) = self.peek() {
            if c == 'l' || c == 'L' || c == 'f' || c == 'F' || c == 'd' || c == 'D' {
                self.bump();
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();
        Token {
            kind: TokenKind::Number,
            text,
            line: start_line,
            column: start_column,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start_line = self.line;
        let start_column = self.column;
        let Some(c) = self.peek() else {
            return Token {
                kind: TokenKind::Eof,
                text: String::new(),
                line: start_line,
                column: start_column,
            };
        };

        if c == '\'' {
            return self.read_char();
        }

        if c == '"' {
            return self.read_string();
        }

        if c == '\"' && self.peek_next() == Some('\"') && self.peek_next2() == Some('\"') {
            return self.read_blocking();
        }
        if c == '/' && self.peek_next() == Some('/') {
            return self.read_comment_line();
        }
        if c == '/' && self.peek_next() == Some('*') {
            return self.read_comment_block();
        }
        if c == '.' && self.peek_next().is_some_and(|d| d.is_ascii_digit()) {
            return self.read_number();
        }
        if c.is_ascii_digit() {
            return self.read_number();
        }
        if c.is_alphabetic() || c == '_' || c == '$' {
            return self.read_identifier_or_keyword();
        }
        self.read_operator()
    }
}
