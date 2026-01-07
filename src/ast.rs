// ArcScript AST (MVP subset)

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Member {
        object: Box<Expr>,
        field: String,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    TableLiteral(Vec<TableField>),
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum TableField {
    KeyValue { key: String, value: Expr },
    Value(Expr),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl {
        name: String,
        init: Expr,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    Expr(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        elif_branches: Vec<(Expr, Stmt)>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        var_name: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        body: Box<Stmt>,
    },
    Break,
    Continue,
    Return(Option<Expr>),
    FuncDecl(FuncDecl),
    ObjectDecl(ObjectDecl),
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct EventDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct ObjectDecl {
    pub name: String,
    pub members: Vec<ObjectMember>,
}

#[derive(Debug, Clone)]
pub enum ObjectMember {
    Var(Stmt),
    Method(FuncDecl),
    Event(EventDecl),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}
