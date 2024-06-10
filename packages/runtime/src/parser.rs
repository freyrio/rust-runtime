// parser.rs
use std::fs;
use crate::ast::{Expr, ExprKind, Literal, BinaryOp, Statement, StatementKind, Program};

pub fn parse_file(file_path: &str) -> Program {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    parse(&contents, file_path)
}

fn parse(source: &str, file_name: &str) -> Program {
    let mut statements = Vec::new();
    let lines = source.lines().enumerate();

    for (line_number, line) in lines {
        if let Some(statement) = parse_statement(line, line_number + 1, file_name) {
            statements.push(statement);
        }
    }

    Program { statements }
}

fn parse_statement(line: &str, line_number: usize, file_name: &str) -> Option<Statement> {
    if line.starts_with("let") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[1].to_string();
        let expr = parse_expr(parts[3], line_number, file_name);
        Some(Statement {
            kind: StatementKind::VariableDeclaration(name, expr),
            line: line_number,
            file: file_name.to_string(),
        })
    } else if line.starts_with("fn") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[1].to_string();
        let args_str = &line[line.find('(').unwrap() + 1..line.find(')').unwrap()];
        let args = args_str.split(',').map(|arg| arg.trim().to_string()).collect();
        let body = Vec::new(); // We'll handle function bodies later
        Some(Statement {
            kind: StatementKind::FunctionDeclaration(name, args, body),
            line: line_number,
            file: file_name.to_string(),
        })
    } else if line.starts_with("println!") {
        let start = line.find('(').unwrap() + 1;
        let end = line.rfind(')').unwrap();
        let args_str = &line[start..end];
        let args = args_str.split(',').map(|arg| parse_expr(arg.trim(), line_number, file_name)).collect();
        Some(Statement {
            kind: StatementKind::PrintStatement(args),
            line: line_number,
            file: file_name.to_string(),
        })
    } else if !line.is_empty() {
        let expr = parse_expr(line, line_number, file_name);
        Some(Statement {
            kind: StatementKind::ExpressionStatement(expr),
            line: line_number,
            file: file_name.to_string(),
        })
    } else {
        None
    }
}

fn parse_expr(expr: &str, line_number: usize, file_name: &str) -> Expr {
    parse_binary_op(expr, line_number, file_name)
}

fn parse_binary_op(expr: &str, line_number: usize, file_name: &str) -> Expr {
    let mut parts = expr.split_whitespace();
    let mut left = parse_term(parts.next().unwrap(), line_number, file_name);

    while let Some(op) = parts.next() {
        let right = parse_term(parts.next().unwrap(), line_number, file_name);
        left = Expr {
            kind: ExprKind::BinaryOp(match op {
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Sub,
                "*" => BinaryOp::Mul,
                "/" => BinaryOp::Div,
                _ => panic!("Invalid operator: {}", op),
            }, Box::new(left), Box::new(right)),
            line: line_number,
            file: file_name.to_string(),
        };
    }

    left
}

fn parse_term(term: &str, line_number: usize, file_name: &str) -> Expr {
    if let Ok(int_value) = term.parse::<i64>() {
        Expr {
            kind: ExprKind::Literal(Literal::Int(int_value)),
            line: line_number,
            file: file_name.to_string(),
        }
    } else if let Ok(float_value) = term.parse::<f64>() {
        Expr {
            kind: ExprKind::Literal(Literal::Float(float_value)),
            line: line_number,
            file: file_name.to_string(),
        }
    } else if term.starts_with('"') && term.ends_with('"') {
        let str_value = term[1..term.len() - 1].to_string();
        Expr {
            kind: ExprKind::Literal(Literal::String(str_value)),
            line: line_number,
            file: file_name.to_string(),
        }
    } else if term.starts_with('[') && term.ends_with(']') {
        let elements_str = &term[1..term.len() - 1];
        let elements = elements_str.split(',').map(|elem| parse_expr(elem.trim(), line_number, file_name)).collect();
        Expr {
            kind: ExprKind::Literal(Literal::Array(elements)),
            line: line_number,
            file: file_name.to_string(),
        }
    } else if term.contains('(') {
        let name = term[..term.find('(').unwrap()].to_string();
        let args_str = &term[term.find('(').unwrap() + 1..term.find(')').unwrap()];
        let args = args_str.split(',').map(|arg| parse_expr(arg.trim(), line_number, file_name)).collect();
        Expr {
            kind: ExprKind::FunctionCall(name, args),
            line: line_number,
            file: file_name.to_string(),
        }
    } else {
        Expr {
            kind: ExprKind::Variable(term.to_string()),
            line: line_number,
            file: file_name.to_string(),
        }
    }
}