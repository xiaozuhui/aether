//! Example: Using the Aether DSL

use aether::Aether;

fn main() {
    let mut engine = Aether::new();

    println!("=== Aether Language Examples ===\n");

    // Example 1: Basic arithmetic
    println!("1. Basic Arithmetic:");
    let code1 = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;
    match engine.eval(code1) {
        Ok(result) => println!("   10 + 20 = {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 2: Functions
    println!("2. Function Definition:");
    let code2 = r#"
        Func ADD (A, B) {
            Return (A + B)
        }
        
        Func MULTIPLY (A, B) {
            Return (A * B)
        }
        
        ADD(5, 3)
    "#;
    match engine.eval(code2) {
        Ok(result) => println!("   ADD(5, 3) = {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 3: Arrays
    println!("3. Arrays:");
    let code3 = r#"
        Set NUMBERS [1, 2, 3, 4, 5]
        NUMBERS[2]
    "#;
    match engine.eval(code3) {
        Ok(result) => println!("   NUMBERS[2] = {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 4: For loops
    println!("4. For Loop (Sum of array):");
    let code4 = r#"
        Set SUM 0
        For NUM In [1, 2, 3, 4, 5] {
            Set SUM (SUM + NUM)
        }
        SUM
    "#;
    match engine.eval(code4) {
        Ok(result) => println!("   Sum of [1,2,3,4,5] = {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 5: Conditionals
    println!("5. If-Else:");
    let code5 = r#"
        Set AGE 25
        
        If (AGE >= 18) {
            Set STATUS "Adult"
        } Else {
            Set STATUS "Minor"
        }
        
        STATUS
    "#;
    match engine.eval(code5) {
        Ok(result) => println!("   Status: {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 6: Recursion
    println!("6. Recursive Factorial:");
    let code6 = r#"
        Func FACTORIAL (N) {
            If (N <= 1) {
                Return 1
            } Else {
                Return (N * FACTORIAL((N - 1)))
            }
        }
        
        FACTORIAL(5)
    "#;
    match engine.eval(code6) {
        Ok(result) => println!("   5! = {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 7: Nested functions
    println!("7. Nested Function Calls:");
    let code7 = r#"
        Func SQUARE (X) {
            Return (X * X)
        }
        
        Func SUM_OF_SQUARES (A, B) {
            Return (SQUARE(A) + SQUARE(B))
        }
        
        SUM_OF_SQUARES(3, 4)
    "#;
    match engine.eval(code7) {
        Ok(result) => println!("   3² + 4² = {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 8: String concatenation
    println!("8. String Operations:");
    let code8 = r#"
        Set FIRST "Hello"
        Set LAST "World"
        (FIRST + " " + LAST)
    "#;
    match engine.eval(code8) {
        Ok(result) => println!("   Result: {}\n", result),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    println!("=== All examples completed! ===");
}
