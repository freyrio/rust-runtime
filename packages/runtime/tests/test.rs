// tests/test.rs
use rust_runtime::parser;
use rust_runtime::runtime::Runtime;

#[test]
fn test_basic_program() {
    let source_code = r#"
        fn main() {
            let x = 10;
            let y = 20;
            println!("x + y = {}", x + y);
        }
    "#;

    let program = parser::parse(source_code, "test.rs");
    let mut runtime = Runtime::new();
    runtime.run(program);
    // Add assertions to check the expected output or behavior
}

// Add more test functions for different scenarios