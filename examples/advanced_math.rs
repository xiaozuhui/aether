// examples/advanced_math.rs
//! Examples demonstrating Aether's advanced math and NumPy-like operations

use aether::Aether;

fn main() {
    let mut engine = Aether::new();

    // Example 1: Advanced Trigonometry
    println!("=== Advanced Trigonometry ===");
    let code = r#"
        Println("Arc functions:")
        Println(Asin(0.5))
        Println(Acos(0.5))
        Println(Atan(1.0))
        Println(Atan2(1.0, 1.0))
        
        Println("Hyperbolic functions:")
        Println(Sinh(1.0))
        Println(Cosh(1.0))
        Println(Tanh(1.0))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 2: Special Mathematical Functions
    println!("\n=== Special Functions ===");
    let code = r#"
        Println("Factorial: 5! = " + ToString(Factorial(5)))
        Println("Gamma(5) = " + ToString(Gamma(5)))
        Println("Erf(1.0) = " + ToString(Erf(1.0)))
        Println("Hypot(3, 4) = " + ToString(Hypot(3, 4)))
        Println("Sign(-5) = " + ToString(Sign(-5)))
        Println("Clamp(15, 0, 10) = " + ToString(Clamp(15, 0, 10)))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 3: Statistics
    println!("\n=== Statistics ===");
    let code = r#"
        Set DATA [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        Println("Data: " + Join(DATA, ", "))
        Println("Mean: " + ToString(Mean(DATA)))
        Println("Median: " + ToString(Median(DATA)))
        Println("Std Dev: " + ToString(Std(DATA)))
        Println("Variance: " + ToString(Variance(DATA)))
        Println("25th percentile: " + ToString(Quantile(DATA, 0.25)))
        Println("75th percentile: " + ToString(Quantile(DATA, 0.75)))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 4: Vector Operations (NumPy-like)
    println!("\n=== Vector Operations ===");
    let code = r#"
        Set V1 [1, 2, 3]
        Set V2 [4, 5, 6]
        
        Println("v1 = " + Join(V1, ", "))
        Println("v2 = " + Join(V2, ", "))
        Println("Dot product: " + ToString(Dot(V1, V2)))
        Println("Norm of v1: " + ToString(Norm(V1)))
        Println("Distance: " + ToString(Distance(V1, V2)))
        
        Set V1_NORM Normalize(V1)
        Println("Normalized v1: " + Join(V1_NORM, ", "))
        
        Set CROSS Cross(V1, V2)
        Println("Cross product: " + Join(CROSS, ", "))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 5: Matrix Operations
    println!("\n=== Matrix Operations ===");
    let code = r#"
        Set M1 [[1, 2], [3, 4]]
        Set M2 [[5, 6], [7, 8]]
        
        Println("Matrix multiplication:")
        Set RESULT Matmul(M1, M2)
        Set ROW1 RESULT[0]
        Set ROW2 RESULT[1]
        Println("[" + Join(ROW1, ", ") + "]")
        Println("[" + Join(ROW2, ", ") + "]")
        
        Println("Transpose:")
        Set TRANS Transpose(M1)
        Set T_ROW1 TRANS[0]
        Set T_ROW2 TRANS[1]
        Println("[" + Join(T_ROW1, ", ") + "]")
        Println("[" + Join(T_ROW2, ", ") + "]")
        
        Println("Determinant: " + ToString(Determinant(M1)))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 6: 3D Geometry
    println!("\n=== 3D Geometry ===");
    let code = r#"
        Set M3 [[1, 0, 2], [0, 3, 1], [2, 1, 0]]
        Println("3x3 Determinant: " + ToString(Determinant(M3)))
        
        Set P1 [1, 0, 0]
        Set P2 [0, 1, 0]
        Set P3 [0, 0, 1]
        
        Println("Triangle vertices:")
        Println("P1: " + Join(P1, ", "))
        Println("P2: " + Join(P2, ", "))
        Println("P3: " + Join(P3, ", "))
        
        Set EDGE1 [(-1), 1, 0]
        Set EDGE2 [(-1), 0, 1]
        Set NORMAL Cross(EDGE1, EDGE2)
        Println("Normal vector: " + Join(NORMAL, ", "))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 7: Mathematical Constants
    println!("\n=== Mathematical Constants ===");
    let code = r#"
        Println("PI = " + ToString(PI()))
        Println("E = " + ToString(E()))
        Println("TAU = " + ToString(TAU()))
        Println("PHI (Golden Ratio) = " + ToString(PHI()))
        
        Set CIRCLE_AREA (PI() * Pow(5, 2))
        Println("Area of circle (r=5): " + ToString(CIRCLE_AREA))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 8: Complex Calculations
    println!("\n=== Complex Calculations ===");
    let code = r#"
        Set X 1.96
        Set SQRT2 Sqrt(2)
        Set ERF_VAL Erf(X / SQRT2)
        Set CDF ((1 + ERF_VAL) / 2)
        Println("P(Z <= 1.96) ≈ " + ToString(CDF))
        
        Set A 3
        Set B 4
        Set C 5
        Set A_SQ Pow(A, 2)
        Set B_SQ Pow(B, 2)
        Set C_SQ Pow(C, 2)
        Set DIAGONAL Sqrt(A_SQ + B_SQ + C_SQ)
        Println("3D diagonal of (3,4,5) box: " + ToString(DIAGONAL))
        
        Set PRINCIPAL 1000
        Set RATE 0.05
        Set YEARS 10
        Set EXPONENT (RATE * YEARS)
        Set AMOUNT (PRINCIPAL * Exp(EXPONENT))
        Println("Continuous compound interest: $" + ToString(Round(AMOUNT)))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    // Example 9: Data Analysis Pipeline
    println!("\n=== Data Analysis Pipeline ===");
    let code = r#"
        Set SAMPLES [23, 45, 12, 67, 34, 89, 56, 43, 21, 78]
        Println("Raw data: " + Join(SAMPLES, ", "))
        
        Set AVG Mean(SAMPLES)
        Set SD Std(SAMPLES)
        Println("Mean: " + ToString(Round(AVG)))
        Println("Std Dev: " + ToString(Round(SD)))
        
        Set SORTED Sort(SAMPLES)
        Set Q1 Quantile(SORTED, 0.25)
        Set Q2 Quantile(SORTED, 0.50)
        Set Q3 Quantile(SORTED, 0.75)
        Println("Q1: " + ToString(Q1))
        Println("Q2 (Median): " + ToString(Q2))
        Println("Q3: " + ToString(Q3))
        Println("IQR: " + ToString(Q3 - Q1))
    "#;
    if let Err(e) = engine.eval(code) {
        eprintln!("Error: {}", e);
    }

    println!("\n=== All advanced math examples completed! ===");
}
