// examples/builtin_functions.rs
//! Examples demonstrating Aether's built-in functions

use aether::Aether;

fn main() {
    let mut engine = Aether::new();

    // Example 1: I/O Functions
    println!("=== I/O Functions ===");
    let code = r#"
        Println("Hello, World!")
        Println("Testing built-in functions")
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 2: Type Functions
    println!("\n=== Type Functions ===");
    let code = r#"
        Set X 42
        Set Y "Hello"
        Set Z [1, 2, 3]
        Println(Type(X))
        Println(Type(Y))
        Println(Type(Z))
        Println(ToString(X))
        Println(ToNumber("123"))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 3: Array Functions
    println!("\n=== Array Functions ===");
    let code = r#"
        Set NUMS Range(1, 6)
        Println(NUMS)
        Println(Len(NUMS))
        Println(Sum(NUMS))
        Println(Max(NUMS))
        Println(Min(NUMS))
        
        Set REVERSED Reverse(NUMS)
        Println(REVERSED)
        
        Set SORTED Sort([5, 2, 8, 1, 9])
        Println(SORTED)
        
        Set JOINED Join(NUMS, ", ")
        Println(JOINED)
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 4: String Functions
    println!("\n=== String Functions ===");
    let code = r#"
        Set TEXT "Hello World"
        Println(Upper(TEXT))
        Println(Lower(TEXT))
        Println(Contains(TEXT, "World"))
        Println(StartsWith(TEXT, "Hello"))
        Println(EndsWith(TEXT, "World"))
        
        Set PARTS Split(TEXT, " ")
        Println(PARTS)
        
        Set REPLACED Replace(TEXT, "World", "Aether")
        Println(REPLACED)
        
        Set REPEATED Repeat("Hi ", 3)
        Println(REPEATED)
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 5: Math Functions
    println!("\n=== Math Functions ===");
    let code = r#"
        Println(Abs(-42))
        Println(Floor(3.7))
        Println(Ceil(3.2))
        Println(Round(3.5))
        Println(Sqrt(16))
        Println(Pow(2, 10))
        Println(Exp(1))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 6: Dict Functions
    println!("\n=== Dict Functions ===");
    let code = r#"
        Set PERSON {"name": "Alice", "age": 30}
        Println(Keys(PERSON))
        Println(Values(PERSON))
        Println(Has(PERSON, "name"))
        Println(Has(PERSON, "email"))
        
        Set EXTRA {"email": "alice@example.com"}
        Set MERGED Merge(PERSON, EXTRA)
        Println(MERGED)
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 7: Combined Usage
    println!("\n=== Combined Usage ===");
    let code = r#"
        Set NUMBERS Range(1, 11)
        Println(Join(NUMBERS, ", "))
        
        Set NAME "aether programming"
        Set TITLE Upper(NAME)
        Println(Replace(TITLE, "PROGRAMMING", "LANGUAGE"))
        
        Set A 3
        Set B 4
        Set ASQUARED Pow(A, 2)
        Set BSQUARED Pow(B, 2)
        Set C Sqrt(ASQUARED + BSQUARED)
        Println(Round(C))
        Set D Sqrt(Pow(A,2) + Pow(B,2))
        Println(Round(D))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    println!("\n=== All built-in function examples completed! ===");
}
