mod ast;
mod debugger;
mod parser;
mod runtime;

use std::env;
use runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file> [--debug]", args[0]);
        return;
    }

    let file_path = &args[1];
    let enable_debug = args.len() > 2 && args[2] == "--debug";

    let program = parser::parse_file(file_path);
    let mut runtime = Runtime::new();
    runtime.debugger.enable = enable_debug;
    runtime.run(program);
}