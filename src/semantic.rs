use std::collections::HashMap;
use crate::parser::*;

type ScopeId = usize;
type SymbolId = usize;

#[derive(Debug,Clone,PartialEq)]
pub enum ResolvedType {
    Primitive(PrimitiveType),
    Reference(String),
    Array(Box<ResolvedType>),
    Method {
        param_types: Vec<ResolvedType>,
        return_type: Box<ResolvedType>
    },
    Null,
    Unknown,
}

#[derive(Debug,Clone,PartialEq)]
pub enum SymbolKind {
    LocalVar,
    Field,
    Param,
    Method,
    Constructor,
    ClassName,
}

#[derive(Debug,Clone)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: String,
    pub kind: SymbolKind,
    pub resolved_type: ResolvedType, //  token 那里解析出的类型
}

#[derive(Debug)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub symbols: HashMap<String,Vec<SymbolId>>, 
}

pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    pub symbols: Vec<Symbol>,
    pub current: ScopeId,
}

impl SymbolTable {
    pub fn new() -> Self {
        let root = Scope {
            id: 0,
            parent: None,
            symbols: HashMap::new(),
        };
        return SymbolTable {
            scopes: vec![root],
            current: 0,
            symbols: Vec::new(),
        };
    }
    pub fn enter_scope(&mut self) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(Scope { id, parent: Some(self.current), symbols: HashMap::new() });
        self.current = id;
        return id;
    }
    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current].parent {
            self.current = parent;
        }
    }
    // 向上回溯作用域查找符号，变量，方法调用
    pub fn lookup(&mut self, name: &str) -> Option<&Symbol> {
        let mut current_id = Some(self.current);
        while let Some(scope_id) = current_id {
            let scope = &self.scopes[scope_id];
            if let Some(symbol_ids) = scope.symbols.get(name) {
                return self.symbols.get(symbol_ids[0]);
            }
            current_id = scope.parent;
        }
        None
    }
    pub fn define(&mut self, name: String, kind: SymbolKind, resolved_type: ResolvedType) -> SymbolId {
        let id = self.symbols.len();
        let sym = Symbol {id,name:name.clone(),kind,resolved_type};
        self.symbols.push(sym);
        let scope = &mut self.scopes[self.current];
        scope.symbols.entry(name).or_insert_with(Vec::new).push(id);
        id
    }
}

pub struct ErrorMessage {
    pub message: String,
}
pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    pub errors: Vec<ErrorMessage>,
    pub expect_con: Option<String>,
    pub current_con: Option<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            expect_con: None,
            current_con: None,
        }
    }

    pub fn add_errors(&mut self,msg: impl Into<String>) {
        self.errors.push(ErrorMessage { message: msg.into() });
    }
    pub fn run(&mut self,c: CompilationUnit) {
        for class in &c.classes {
            self.visit_class(class);
        }
    }
    pub fn visit_class(&mut self,class: &ClassDecl) {
        
    }

}