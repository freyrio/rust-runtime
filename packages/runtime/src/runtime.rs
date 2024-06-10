//runtime.rs
use std::collections::HashMap;
use crate::ast::{Expr, ExprKind, Literal, BinaryOp, Statement, StatementKind, Program};
use crate::debugger::Debugger;

pub struct Function {
    pub args: Vec<String>,
    pub body: Vec<Statement>,
}

pub struct Runtime {
    pub variables: HashMap<String, String>,
    pub functions: HashMap<String, Function>,
    pub debugger: Debugger,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
            functions: HashMap::new(),
            debugger: Debugger::new(),
        }
    }

    pub fn run(&mut self, program: Program) {
        for statement in program.statements {
            self.debugger.step(self, &statement);
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&mut self, statement: Statement) {
        match statement.kind {
            StatementKind::VariableDeclaration(name, expr) => {
                let value = self.eval_expr(expr);
                self.variables.insert(name, value);
            }
            StatementKind::FunctionDeclaration(name, args, body) => {
                self.functions.insert(name, Function { args, body });
            }
            StatementKind::PrintStatement(args) => {
                let mut formatted_args = Vec::new();
                for arg in args {
                    let value = self.eval_expr(arg);
                    formatted_args.push(value);
                }
                let output = formatted_args.join(", ");
                println!("{}", output);
            }
            StatementKind::ExpressionStatement(expr) => {
                self.eval_expr(expr);
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> String {
        match expr.kind {
            ExprKind::Literal(literal) => match literal {
                Literal::Int(value) => value.to_string(),
                Literal::Float(value) => value.to_string(),
                Literal::String(value) => value,
                Literal::Array(elements) => {
                    let mut array_str = String::new();
                    array_str.push('[');
                    let elements_str: Vec<String> = elements.into_iter().map(|elem| self.eval_expr(elem)).collect();
                    array_str.push_str(&elements_str.join(", "));
                    array_str.push(']');
                    array_str
                }
            },
            ExprKind::Variable(name) => self.variables.get(&name).unwrap().clone(),
            ExprKind::FunctionCall(name, args) => {
                let function = self.functions.get(&name).unwrap();
                let mut local_variables = HashMap::new();
                for (arg_name, arg_value) in function.args.iter().zip(args.into_iter()) {
                    let value = self.eval_expr(arg_value);
                    local_variables.insert(arg_name.clone(), value);
                }
                let prev_variables = self.variables.clone();
                self.variables = local_variables;
                for statement in &function.body {
                    self.execute_statement(statement.to_owned());
                }
                let result = self.variables.get("return").unwrap().clone();
                self.variables = prev_variables;
                result
            }
            ExprKind::BinaryOp(op, left, right) => {
                let left_value = self.eval_expr(*left);
                let right_value = self.eval_expr(*right);
                match op {
                    BinaryOp::Add => (left_value.parse::<f64>().unwrap() + right_value.parse::<f64>().unwrap()).to_string(),
                    BinaryOp::Sub => (left_value.parse::<f64>().unwrap() - right_value.parse::<f64>().unwrap()).to_string(),
                    BinaryOp::Mul => (left_value.parse::<f64>().unwrap() * right_value.parse::<f64>().unwrap()).to_string(),
                    BinaryOp::Div => (left_value.parse::<f64>().unwrap() / right_value.parse::<f64>().unwrap()).to_string(),
                }
            }
        }
    }
}