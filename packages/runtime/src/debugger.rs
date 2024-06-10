// debugger.rs
use crate::runtime::Runtime;
use crate::ast::Statement;
use std::io::{self, Write};

pub struct Debugger {
    pub enable: bool,
    pub breakpoints: Vec<usize>,
    pub current_line: usize,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            enable: false,
            breakpoints: Vec::new(),
            current_line: 0,
        }
    }

    pub fn step(&mut self, runtime: &Runtime, statement: &Statement) {
        if self.enable {
            println!("Line: {}, File: {}", statement.line, statement.file);
            println!("Statement: {:?}", statement.kind);
            println!("Variables: {:?}", runtime.variables);

            loop {
                print!("Debug> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                match input {
                    "n" => break,
                    "c" => {
                        self.enable = false;
                        break;
                    }
                    "p" => {
                        let mut args = input.split_whitespace();
                        args.next();
                        if let Some(var_name) = args.next() {
                            if let Some(value) = runtime.variables.get(var_name) {
                                println!("{}: {}", var_name, value);
                            } else {
                                println!("Variable not found: {}", var_name);
                            }
                        } else {
                            println!("Usage: p <variable>");
                        }
                    }
                    _ => {
                        println!("Unknown command. Available commands:");
                        println!("  n - step to the next line");
                        println!("  c - continue execution");
                        println!("  p <variable> - print the value of a variable");
                    }
                }
            }
        }
    }
}