// ast.rs
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub line: usize,
    pub file: String,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(Literal),
    Variable(String),
    FunctionCall(String, Vec<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub line: usize,
    pub file: String,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    VariableDeclaration(String, Expr),
    FunctionDeclaration(String, Vec<String>, Vec<Statement>),
    PrintStatement(Vec<Expr>),
    ExpressionStatement(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}