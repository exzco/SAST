use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Public,
    Protected,
    Private,
    Static,
    Final,
    Abstract,
    Synchronized,
    Native,
    Transient,
    Volatile,
    Strictfp,
    Default,
    Annotation {
        name: String,
        args: Option<Vec<String>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    Boolean,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Reference {
        name: String,
        type_args: Vec<Type>,
    },
    Array {
        element_type: Box<Type>,
        dimensions: usize,
    },
    Var,
    Wildcard {
        bound: Option<Box<Type>>,
        is_extends: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    pub package: Option<String>,
    pub imports: Vec<String>,
    pub classes: Vec<ClassDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclarator {
    pub name: String,
    pub init: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub labels: Vec<Expr>,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub modifiers: Vec<Modifier>,
    pub types: Vec<Type>,
    pub var_name: String,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Field(FieldDecl),
    Method(MethodDecl),
    Constructor(ConstructorDecl),
    Class(ClassDecl),
    StaticInit(Box<Stmt>),
    InstanceInit(Box<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub modifiers: Vec<Modifier>,
    pub var_type: Type,
    pub declarators: Vec<VarDeclarator>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDecl {
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: String,
    pub params: Vec<MethodParam>,
    pub throws: Vec<String>,
    pub body: Option<Box<Stmt>>,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorDecl {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub params: Vec<MethodParam>,
    pub throws: Vec<String>,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodParam {
    pub modifiers: Vec<Modifier>,
    pub var_type: Type,
    pub name: String,
    pub is_varargs: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum Stmt {
    Empty,
    Block {
        statements: Vec<Stmt>,
    },
    ExprStmt {
        expr: Expr,
    },
    VarDecl {
        modifiers: Vec<Modifier>,
        var_type: Type,
        declarators: Vec<VarDeclarator>,
    },
    LocalClassDecl(ClassDecl),
    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    DoWhile {
        body: Box<Stmt>,
        condition: Expr,
    },
    For {
        init: Vec<Stmt>,
        condition: Option<Expr>,
        update: Vec<Expr>,
        body: Box<Stmt>,
    },
    ForEach {
        modifiers: Vec<Modifier>,
        var_type: Type,
        var_name: String,
        iterable: Expr,
        body: Box<Stmt>,
    },
    Switch {
        selector: Expr,
        cases: Vec<SwitchCase>,
    },
    Yield {
        value: Expr,
    },
    Return {
        value: Option<Expr>,
    },
    Break {
        label: Option<String>,
    },
    Continue {
        label: Option<String>,
    },
    Throw {
        expr: Expr,
    },
    Synchronized {
        lock: Expr,
        body: Box<Stmt>,
    },
    Try {
        resources: Vec<Stmt>,
        body: Box<Stmt>,
        catches: Vec<CatchClause>,
        finally_body: Option<Box<Stmt>>,
    },
    Labeled {
        label: String,
        body: Box<Stmt>,
    },
    Assert {
        condition: Expr,
        message: Option<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntLiteral(i64),
    StringLiteral(String),
    CharLiteral(String),
    BoolLiteral(bool),
    NullLiteral,
    Ident(String),
    This,
    Super,
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    Ternary {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    MemberAccess {
        object: Box<Expr>,
        member: String,
    },
    ArrayAccess {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    NewInstance {
        class_type: Type,
        args: Vec<Expr>,
    },
    NewArray {
        elem_type: Type,
        dimensions: Vec<Expr>,
    },
    NewArrayInit {
        elem_type: Type,
        elements: Vec<Expr>,
    },
    ArrayInit(Vec<Expr>),
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Cast {
        target_type: Type,
        expr: Box<Expr>,
    },
    InstanceOf {
        expr: Box<Expr>,
        target_type: Type,
        pattern_var: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Percent,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    AndAnd,
    OrOr,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Ursh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
    PlusPlus,
    MinusMinus,
}

pub fn token_kind_to_bin_op(kind: TokenKind) -> Option<BinOp> {
    match kind {
        TokenKind::Plus => Some(BinOp::Add),
        TokenKind::Minus => Some(BinOp::Sub),
        TokenKind::Star => Some(BinOp::Mul),
        TokenKind::Slash => Some(BinOp::Div),
        TokenKind::Percent => Some(BinOp::Percent),

        TokenKind::EqEq => Some(BinOp::Eq),
        TokenKind::Ne => Some(BinOp::Ne),
        TokenKind::Lt => Some(BinOp::Lt),
        TokenKind::Gt => Some(BinOp::Gt),
        TokenKind::Le => Some(BinOp::Le),
        TokenKind::Ge => Some(BinOp::Ge),

        TokenKind::AndAnd => Some(BinOp::AndAnd),
        TokenKind::OrOr => Some(BinOp::OrOr),

        TokenKind::And => Some(BinOp::And),
        TokenKind::Or => Some(BinOp::Or),
        TokenKind::Xor => Some(BinOp::Xor),
        TokenKind::Shl => Some(BinOp::Shl),
        TokenKind::Shr => Some(BinOp::Shr),
        TokenKind::Ursh => Some(BinOp::Ursh),

        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        static EOF_TOKEN: Token = Token {
            kind: TokenKind::Eof,
            text: String::new(),
            line: 0,
            column: 0,
        };
        self.tokens.get(self.pos).unwrap_or(&EOF_TOKEN)
    }

    fn peek_offset(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    fn next(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            t
        } else {
            Token {
                kind: TokenKind::Eof,
                text: String::new(),
                line: 0,
                column: 0,
            }
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, String> {
        if self.peek().kind == kind {
            Ok(self.next())
        } else {
            Err(format!("期望 {:?}，当前 {:?}", kind, self.peek()))
        }
    }

    fn expect_keyword(&mut self, kw: &str) -> Result<Token, String> {
        let tok = self.peek();
        if tok.kind == TokenKind::Keyword && tok.text == kw {
            Ok(self.next())
        } else {
            Err(format!("期望关键字 '{}'，实际遇到 {:?}", kw, tok))
        }
    }

    fn expect_gt(&mut self) -> Result<(), String> {
        if self.peek().kind == TokenKind::Gt {
            self.next();
            Ok(())
        } else if self.peek().kind == TokenKind::Shr {
            if self.pos < self.tokens.len() {
                self.tokens[self.pos].kind = TokenKind::Gt;
                self.tokens[self.pos].text = ">".to_string();
            }
            Ok(())
        } else if self.peek().kind == TokenKind::Ursh {
            if self.pos < self.tokens.len() {
                self.tokens[self.pos].kind = TokenKind::Shr;
                self.tokens[self.pos].text = ">>".to_string();
            }
            Ok(())
        } else {
            Err(format!("期望 Gt，当前 {:?}", self.peek()))
        }
    }

    fn skip_annotations_at(&self, mut offset: usize) -> usize {
        while self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::At) {
            offset += 1;
            if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Ident) {
                offset += 1;
                while self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Dot)
                    && self.peek_offset(offset + 1).is_some_and(|t| t.kind == TokenKind::Ident)
                {
                    offset += 2;
                }
            }
            if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::LParen) {
                let mut depth = 1;
                offset += 1;
                while depth > 0 && self.peek_offset(offset).is_some() {
                    if self.peek_offset(offset).unwrap().kind == TokenKind::LParen {
                        depth += 1;
                    } else if self.peek_offset(offset).unwrap().kind == TokenKind::RParen {
                        depth -= 1;
                    }
                    offset += 1;
                }
            }
        }
        offset
    }

    fn is_package_start(&self) -> bool {
        let offset = self.skip_annotations_at(0);
        self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Keyword && t.text == "package")
    }

    pub fn parse_compile_unit(&mut self) -> Result<CompilationUnit, String> {
        if self.peek().text == "module" || (self.peek().text == "open" && self.peek_offset(1).is_some_and(|t| t.text == "module")) {
            let _ = self.parse_modifiers();
            if self.peek().text == "open" { self.next(); }
            if self.peek().text == "module" { self.next(); }
            let mod_name = self.parse_qualified_name()?;
            self.expect(TokenKind::LBrace)?;
            while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
                self.next();
            }
            self.expect(TokenKind::RBrace)?;
            return Ok(CompilationUnit {
                package: None,
                imports: vec![],
                classes: vec![ClassDecl {
                    modifiers: vec![],
                    name: mod_name,
                    super_class: None,
                    interfaces: vec![],
                    members: vec![],
                }],
            });
        }

        let package = if self.is_package_start() {
            let _pkg_mods = self.parse_modifiers()?;
            Some(self.parse_package()?)
        } else {
            None
        };

        let mut imports = Vec::new();
        while self.peek().kind == TokenKind::Keyword && self.peek().text == "import" {
            imports.push(self.parse_import()?);
        }

        let mut classes = Vec::new();
        while self.peek().kind != TokenKind::Eof {
            if self.peek().kind == TokenKind::Semicolon {
                self.next();
                continue;
            }
            classes.push(self.parse_class_decl()?);
        }

        Ok(CompilationUnit {
            package,
            imports,
            classes,
        })
    }

    fn parse_package(&mut self) -> Result<String, String> {
        self.expect_keyword("package")?;
        let name = self.parse_qualified_name()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(name)
    }

    fn parse_import(&mut self) -> Result<String, String> {
        self.expect_keyword("import")?;
        if self.peek().kind == TokenKind::Keyword && self.peek().text == "static" {
            self.next();
        }
        let name = self.parse_qualified_name()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(name)
    }

    fn parse_qualified_name(&mut self) -> Result<String, String> {
        let mut name = String::new();
        loop {
            if self.peek().kind == TokenKind::Star {
                self.next();
                name.push('*');
            } else {
                let part = self.expect(TokenKind::Ident)?.text;
                name.push_str(&part);
            }

            if self.peek().kind == TokenKind::Dot {
                self.next();
                name.push('.');
            } else {
                break;
            }
        }
        Ok(name)
    }

    fn expect_qualified_ident(&mut self) -> Result<String, String> {
        let mut name = self.expect(TokenKind::Ident)?.text;
        while self.peek().kind == TokenKind::Dot && self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::Ident) {
            self.next();
            let part = self.expect(TokenKind::Ident)?.text;
            name.push('.');
            name.push_str(&part);
        }
        Ok(name)
    }

    fn skip_type_params(&mut self) -> Result<(), String> {
        if self.peek().kind == TokenKind::Lt {
            self.next();
            let mut depth = 1;
            while depth > 0 && self.peek().kind != TokenKind::Eof {
                if self.peek().kind == TokenKind::Lt {
                    depth += 1;
                    self.next();
                } else if self.peek().kind == TokenKind::Gt {
                    depth -= 1;
                    self.next();
                } else if self.peek().kind == TokenKind::Shr {
                    if depth >= 2 {
                        depth -= 2;
                        self.next();
                    } else {
                        depth -= 1;
                        self.tokens[self.pos].kind = TokenKind::Gt;
                        self.tokens[self.pos].text = ">".to_string();
                    }
                } else if self.peek().kind == TokenKind::Ursh {
                    if depth >= 3 {
                        depth -= 3;
                        self.next();
                    } else {
                        depth -= 1;
                        self.tokens[self.pos].kind = TokenKind::Shr;
                        self.tokens[self.pos].text = ">>".to_string();
                    }
                } else {
                    self.next();
                }
            }
        }
        Ok(())
    }

    fn parse_class_decl(&mut self) -> Result<ClassDecl, String> {
        let modifiers = self.parse_modifiers()?;
        self.parse_class_decl_with_modifiers(modifiers)
    }

    fn parse_class_decl_with_modifiers(&mut self, modifiers: Vec<Modifier>) -> Result<ClassDecl, String> {
        let is_at_interface = self.peek().kind == TokenKind::At
            && self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::Keyword && t.text == "interface");

        let type_kind = if is_at_interface {
            self.next(); // '@'
            self.next(); // 'interface'
            "annotation".to_string()
        } else if self.peek().kind == TokenKind::Keyword && matches!(self.peek().text.as_str(), "class" | "interface" | "enum") {
            self.next().text
        } else if self.peek().kind == TokenKind::Ident && self.peek().text == "record" {
            self.next().text
        } else {
            return Err(format!("期望类声明关键字，实际遇到 {:?}", self.peek()));
        };

        let name = self.expect(TokenKind::Ident)?.text;

        if self.peek().kind == TokenKind::Lt {
            self.skip_type_params()?;
        }

        if type_kind == "record" && self.peek().kind == TokenKind::LParen {
            self.next();
            while self.peek().kind != TokenKind::RParen && self.peek().kind != TokenKind::Eof {
                self.next();
            }
            self.expect(TokenKind::RParen)?;
        }

        let mut super_class = None;
        let mut interfaces = Vec::new();

        if self.peek().kind == TokenKind::Keyword && self.peek().text == "extends" {
            self.next();
            if type_kind == "interface" {
                loop {
                    interfaces.push(self.parse_type()?);
                    if self.peek().kind == TokenKind::Comma {
                        self.next();
                    } else {
                        break;
                    }
                }
            } else {
                super_class = Some(self.parse_type()?);
            }
        }

        if self.peek().kind == TokenKind::Keyword && self.peek().text == "implements" {
            self.next();
            loop {
                interfaces.push(self.parse_type()?);
                if self.peek().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenKind::LBrace)?;
        let mut members = Vec::new();

        if type_kind == "enum" {
            while self.peek().kind != TokenKind::Semicolon && self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
                let _ = self.parse_modifiers();
                if self.peek().kind == TokenKind::Ident {
                    let _ = self.next();
                    if self.peek().kind == TokenKind::LParen {
                        self.next();
                        let mut paren_depth = 1;
                        while paren_depth > 0 && self.peek().kind != TokenKind::Eof {
                            if self.peek().kind == TokenKind::LParen {
                                paren_depth += 1;
                            } else if self.peek().kind == TokenKind::RParen {
                                paren_depth -= 1;
                            }
                            self.next();
                        }
                    }
                    if self.peek().kind == TokenKind::LBrace {
                        self.next();
                        while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
                            if self.peek().kind == TokenKind::Semicolon {
                                self.next();
                                continue;
                            }
                            let _ = self.parse_class_member();
                        }
                        self.expect(TokenKind::RBrace)?;
                    }
                }
                if self.peek().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
            if self.peek().kind == TokenKind::Semicolon {
                self.next();
            }
        }

        while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
            if self.peek().kind == TokenKind::Semicolon {
                self.next();
                continue;
            }
            members.push(self.parse_class_member()?);
        }
        self.expect(TokenKind::RBrace)?;

        Ok(ClassDecl {
            modifiers,
            name,
            super_class: super_class.map(|t| format!("{:?}", t)),
            interfaces: interfaces.into_iter().map(|t| format!("{:?}", t)).collect(),
            members,
        })
    }

    fn parse_class_member(&mut self) -> Result<ClassMember, String> {
        let modifiers = self.parse_modifiers()?;

        if modifiers.contains(&Modifier::Static) && self.peek().kind == TokenKind::LBrace {
            let body = Box::new(self.parse_block()?);
            return Ok(ClassMember::StaticInit(body));
        }

        if self.peek().kind == TokenKind::LBrace {
            let body = Box::new(self.parse_block()?);
            return Ok(ClassMember::InstanceInit(body));
        }

        let is_inner_at_interface = self.peek().kind == TokenKind::At
            && self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::Keyword && t.text == "interface");
        if is_inner_at_interface
            || (self.peek().kind == TokenKind::Keyword && matches!(self.peek().text.as_str(), "class" | "interface" | "enum"))
            || (self.peek().kind == TokenKind::Ident && self.peek().text == "record")
        {
            let class_decl = self.parse_class_decl_with_modifiers(modifiers)?;
            return Ok(ClassMember::Class(class_decl));
        }

        if self.peek().kind == TokenKind::Lt {
            self.skip_type_params()?;
        }

        let first_type = self.parse_type()?;

        if self.peek().kind == TokenKind::LParen {
            let name = match &first_type {
                Type::Reference { name, .. } => name.clone(),
                Type::Primitive(p) => format!("{:?}", p),
                _ => "Unknown".to_string(),
            };
            return self.parse_constructor_tail(modifiers, name);
        }

        let name = if self.peek().kind == TokenKind::Ident || self.peek().kind == TokenKind::Keyword {
            self.next().text
        } else {
            return Err(format!("期望类成员标识符，当前 {:?}", self.peek()));
        };

        if self.peek().kind == TokenKind::LParen {
            self.parse_method_decl_tail(modifiers, first_type, name)
        } else {
            self.parse_field_decl_tail(modifiers, first_type, name)
        }
    }

    fn parse_constructor_tail(&mut self, modifiers: Vec<Modifier>, name: String) -> Result<ClassMember, String> {
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                let p_mods = self.parse_modifiers()?;
                let mut p_type = self.parse_type()?;
                let mut is_varargs = false;
                while self.peek().kind == TokenKind::At {
                    self.next();
                    let _ = self.expect_qualified_ident()?;
                    if self.peek().kind == TokenKind::LParen {
                        self.next();
                        let mut depth = 1;
                        while depth > 0 && self.peek().kind != TokenKind::Eof {
                            if self.peek().kind == TokenKind::LParen { depth += 1; }
                            else if self.peek().kind == TokenKind::RParen { depth -= 1; }
                            self.next();
                        }
                    }
                }
                if self.peek().kind == TokenKind::Ellipsis {
                    self.next();
                    is_varargs = true;
                    p_type = Type::Array {
                        element_type: Box::new(p_type),
                        dimensions: 1,
                    };
                }
                let p_name = self.expect(TokenKind::Ident)?.text;
                params.push(MethodParam {
                    modifiers: p_mods,
                    var_type: p_type,
                    name: p_name,
                    is_varargs,
                });
                if self.peek().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen)?;

        let mut throws = Vec::new();
        if self.peek().kind == TokenKind::Keyword && self.peek().text == "throws" {
            self.next();
            loop {
                throws.push(self.expect_qualified_ident()?);
                if self.peek().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }

        let body = Box::new(self.parse_block()?);

        Ok(ClassMember::Constructor(ConstructorDecl {
            modifiers,
            name,
            params,
            throws,
            body,
        }))
    }

    fn parse_method_decl_tail(
        &mut self,
        modifiers: Vec<Modifier>,
        return_type: Type,
        name: String,
    ) -> Result<ClassMember, String> {
        self.expect(TokenKind::LParen)?;

        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                let p_mods = self.parse_modifiers()?;
                let mut p_type = self.parse_type()?;
                let mut is_varargs = false;
                while self.peek().kind == TokenKind::At {
                    self.next();
                    let _ = self.expect_qualified_ident()?;
                    if self.peek().kind == TokenKind::LParen {
                        self.next();
                        let mut depth = 1;
                        while depth > 0 && self.peek().kind != TokenKind::Eof {
                            if self.peek().kind == TokenKind::LParen { depth += 1; }
                            else if self.peek().kind == TokenKind::RParen { depth -= 1; }
                            self.next();
                        }
                    }
                }
                if self.peek().kind == TokenKind::Ellipsis {
                    self.next();
                    is_varargs = true;
                    p_type = Type::Array {
                        element_type: Box::new(p_type),
                        dimensions: 1,
                    };
                }
                let p_name = self.expect(TokenKind::Ident)?.text;
                params.push(MethodParam {
                    modifiers: p_mods,
                    var_type: p_type,
                    name: p_name,
                    is_varargs,
                });

                if self.peek().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen)?;

        let mut default_value = None;
        if self.peek().kind == TokenKind::Keyword && self.peek().text == "default" {
            self.next();
            default_value = Some(self.parse_expression()?);
        }

        let mut throws = Vec::new();
        if self.peek().kind == TokenKind::Keyword && self.peek().text == "throws" {
            self.next();
            loop {
                throws.push(self.expect_qualified_ident()?);
                if self.peek().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }

        let body = if self.peek().kind == TokenKind::LBrace {
            Some(Box::new(self.parse_block()?))
        } else {
            self.expect(TokenKind::Semicolon)?;
            None
        };

        Ok(ClassMember::Method(MethodDecl {
            modifiers,
            return_type,
            name,
            params,
            throws,
            body,
            default_value,
        }))
    }

    fn parse_field_decl_tail(
        &mut self,
        modifiers: Vec<Modifier>,
        var_type: Type,
        first_name: String,
    ) -> Result<ClassMember, String> {
        let mut declarators = Vec::new();

        let init = if self.peek().kind == TokenKind::Eq {
            self.next();
            Some(self.parse_expression()?)
        } else {
            None
        };
        declarators.push(VarDeclarator {
            name: first_name,
            init,
        });

        while self.peek().kind == TokenKind::Comma {
            self.next();
            let name = self.expect(TokenKind::Ident)?.text;
            let init = if self.peek().kind == TokenKind::Eq {
                self.next();
                Some(self.parse_expression()?)
            } else {
                None
            };
            declarators.push(VarDeclarator { name, init });
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(ClassMember::Field(FieldDecl {
            modifiers,
            var_type,
            declarators,
        }))
    }

    fn parse_empty(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Empty)
    }

    fn parse_block(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::LBrace)?;
        let mut statements = Vec::new();
        while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
            let stmt = self.parser_distribute()?;
            statements.push(stmt);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Stmt::Block { statements })
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn is_assignment_op(&self, kind: &TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Eq
                | TokenKind::PlusEq
                | TokenKind::MinusEq
                | TokenKind::StarEq
                | TokenKind::SlashEq
                | TokenKind::PercentEq
                | TokenKind::AndEq
                | TokenKind::OrEq
                | TokenKind::XorEq
                | TokenKind::ShlEq
                | TokenKind::ShrEq
                | TokenKind::UrshEq
        )
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_conditional()?;
        if self.is_assignment_op(&self.peek().kind) {
            let _op = self.next();
            let value = self.parse_assignment()?;
            return Ok(Expr::Assign {
                target: Box::new(expr),
                value: Box::new(value),
            });
        }
        Ok(expr)
    }

    fn parse_conditional(&mut self) -> Result<Expr, String> {
        let expr = self.parse_binary_expr(0)?;
        if self.peek().kind == TokenKind::Question {
            self.next();
            let then_expr = self.parse_expression()?;
            self.expect(TokenKind::Colon)?;
            let else_expr = self.parse_conditional()?;
            return Ok(Expr::Ternary {
                cond: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            });
        }
        Ok(expr)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.parse_unary_expr()?;

        loop {
            if self.peek().kind == TokenKind::Keyword && self.peek().text == "instanceof" {
                if 7 < min_prec {
                    break;
                }
                self.next();
                let target_type = self.parse_type()?;
                let pattern_var = if self.peek().kind == TokenKind::Ident {
                    Some(self.next().text)
                } else {
                    None
                };
                left = Expr::InstanceOf {
                    expr: Box::new(left),
                    target_type,
                    pattern_var,
                };
                continue;
            }

            if self.peek().kind == TokenKind::Lt {
                let mut depth = 0;
                let mut offset = 0;
                let mut is_generic_method_ref = false;
                while let Some(tok) = self.peek_offset(offset) {
                    if tok.kind == TokenKind::Lt {
                        depth += 1;
                    } else if tok.kind == TokenKind::Gt {
                        depth -= 1;
                        if depth == 0 {
                            if self.peek_offset(offset + 1).is_some_and(|t| t.kind == TokenKind::ColonColon) {
                                is_generic_method_ref = true;
                            }
                            break;
                        }
                    } else if tok.kind == TokenKind::Shr {
                        if depth <= 2 {
                            if self.peek_offset(offset + 1).is_some_and(|t| t.kind == TokenKind::ColonColon) {
                                is_generic_method_ref = true;
                            }
                            break;
                        }
                        depth -= 2;
                    } else if tok.kind == TokenKind::Semicolon || tok.kind == TokenKind::RParen || tok.kind == TokenKind::Eof {
                        break;
                    }
                    offset += 1;
                }
                if is_generic_method_ref {
                    break;
                }
            }

            let prec = match self.get_binary_precedence(&self.peek().kind) {
                Some(p) if p >= min_prec => p,
                _ => break,
            };

            let op_kind = self.next().kind;
            let op = token_kind_to_bin_op(op_kind).ok_or("未知二元运算符")?;

            let right = self.parse_binary_expr(prec + 1)?;

            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn is_cast_start(&self) -> bool {
        if self.peek().kind != TokenKind::LParen {
            return false;
        }
        let mut offset = self.skip_annotations_at(1);
        let Some(first) = self.peek_offset(offset) else { return false; };
        let is_prim = first.kind == TokenKind::Keyword
            && matches!(
                first.text.as_str(),
                "int" | "boolean" | "char" | "byte" | "short" | "long" | "float" | "double" | "void"
            );
        let is_ident = first.kind == TokenKind::Ident;
        if !is_prim && !is_ident {
            return false;
        }
        offset += 1;
        while self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Dot) {
            offset += 1;
            offset = self.skip_annotations_at(offset);
            if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Ident) {
                offset += 1;
            } else {
                break;
            }
        }
        if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Lt) {
            let mut depth = 0;
            while let Some(tok) = self.peek_offset(offset) {
                if tok.kind == TokenKind::Lt {
                    depth += 1;
                } else if tok.kind == TokenKind::Gt {
                    depth -= 1;
                    if depth == 0 {
                        offset += 1;
                        break;
                    }
                } else if tok.kind == TokenKind::Shr {
                    if depth <= 2 {
                        offset += 1;
                        break;
                    }
                    depth -= 2;
                } else if tok.kind == TokenKind::Ursh {
                    if depth <= 3 {
                        offset += 1;
                        break;
                    }
                    depth -= 3;
                } else if tok.kind == TokenKind::Semicolon || tok.kind == TokenKind::Eof {
                    return false;
                }
                offset += 1;
            }
        }
        loop {
            offset = self.skip_annotations_at(offset);
            if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::LBracket)
                && self.peek_offset(offset + 1).is_some_and(|t| t.kind == TokenKind::RBracket)
            {
                offset += 2;
            } else {
                break;
            }
        }
        if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::RParen) {
            if let Some(next) = self.peek_offset(offset + 1) {
                return matches!(
                    next.kind,
                    TokenKind::Ident
                        | TokenKind::Number
                        | TokenKind::String
                        | TokenKind::Char
                        | TokenKind::LParen
                        | TokenKind::Not
                        | TokenKind::Tilde
                        | TokenKind::PlusPlus
                        | TokenKind::MinusMinus
                        | TokenKind::Minus
                        | TokenKind::Plus
                ) || (next.kind == TokenKind::Keyword
                    && matches!(next.text.as_str(), "this" | "super" | "new" | "null" | "true" | "false"));
            }
        }
        false
    }

    fn parse_unary_expr(&mut self) -> Result<Expr, String> {
        if self.is_cast_start() {
            self.next(); // '('
            let target_type = self.parse_type()?;
            self.expect(TokenKind::RParen)?;
            let expr = self.parse_unary_expr()?;
            return Ok(Expr::Cast {
                target_type,
                expr: Box::new(expr),
            });
        }

        match self.peek().kind {
            TokenKind::Plus => {
                self.next();
                self.parse_unary_expr()
            }
            TokenKind::Minus => {
                self.next();
                let expr = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            TokenKind::Not => {
                self.next();
                let expr = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            TokenKind::Tilde => {
                self.next();
                let expr = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnaryOp::BitNot,
                    expr: Box::new(expr),
                })
            }
            TokenKind::PlusPlus => {
                self.next();
                let expr = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnaryOp::PlusPlus,
                    expr: Box::new(expr),
                })
            }
            TokenKind::MinusMinus => {
                self.next();
                let expr = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnaryOp::MinusMinus,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix_expr(),
        }
    }

    fn parse_postfix_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.peek().kind == TokenKind::Lt {
                let mut depth = 0;
                let mut offset = 0;
                let mut is_generic_method_ref = false;
                while let Some(tok) = self.peek_offset(offset) {
                    if tok.kind == TokenKind::Lt { depth += 1; }
                    else if tok.kind == TokenKind::Gt {
                        depth -= 1;
                        if depth == 0 {
                            if self.peek_offset(offset + 1).is_some_and(|t| t.kind == TokenKind::ColonColon) {
                                is_generic_method_ref = true;
                            }
                            break;
                        }
                    } else if tok.kind == TokenKind::Shr {
                        if depth <= 2 {
                            if self.peek_offset(offset + 1).is_some_and(|t| t.kind == TokenKind::ColonColon) {
                                is_generic_method_ref = true;
                            }
                            break;
                        }
                        depth -= 2;
                    } else if tok.kind == TokenKind::Semicolon || tok.kind == TokenKind::RParen || tok.kind == TokenKind::Eof {
                        break;
                    }
                    offset += 1;
                }
                if is_generic_method_ref {
                    self.skip_type_params()?;
                    continue;
                }
            }

            if self.peek().kind == TokenKind::Dot {
                if self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::Keyword && t.text == "new") {
                    self.next(); // '.'
                    self.next(); // 'new'
                    let inner_type = self.parse_base_type()?;
                    self.expect(TokenKind::LParen)?;
                    let mut args = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.peek().kind == TokenKind::Comma { self.next(); }
                            else { break; }
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    if self.peek().kind == TokenKind::LBrace {
                        self.next();
                        while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
                            if self.peek().kind == TokenKind::Semicolon { self.next(); continue; }
                            let _ = self.parse_class_member();
                        }
                        self.expect(TokenKind::RBrace)?;
                    }
                    expr = Expr::NewInstance {
                        class_type: inner_type,
                        args,
                    };
                    continue;
                }

                self.next();
                if self.peek().kind == TokenKind::Lt {
                    self.skip_type_params()?;
                }
                let member = if self.peek().kind == TokenKind::Ident || self.peek().kind == TokenKind::Keyword {
                    self.next().text
                } else {
                    return Err(format!("期望成员标识符，当前 {:?}", self.peek()));
                };
                expr = Expr::MemberAccess {
                    object: Box::new(expr),
                    member,
                };
            } else if self.peek().kind == TokenKind::ColonColon {
                self.next(); // '::'
                if self.peek().kind == TokenKind::Lt {
                    self.skip_type_params()?;
                }
                let member = if self.peek().kind == TokenKind::Ident || self.peek().kind == TokenKind::Keyword {
                    self.next().text
                } else {
                    return Err(format!("期望方法引用标识符，当前 {:?}", self.peek()));
                };
                expr = Expr::MemberAccess {
                    object: Box::new(expr),
                    member: format!("::{}", member),
                };
            } else if self.peek().kind == TokenKind::LParen {
                self.next();
                let mut args = Vec::new();
                if self.peek().kind != TokenKind::RParen {
                    loop {
                        args.push(self.parse_expression()?);
                        if self.peek().kind == TokenKind::Comma {
                            self.next();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.peek().kind == TokenKind::LBracket {
                self.next();
                if self.peek().kind == TokenKind::RBracket {
                    self.next();
                    expr = Expr::MemberAccess {
                        object: Box::new(expr),
                        member: "[]".to_string(),
                    };
                } else {
                    let index = self.parse_expression()?;
                    self.expect(TokenKind::RBracket)?;
                    expr = Expr::ArrayAccess {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
            } else if self.peek().kind == TokenKind::PlusPlus {
                self.next();
                expr = Expr::Unary {
                    op: UnaryOp::PlusPlus,
                    expr: Box::new(expr),
                };
            } else if self.peek().kind == TokenKind::MinusMinus {
                self.next();
                expr = Expr::Unary {
                    op: UnaryOp::MinusMinus,
                    expr: Box::new(expr),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let tok = self.peek();
        match tok.kind {
            TokenKind::Number => {
                let text = self.next().text;
                let clean_text = text
                    .trim_end_matches(|c| c == 'l' || c == 'L' || c == 'f' || c == 'F' || c == 'd' || c == 'D')
                    .replace('_', "");
                let val = if clean_text.starts_with("0x") || clean_text.starts_with("0X") {
                    i64::from_str_radix(&clean_text[2..], 16).unwrap_or(0)
                } else if clean_text.starts_with("0b") || clean_text.starts_with("0B") {
                    i64::from_str_radix(&clean_text[2..], 2).unwrap_or(0)
                } else if let Ok(v) = clean_text.parse::<i64>() {
                    v
                } else if let Ok(f) = clean_text.parse::<f64>() {
                    f as i64
                } else {
                    0
                };
                Ok(Expr::IntLiteral(val))
            }
            TokenKind::String | TokenKind::TextBlock => {
                let text = self.next().text;
                Ok(Expr::StringLiteral(text))
            }
            TokenKind::Char => {
                let text = self.next().text;
                Ok(Expr::CharLiteral(text))
            }
            TokenKind::Ident => {
                let text = self.next().text;
                if self.peek().kind == TokenKind::Arrow {
                    self.next(); // '->'
                    let body = if self.peek().kind == TokenKind::LBrace {
                        let stmt = self.parse_block()?;
                        Box::new(Expr::StringLiteral(format!("{:?}", stmt)))
                    } else {
                        Box::new(self.parse_expression()?)
                    };
                    return Ok(Expr::Lambda {
                        params: vec![text],
                        body,
                    });
                }
                match text.as_str() {
                    "null" => Ok(Expr::NullLiteral),
                    "true" => Ok(Expr::BoolLiteral(true)),
                    "false" => Ok(Expr::BoolLiteral(false)),
                    "this" => Ok(Expr::This),
                    "super" => Ok(Expr::Super),
                    _ => Ok(Expr::Ident(text)),
                }
            }
            TokenKind::Keyword => {
                let text = self.next().text;
                match text.as_str() {
                    "null" => Ok(Expr::NullLiteral),
                    "true" => Ok(Expr::BoolLiteral(true)),
                    "false" => Ok(Expr::BoolLiteral(false)),
                    "this" => Ok(Expr::This),
                    "super" => Ok(Expr::Super),
                    "int" | "boolean" | "char" | "byte" | "short" | "long" | "float" | "double" | "void" => {
                        let prim_name = text;
                        let mut elem_type = prim_name.clone();
                        while self.peek().kind == TokenKind::LBracket && self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::RBracket) {
                            self.next();
                            self.next();
                            elem_type.push_str("[]");
                        }
                        Ok(Expr::Ident(elem_type))
                    }
                    "new" => {
                        let elem_type = self.parse_base_type()?;
                        if self.peek().kind == TokenKind::LBracket {
                            let mut dimensions = Vec::new();
                            while self.peek().kind == TokenKind::LBracket {
                                self.next();
                                if self.peek().kind != TokenKind::RBracket {
                                    dimensions.push(self.parse_expression()?);
                                    self.expect(TokenKind::RBracket)?;
                                } else {
                                    self.expect(TokenKind::RBracket)?;
                                }
                            }
                            if self.peek().kind == TokenKind::LBrace {
                                self.next();
                                let mut elements = Vec::new();
                                if self.peek().kind != TokenKind::RBrace {
                                    loop {
                                        elements.push(self.parse_expression()?);
                                        if self.peek().kind == TokenKind::Comma {
                                            self.next();
                                            if self.peek().kind == TokenKind::RBrace {
                                                break;
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                                self.expect(TokenKind::RBrace)?;
                                Ok(Expr::NewArrayInit {
                                    elem_type,
                                    elements,
                                })
                            } else {
                                Ok(Expr::NewArray {
                                    elem_type,
                                    dimensions,
                                })
                            }
                        } else if self.peek().kind == TokenKind::LParen {
                            self.next();
                            let mut args = Vec::new();
                            if self.peek().kind != TokenKind::RParen {
                                loop {
                                    args.push(self.parse_expression()?);
                                    if self.peek().kind == TokenKind::Comma {
                                        self.next();
                                    } else {
                                        break;
                                    }
                                }
                            }
                            self.expect(TokenKind::RParen)?;
                            if self.peek().kind == TokenKind::LBrace {
                                self.next();
                                while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
                                    if self.peek().kind == TokenKind::Semicolon {
                                        self.next();
                                        continue;
                                    }
                                    let _ = self.parse_class_member();
                                }
                                self.expect(TokenKind::RBrace)?;
                            }
                            Ok(Expr::NewInstance {
                                class_type: elem_type,
                                args,
                            })
                        } else {
                            Ok(Expr::NewInstance {
                                class_type: elem_type,
                                args: vec![],
                            })
                        }
                    }
                    _ => Err(format!("未预期的关键字表达式: {}", text)),
                }
            }
            TokenKind::LBrace => {
                self.next();
                let mut elements = Vec::new();
                if self.peek().kind != TokenKind::RBrace {
                    loop {
                        elements.push(self.parse_expression()?);
                        if self.peek().kind == TokenKind::Comma {
                            self.next();
                            if self.peek().kind == TokenKind::RBrace {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RBrace)?;
                Ok(Expr::ArrayInit(elements))
            }
            TokenKind::LParen => {
                if self.is_cast_start() {
                    self.next(); // '('
                    let target_type = self.parse_type()?;
                    self.expect(TokenKind::RParen)?;
                    let expr = self.parse_unary_expr()?;
                    return Ok(Expr::Cast {
                        target_type,
                        expr: Box::new(expr),
                    });
                }

                if self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::Ident)
                    && self.peek_offset(2).is_some_and(|t| t.kind == TokenKind::Arrow)
                {
                    self.next(); // '('
                    let param = self.next().text;
                    self.next(); // '->'
                    let body = if self.peek().kind == TokenKind::LBrace {
                        let stmt = self.parse_block()?;
                        Box::new(Expr::StringLiteral(format!("{:?}", stmt)))
                    } else {
                        Box::new(self.parse_expression()?)
                    };
                    self.expect(TokenKind::RParen)?;
                    return Ok(Expr::Lambda {
                        params: vec![param],
                        body,
                    });
                }

                let mut is_lambda = false;
                let mut offset = 1;
                let mut paren_depth = 1;
                while let Some(tok) = self.peek_offset(offset) {
                    if tok.kind == TokenKind::LParen {
                        paren_depth += 1;
                    } else if tok.kind == TokenKind::RParen {
                        paren_depth -= 1;
                        if paren_depth == 0 {
                            if self.peek_offset(offset + 1).is_some_and(|next| next.kind == TokenKind::Arrow) {
                                is_lambda = true;
                            }
                            break;
                        }
                    } else if tok.kind == TokenKind::Semicolon || tok.kind == TokenKind::Eof {
                        break;
                    }
                    offset += 1;
                }

                if is_lambda {
                    self.next();
                    let mut params = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        loop {
                            let _param_mods = self.parse_modifiers()?;
                            let mut p_type = self.parse_type()?;
                            while self.peek().kind == TokenKind::At {
                                self.next();
                                let _ = self.expect_qualified_ident()?;
                                if self.peek().kind == TokenKind::LParen {
                                    self.next();
                                    let mut depth = 1;
                                    while depth > 0 && self.peek().kind != TokenKind::Eof {
                                        if self.peek().kind == TokenKind::LParen { depth += 1; }
                                        else if self.peek().kind == TokenKind::RParen { depth -= 1; }
                                        self.next();
                                    }
                                }
                            }
                            if self.peek().kind == TokenKind::Ellipsis {
                                self.next();
                                p_type = Type::Array {
                                    element_type: Box::new(p_type),
                                    dimensions: 1,
                                };
                            }
                            let p_name = if self.peek().kind == TokenKind::Ident {
                                self.next().text
                            } else {
                                match p_type {
                                    Type::Reference { name, .. } => name,
                                    _ => "param".to_string(),
                                }
                            };
                            params.push(p_name);
                            if self.peek().kind == TokenKind::Comma {
                                self.next();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    self.expect(TokenKind::Arrow)?;
                    let body = if self.peek().kind == TokenKind::LBrace {
                        let stmt = self.parse_block()?;
                        Box::new(Expr::StringLiteral(format!("{:?}", stmt)))
                    } else {
                        Box::new(self.parse_expression()?)
                    };
                    Ok(Expr::Lambda { params, body })
                } else {
                    self.next();
                    let expr = self.parse_expression()?;
                    self.expect(TokenKind::RParen)?;
                    Ok(expr)
                }
            }
            _ => Err(format!("无法识别的表达式开头: {:?}", tok)),
        }
    }

    fn get_binary_precedence(&self, kind: &TokenKind) -> Option<u8> {
        match kind {
            TokenKind::OrOr => Some(1),
            TokenKind::AndAnd => Some(2),
            TokenKind::Or => Some(3),
            TokenKind::Xor => Some(4),
            TokenKind::And => Some(5),
            TokenKind::EqEq | TokenKind::Ne => Some(6),
            TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge => Some(7),
            TokenKind::Shl | TokenKind::Shr | TokenKind::Ursh => Some(8),
            TokenKind::Plus | TokenKind::Minus => Some(9),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(10),
            _ => None,
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ExprStmt { expr })
    }

    fn parse_modifiers(&mut self) -> Result<Vec<Modifier>, String> {
        let mut modifiers: Vec<Modifier> = Vec::new();
        loop {
            if self.peek().kind == TokenKind::At {
                if self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::Keyword && t.text == "interface") {
                    break;
                }
                self.next();
                let name = self.expect_qualified_ident()?;
                let args = if self.peek().kind == TokenKind::LParen {
                    self.next();
                    let mut arg_tokens = Vec::new();
                    let mut depth = 1;
                    while depth > 0 && self.peek().kind != TokenKind::Eof {
                        if self.peek().kind == TokenKind::LParen {
                            depth += 1;
                        } else if self.peek().kind == TokenKind::RParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        arg_tokens.push(self.next().text);
                    }
                    self.expect(TokenKind::RParen)?;
                    Some(arg_tokens)
                } else {
                    None
                };
                modifiers.push(Modifier::Annotation { name, args });
                continue;
            }

            if self.peek().kind == TokenKind::Keyword {
                let m = match self.peek().text.as_str() {
                    "public" => Modifier::Public,
                    "protected" => Modifier::Protected,
                    "private" => Modifier::Private,
                    "static" => Modifier::Static,
                    "final" => Modifier::Final,
                    "abstract" => Modifier::Abstract,
                    "synchronized" => Modifier::Synchronized,
                    "native" => Modifier::Native,
                    "transient" => Modifier::Transient,
                    "volatile" => Modifier::Volatile,
                    "strictfp" => Modifier::Strictfp,
                    "default" => Modifier::Default,
                    _ => break,
                };
                self.next();
                modifiers.push(m);
            } else {
                break;
            }
        }
        Ok(modifiers)
    }

    fn parse_base_type(&mut self) -> Result<Type, String> {
        while self.peek().kind == TokenKind::At {
            self.next();
            let _ = self.expect_qualified_ident()?;
            if self.peek().kind == TokenKind::LParen {
                self.next();
                let mut depth = 1;
                while depth > 0 && self.peek().kind != TokenKind::Eof {
                    if self.peek().kind == TokenKind::LParen {
                        depth += 1;
                    } else if self.peek().kind == TokenKind::RParen {
                        depth -= 1;
                    }
                    self.next();
                }
            }
        }

        if self.peek().kind == TokenKind::Question {
            self.next();
            if self.peek().kind == TokenKind::Keyword && self.peek().text == "extends" {
                self.next();
                let bound = self.parse_type()?;
                return Ok(Type::Wildcard {
                    bound: Some(Box::new(bound)),
                    is_extends: true,
                });
            } else if self.peek().kind == TokenKind::Keyword && self.peek().text == "super" {
                self.next();
                let bound = self.parse_type()?;
                return Ok(Type::Wildcard {
                    bound: Some(Box::new(bound)),
                    is_extends: false,
                });
            } else {
                return Ok(Type::Wildcard {
                    bound: None,
                    is_extends: true,
                });
            }
        }

        let tok = self.next();
        let mut full_name = match tok.text.as_str() {
            "int" => return Ok(Type::Primitive(PrimitiveType::Int)),
            "boolean" => return Ok(Type::Primitive(PrimitiveType::Boolean)),
            "char" => return Ok(Type::Primitive(PrimitiveType::Char)),
            "byte" => return Ok(Type::Primitive(PrimitiveType::Byte)),
            "short" => return Ok(Type::Primitive(PrimitiveType::Short)),
            "long" => return Ok(Type::Primitive(PrimitiveType::Long)),
            "float" => return Ok(Type::Primitive(PrimitiveType::Float)),
            "double" => return Ok(Type::Primitive(PrimitiveType::Double)),
            "void" => return Ok(Type::Primitive(PrimitiveType::Void)),
            "var" => return Ok(Type::Var),
            _ => tok.text,
        };

        // 解析泛型 List<Map<string,string>>
        loop {
            let mut type_args = Vec::new();
            if self.peek().kind == TokenKind::Lt {
                self.next();
                if self.peek().kind != TokenKind::Gt && self.peek().kind != TokenKind::Shr && self.peek().kind != TokenKind::Ursh {
                    loop {
                        type_args.push(self.parse_type()?);
                        if self.peek().kind == TokenKind::Comma {
                            self.next();
                        } else {
                            break;
                        }
                    }
                }
                self.expect_gt()?;
            }

            if self.peek().kind == TokenKind::Dot {
                self.next(); // '.'
                while self.peek().kind == TokenKind::At {
                    self.next();
                    let _ = self.expect_qualified_ident()?;
                    if self.peek().kind == TokenKind::LParen {
                        self.next();
                        let mut depth = 1;
                        while depth > 0 && self.peek().kind != TokenKind::Eof {
                            if self.peek().kind == TokenKind::LParen { depth += 1; }
                            else if self.peek().kind == TokenKind::RParen { depth -= 1; }
                            self.next();
                        }
                    }
                }
                if self.peek().kind == TokenKind::Ident {
                    let part = self.next().text;
                    full_name.push('.');
                    full_name.push_str(&part);
                } else {
                    return Ok(Type::Reference {
                        name: full_name,
                        type_args,
                    });
                }
            } else {
                return Ok(Type::Reference {
                    name: full_name,
                    type_args,
                });
            }
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let mut t = self.parse_base_type()?;

        loop {
            if self.peek().kind == TokenKind::At {
                let mut offset = 1;
                while self.peek_offset(offset).is_some_and(|tok| tok.kind == TokenKind::Dot || tok.kind == TokenKind::Ident) {
                    offset += 1;
                }
                if self.peek_offset(offset).is_some_and(|tok| tok.kind == TokenKind::LParen) {
                    let mut depth = 1;
                    offset += 1;
                    while depth > 0 && self.peek_offset(offset).is_some() {
                        if self.peek_offset(offset).unwrap().kind == TokenKind::LParen { depth += 1; }
                        else if self.peek_offset(offset).unwrap().kind == TokenKind::RParen { depth -= 1; }
                        offset += 1;
                    }
                }
                if self.peek_offset(offset).is_some_and(|tok| tok.kind == TokenKind::LBracket) {
                    self.next();
                    let _ = self.expect_qualified_ident()?;
                    if self.peek().kind == TokenKind::LParen {
                        self.next();
                        let mut depth = 1;
                        while depth > 0 && self.peek().kind != TokenKind::Eof {
                            if self.peek().kind == TokenKind::LParen { depth += 1; }
                            else if self.peek().kind == TokenKind::RParen { depth -= 1; }
                            self.next();
                        }
                    }
                } else {
                    break;
                }
            }

            if self.peek().kind == TokenKind::LBracket && self.peek_offset(1).is_some_and(|next| next.kind == TokenKind::RBracket) {
                self.next();
                self.expect(TokenKind::RBracket)?;
                t = Type::Array {
                    element_type: Box::new(t),
                    dimensions: 1,
                };
            } else {
                break;
            }
        }

        Ok(t)
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, String> {
        let modifiers: Vec<Modifier> = self.parse_modifiers()?;
        let var_type: Type = self.parse_type()?;
        let mut declarators: Vec<VarDeclarator> = Vec::new();

        loop {
            let name = self.expect(TokenKind::Ident)?.text;
            let init = if self.peek().kind == TokenKind::Eq {
                self.next();
                Some(self.parse_expression()?)
            } else {
                None
            };
            declarators.push(VarDeclarator { name, init });
            if self.peek().kind == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::VarDecl {
            modifiers,
            var_type,
            declarators,
        })
    }

    fn parse_local_class_decl(&mut self) -> Result<Stmt, String> {
        let class_decl = self.parse_class_decl()?;
        Ok(Stmt::LocalClassDecl(class_decl))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("if")?;
        self.expect(TokenKind::LParen)?;
        let cond = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let then_branch = Box::new(self.parser_distribute()?);
        let else_branch = if self.peek().kind == TokenKind::Keyword && self.peek().text == "else" {
            self.next();
            Some(Box::new(self.parser_distribute()?))
        } else {
            None
        };

        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("while")?;
        self.expect(TokenKind::LParen)?;
        let cond = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let body = Box::new(self.parser_distribute()?);
        Ok(Stmt::While { cond, body })
    }

    fn parse_do_while(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("do")?;
        let body = Box::new(self.parser_distribute()?);
        self.expect_keyword("while")?;
        self.expect(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Stmt::DoWhile { body, condition })
    }

    fn is_local_class_start(&self) -> bool {
        let mut offset = self.skip_annotations_at(0);
        while self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Keyword && (t.text == "final" || t.text == "abstract" || t.text == "strictfp")) {
            offset += 1;
        }
        if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::At) && self.peek_offset(offset + 1).is_some_and(|t| t.kind == TokenKind::Keyword && t.text == "interface") {
            return true;
        }
        if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Ident && t.text == "record") {
            return true;
        }
        self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Keyword && matches!(t.text.as_str(), "class" | "interface" | "enum"))
    }

    pub fn is_variable_declaration_start(&self) -> bool {
        let mut offset = self.skip_annotations_at(0);

        let Some(current) = self.peek_offset(offset) else { return false; };
        if current.kind == TokenKind::Keyword && current.text == "final" {
            return true;
        }
        if current.kind == TokenKind::Keyword {
            match current.text.as_str() {
                "int" | "boolean" | "char" | "byte" | "short" | "long" | "float" | "double" => {
                    return true;
                }
                _ => {}
            }
        }
        if (current.kind == TokenKind::Ident || current.kind == TokenKind::Keyword)
            && current.text == "var"
            && self.peek_offset(offset + 1).is_some_and(|next| next.kind == TokenKind::Ident)
        {
            return true;
        }
        if current.kind == TokenKind::Ident {
            offset += 1;
            loop {
                while self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Dot) {
                    offset += 1;
                    offset = self.skip_annotations_at(offset);
                    if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Ident) {
                        offset += 1;
                    } else {
                        break;
                    }
                }
                if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Lt) {
                    let mut depth = 0;
                    while let Some(tok) = self.peek_offset(offset) {
                        if tok.kind == TokenKind::Lt {
                            depth += 1;
                        } else if tok.kind == TokenKind::Gt {
                            depth -= 1;
                            if depth == 0 {
                                offset += 1;
                                break;
                            }
                        } else if tok.kind == TokenKind::Shr {
                            if depth <= 2 {
                                offset += 1;
                                break;
                            }
                            depth -= 2;
                        } else if tok.kind == TokenKind::Ursh {
                            if depth <= 3 {
                                offset += 1;
                                break;
                            }
                            depth -= 3;
                        } else if tok.kind == TokenKind::Semicolon || tok.kind == TokenKind::Eof {
                            return false;
                        }
                        offset += 1;
                    }
                }
                if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Dot) {
                    continue;
                }
                break;
            }

            while let (Some(l), Some(r)) = (self.peek_offset(offset), self.peek_offset(offset + 1)) {
                if l.kind == TokenKind::LBracket && r.kind == TokenKind::RBracket {
                    offset += 2;
                } else {
                    break;
                }
            }

            if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Ident) {
                if let Some(after) = self.peek_offset(offset + 1) {
                    return matches!(
                        after.kind,
                        TokenKind::Eq
                            | TokenKind::Semicolon
                            | TokenKind::Comma
                            | TokenKind::LBracket
                            | TokenKind::Colon
                    );
                }
            }
        }
        false
    }

    fn is_for_each_start(&self) -> bool {
        let mut offset = self.skip_annotations_at(0);
        if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Keyword && t.text == "final") {
            offset += 1;
        }
        if let Some(t) = self.peek_offset(offset) {
            if t.kind == TokenKind::Ident
                || (t.kind == TokenKind::Keyword
                    && matches!(
                        t.text.as_str(),
                        "int"
                            | "boolean"
                            | "char"
                            | "byte"
                            | "short"
                            | "long"
                            | "float"
                            | "double"
                            | "String"
                    ))
            {
                offset += 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
        while self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Dot) {
            offset += 2;
        }
        if self.peek_offset(offset).is_some_and(|t| t.kind == TokenKind::Lt) {
            let mut depth = 0;
            while let Some(tok) = self.peek_offset(offset) {
                if tok.kind == TokenKind::Lt {
                    depth += 1;
                } else if tok.kind == TokenKind::Gt {
                    depth -= 1;
                    if depth == 0 {
                        offset += 1;
                        break;
                    }
                } else if tok.kind == TokenKind::Shr {
                    if depth <= 2 {
                        offset += 1;
                        break;
                    }
                    depth -= 2;
                } else if tok.kind == TokenKind::Ursh {
                    if depth <= 3 {
                        offset += 1;
                        break;
                    }
                    depth -= 3;
                } else if tok.kind == TokenKind::Semicolon || tok.kind == TokenKind::Eof {
                    return false;
                }
                offset += 1;
            }
        }
        while let (Some(l), Some(r)) = (self.peek_offset(offset), self.peek_offset(offset + 1)) {
            if l.kind == TokenKind::LBracket && r.kind == TokenKind::RBracket {
                offset += 2;
            } else {
                break;
            }
        }
        if let (Some(name), Some(colon)) = (self.peek_offset(offset), self.peek_offset(offset + 1)) {
            name.kind == TokenKind::Ident && colon.kind == TokenKind::Colon
        } else {
            false
        }
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("for")?;
        self.expect(TokenKind::LParen)?;

        if self.is_for_each_start() {
            let modifiers = self.parse_modifiers()?;
            let var_type = self.parse_type()?;
            let var_name = self.expect(TokenKind::Ident)?.text;
            self.expect(TokenKind::Colon)?;
            let iterable = self.parse_expression()?;
            self.expect(TokenKind::RParen)?;
            let body = Box::new(self.parser_distribute()?);

            return Ok(Stmt::ForEach {
                modifiers,
                var_type,
                var_name,
                iterable,
                body,
            });
        }

        let mut init = Vec::new();
        if self.peek().kind != TokenKind::Semicolon {
            if self.is_variable_declaration_start() {
                init.push(self.parse_var_decl()?);
            } else {
                let expr = self.parse_expression()?;
                init.push(Stmt::ExprStmt { expr });
                self.expect(TokenKind::Semicolon)?;
            }
        } else {
            self.expect(TokenKind::Semicolon)?;
        }

        let condition = if self.peek().kind != TokenKind::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;

        let mut update = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                update.push(self.parse_expression()?);
                if self.peek().kind == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen)?;

        let body = Box::new(self.parser_distribute()?);
        Ok(Stmt::For {
            init,
            condition,
            update,
            body,
        })
    }

    fn parse_switch(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("switch")?;
        self.expect(TokenKind::LParen)?;
        let selector = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        self.expect(TokenKind::LBrace)?;
        let mut cases = Vec::new();

        while self.peek().kind != TokenKind::RBrace && self.peek().kind != TokenKind::Eof {
            let mut labels = Vec::new();
            if self.peek().kind == TokenKind::Keyword && self.peek().text == "case" {
                self.next();
                loop {
                    labels.push(self.parse_expression()?);
                    if self.peek().kind == TokenKind::Comma {
                        self.next();
                    } else {
                        break;
                    }
                }
                self.expect(TokenKind::Colon)?;
            } else if self.peek().kind == TokenKind::Keyword && self.peek().text == "default" {
                self.next();
                self.expect(TokenKind::Colon)?;
            }

            let mut statements = Vec::new();
            while self.peek().kind != TokenKind::RBrace
                && self.peek().kind != TokenKind::Eof
                && !(self.peek().kind == TokenKind::Keyword
                    && (self.peek().text == "case" || self.peek().text == "default"))
            {
                statements.push(self.parser_distribute()?);
            }

            cases.push(SwitchCase { labels, statements });
        }
        self.expect(TokenKind::RBrace)?;

        Ok(Stmt::Switch { selector, cases })
    }

    fn parse_yield(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("yield")?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Yield { value })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("return")?;
        let value = if self.peek().kind == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return { value })
    }

    fn parse_break(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("break")?;
        let label = if self.peek().kind == TokenKind::Ident {
            Some(self.next().text)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Break { label })
    }

    fn parse_continue(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("continue")?;
        let label = if self.peek().kind == TokenKind::Ident {
            Some(self.next().text)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Continue { label })
    }

    fn parse_throw(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("throw")?;
        let expr = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Throw { expr })
    }

    fn parse_synchronized(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("synchronized")?;
        self.expect(TokenKind::LParen)?;
        let lock = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        let body = Box::new(self.parser_distribute()?);
        Ok(Stmt::Synchronized { lock, body })
    }

    fn parse_try(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("try")?;

        let mut resources = Vec::new();
        if self.peek().kind == TokenKind::LParen {
            self.next();
            while self.peek().kind != TokenKind::RParen && self.peek().kind != TokenKind::Eof {
                let modifiers = self.parse_modifiers()?;
                let var_type = self.parse_type()?;
                let mut declarators = Vec::new();
                loop {
                    let name = self.expect(TokenKind::Ident)?.text;
                    let init = if self.peek().kind == TokenKind::Eq {
                        self.next();
                        Some(self.parse_expression()?)
                    } else {
                        None
                    };
                    declarators.push(VarDeclarator { name, init });
                    if self.peek().kind == TokenKind::Comma {
                        self.next();
                    } else {
                        break;
                    }
                }
                if self.peek().kind == TokenKind::Semicolon {
                    self.next();
                }
                resources.push(Stmt::VarDecl {
                    modifiers,
                    var_type,
                    declarators,
                });
            }
            self.expect(TokenKind::RParen)?;
        }

        let body = Box::new(self.parse_block()?);

        let mut catches = Vec::new();
        while self.peek().kind == TokenKind::Keyword && self.peek().text == "catch" {
            self.next();
            self.expect(TokenKind::LParen)?;

            let modifiers = self.parse_modifiers()?;
            let mut types = Vec::new();
            loop {
                types.push(self.parse_type()?);
                if self.peek().kind == TokenKind::Or {
                    self.next();
                } else {
                    break;
                }
            }

            let var_name = self.expect(TokenKind::Ident)?.text;
            self.expect(TokenKind::RParen)?;
            let catch_body = Box::new(self.parse_block()?);

            catches.push(CatchClause {
                modifiers,
                types,
                var_name,
                body: catch_body,
            });
        }

        let finally_body = if self.peek().kind == TokenKind::Keyword && self.peek().text == "finally" {
            self.next();
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Ok(Stmt::Try {
            resources,
            body,
            catches,
            finally_body,
        })
    }

    fn parse_labeled(&mut self) -> Result<Stmt, String> {
        let label = self.expect(TokenKind::Ident)?.text;
        self.expect(TokenKind::Colon)?;
        let body = Box::new(self.parser_distribute()?);
        Ok(Stmt::Labeled { label, body })
    }

    fn parse_assert(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("assert")?;
        let condition = self.parse_expression()?;
        let message = if self.peek().kind == TokenKind::Colon {
            self.next();
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Assert { condition, message })
    }

    fn parser_distribute(&mut self) -> Result<Stmt, String> {
        let token: &Token = self.peek();
        if token.kind == TokenKind::Semicolon {
            return self.parse_empty();
        }
        if token.kind == TokenKind::LBrace {
            return self.parse_block();
        }
        if token.kind == TokenKind::Ident && self.peek_offset(1).is_some_and(|t| t.kind == TokenKind::Colon) {
            return self.parse_labeled();
        }

        if token.kind == TokenKind::At {
            if self.is_local_class_start() {
                return self.parse_local_class_decl();
            }
            return self.parse_var_decl();
        }

        if self.is_local_class_start() {
            return self.parse_local_class_decl();
        }

        if token.kind == TokenKind::Keyword {
            match self.peek().text.as_str() {
                "if" => return self.parse_if(),
                "while" => return self.parse_while(),
                "do" => return self.parse_do_while(),
                "for" => return self.parse_for(),
                "return" => return self.parse_return(),
                "break" => return self.parse_break(),
                "continue" => return self.parse_continue(),
                "throw" => return self.parse_throw(),
                "try" => return self.parse_try(),
                "switch" => return self.parse_switch(),
                "synchronized" => return self.parse_synchronized(),
                "assert" => return self.parse_assert(),
                "yield" => return self.parse_yield(),
                "class" | "interface" | "enum" => return self.parse_local_class_decl(),

                "int" | "boolean" | "char" | "byte" | "short" | "long" | "float" | "double" => {
                    return self.parse_var_decl();
                }
                _ => {}
            }
        }
        if self.is_variable_declaration_start() {
            return self.parse_var_decl();
        }
        self.parse_expr_stmt()
    }
}
