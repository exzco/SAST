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
    pub class_scopes: HashMap<String, ScopeId>,
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
            class_scopes: HashMap::new(),
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
    pub fn set_scope(&mut self, id:ScopeId) {
        if id <= self.scopes.len() {
            self.current = id;
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
    // 记录类名对应的作用域 id
    pub fn bind_class_scope(&mut self, class_name: String, scope_id: ScopeId) {
        self.class_scopes.insert(class_name, scope_id);
    }

    // 根据类名检索该类的成员
    pub fn lookup_in_class(&self, class_name: &str, member_name: &str) -> Option<&Symbol> {
        let scope_id = self.class_scopes.get(class_name)?;
        let scope = self.scopes.get(*scope_id)?;
        let symbol_ids = scope.symbols.get(member_name)?;
        self.symbols.get(symbol_ids[0])
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
        // 1. 收集类、字段、方法签名
        for class in &c.classes {
            self.collect_class_symbols(class);
        }
        // 2. 分析内部代码
        for class in &c.classes {
            self.visit_class_body(class);
        }
    }
    pub fn resolve_type(&self, ty: &Type) -> ResolvedType {
        match ty {
            Type::Primitive(p) => ResolvedType::Primitive(p.clone()),
            Type::Reference { name, .. } => ResolvedType::Reference(name.clone()),
            Type::Array { element_type, dimensions } => {
                let mut current = self.resolve_type(element_type);
                for _ in 0..*dimensions {
                    current = ResolvedType::Array(Box::new(current));
                }
                current
            }
            Type::Var => ResolvedType::Unknown,
            Type::Wildcard { bound, .. } => {
                if let Some(b) = bound {
                    self.resolve_type(b)
                } else {
                    ResolvedType::Unknown
                }
            }
        }
    }


    pub fn collect_class_symbols(&mut self,class: &ClassDecl) {
        let class_name = class.name.clone();
        // 将类写入符号表
        self.symbol_table.define(
            class_name.clone(),
            SymbolKind::ClassName,
            ResolvedType::Reference((class_name.clone())),
        );
        // 绑定一个 scopeId 方便二次扫描时定位
        let class_scope_id = self.symbol_table.enter_scope();
        self.symbol_table.bind_class_scope(class_name, class_scope_id);

        // 收集类成员
        for member in &class.members {
            match member {
                ClassMember::Field(f) => self.collect_field_decl(f),
                ClassMember::Method(m) => self.collect_method_decl(m),
                ClassMember::Constructor(c) => self.collect_constructor_decl(c),
                ClassMember::Class(nested) => self.collect_class_symbols(nested),
                ClassMember::StaticInit(_) | ClassMember::InstanceInit(_) => {}
            }
        }
        self.symbol_table.exit_scope();
    }

    pub fn collect_field_decl(&mut self, field: &FieldDecl) {
        let field_type = self.resolve_type(&field.var_type);

        // 遍历 declarators, 给每个属性注册 Field 符号
        for decl in &field.declarators {
            self.symbol_table.define(
                decl.name.clone(),
                SymbolKind::Field,
                field_type.clone(),
            );
        }
    }

    pub fn collect_method_decl(&mut self,method: &MethodDecl) {
        let method_return_type = self.resolve_type(&method.return_type);

        let param_types: Vec<ResolvedType> = method.params.iter().map(|p| self.resolve_type(&p.var_type)).collect();
        let method_type: ResolvedType = ResolvedType::Method {
            param_types,
            return_type: Box::new(method_return_type),
        };

        self.symbol_table.define(
            method.name.clone(),
            SymbolKind::Method,
            method_type,
        );
    }

    pub fn collect_constructor_decl(&mut self, con: &ConstructorDecl) {
        let param_types = con.params.iter().map(|p| self.resolve_type(&p.var_type)).collect();
        let con_type = ResolvedType::Method { 
            param_types,
            return_type: Box::new(ResolvedType::Reference(con.name.clone())),
        };

        self.symbol_table.define(
            con.name.clone(),
            SymbolKind::Constructor,
            con_type,
        );
    }
    pub fn visit_class_body(&mut self, class: &ClassDecl) {
        if let Some(&scope_id) = self.symbol_table.class_scopes.get(&class.name) {
            self.symbol_table.set_scope(scope_id);

            for member in &class.members {
                match member {
                    ClassMember::Method(m) => self.visit_method_body(m),
                    ClassMember::Constructor(c) => self.visit_constructor_body(c),
                    ClassMember::Class(nested) => self.visit_class_body(nested),
                    ClassMember::StaticInit(stmt) | ClassMember::InstanceInit(stmt) => {
                        self.visit_stmt(stmt);
                    }
                    ClassMember::Field(_) => {}
                }
            }
            self.symbol_table.exit_scope();
        }
    }

    pub fn visit_constructor_body(&mut self, con: &ConstructorDecl) {
        self.symbol_table.enter_scope();
        for param in &con.params {
            let param_type = self.resolve_type(&param.var_type);
            self.symbol_table.define(
                param.name.clone(),
                SymbolKind::Param,
                param_type,
            );
        }
        self.visit_stmt(&con.body);
        self.symbol_table.exit_scope();
    }


    pub fn visit_method_body(&mut self, method: &MethodDecl) {
        self.symbol_table.enter_scope();
        for param in &method.params {
            let param_type = self.resolve_type(&param.var_type);
            self.symbol_table.define(
                param.name.clone(),
                SymbolKind::Param,
                param_type,
            );
        }
        if let Some(body) = &method.body {
            self.visit_stmt(body);
        }
        self.symbol_table.exit_scope();
    }

    pub fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            // 1. 代码块
            Stmt::Block { statements } => {
                self.symbol_table.enter_scope();
                for s in statements {
                    self.visit_stmt(s);
                }
                self.symbol_table.exit_scope();
            }

            // 2. 局部变量声明
            Stmt::VarDecl { var_type, declarators, .. } => {
                self.visit_var_decl_stmt(var_type, declarators);
            }

            // 3. 表达式语句
            Stmt::ExprStmt { expr } => {
                self.resolve_expr(expr);
            }

            // 4. 条件分支
            Stmt::If { cond, then_branch, else_branch } => {
                self.resolve_expr(cond);
                self.visit_stmt(then_branch);
                if let Some(else_stmt) = else_branch {
                    self.visit_stmt(else_stmt);
                }
            }

            // 5. 循环
            Stmt::While { cond, body } => {
                self.resolve_expr(cond);
                self.visit_stmt(body);
            }
            Stmt::DoWhile { condition, body } => {
                self.visit_stmt(body);
                self.resolve_expr(condition);
            }
            Stmt::For { init, condition, update, body } => {
                self.symbol_table.enter_scope();
                for s in init {
                    self.visit_stmt(s);
                }
                if let Some(cond) = condition {
                    self.resolve_expr(cond);
                }
                for expr in update {
                    self.resolve_expr(expr);
                }
                self.visit_stmt(body);
                self.symbol_table.exit_scope();
            }

            // 6. Return 解析返回表达式
            Stmt::Return { value } => {
                if let Some(expr) = value {
                    self.resolve_expr(expr);
                }
            }

            // 其余控制流语句（Empty, Break, Continue 等无需符号绑定行为）
            _ => {}
        }
    }

    pub fn visit_var_decl_stmt(&mut self, var_type: &Type, declarators: &[VarDeclarator]) {
        let resolved_type = self.resolve_type(var_type);

        for decl in declarators {
            let var_name = decl.name.clone();

            if let Some(init_expr) = &decl.init {
                self.resolve_expr(init_expr);
            }

            let current_scope_id = self.symbol_table.current;
            let already_exists = self.symbol_table.scopes[current_scope_id]
                .symbols
                .contains_key(&var_name);

            if already_exists {
                self.add_errors(format!(
                    "Variable '{}' is already defined in the current scope",
                    var_name
                ));
            } else {
                self.symbol_table.define(
                    var_name,
                    SymbolKind::LocalVar,
                    resolved_type.clone(),
                );
            }
        }
    }


}
