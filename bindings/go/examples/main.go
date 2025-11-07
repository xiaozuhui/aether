package main

import (
	"fmt"
	"log"

	aether "github.com/xiaozuhui/aether-go"
)

func main() {
	fmt.Println("=== Aether Go Bindings Examples ===")
	fmt.Println("Aether version:", aether.Version())
	fmt.Println()

	basicArithmetic()
	stringOperations()
	functions()
	controlFlow()
	arrays()
	fibonacci()
}

func basicArithmetic() {
	fmt.Println("--- Basic Arithmetic ---")
	engine := aether.New()
	defer engine.Close()

	code := `
		Set X 10
		Set Y 20
		Set SUM (X + Y)
		Set PRODUCT (X * Y)
		Print "X =", X
		Print "Y =", Y
		Print "Sum =", SUM
		Print "Product =", PRODUCT
		PRODUCT
	`

	result, err := engine.Eval(code)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Result: %s\n\n", result)
}

func stringOperations() {
	fmt.Println("--- String Operations ---")
	engine := aether.New()
	defer engine.Close()

	code := `
		Set FIRST "Hello"
		Set SECOND "Aether"
		Set GREETING (FIRST + " " + SECOND + "!")
		Print GREETING
		GREETING
	`

	result, err := engine.Eval(code)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Result: %s\n\n", result)
}

func functions() {
	fmt.Println("--- Functions ---")
	engine := aether.New()
	defer engine.Close()

	code := `
		Func ADD (A, B) {
			Return (A + B)
		}
		
		Func MULTIPLY (A, B) {
			Return (A * B)
		}
		
		Func CALCULATE (X, Y) {
			Set SUM ADD(X, Y)
			Set PROD MULTIPLY(X, Y)
			Print "Sum:", SUM
			Print "Product:", PROD
			Return PROD
		}
		
		CALCULATE(5, 6)
	`

	result, err := engine.Eval(code)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Final result: %s\n\n", result)
}

func controlFlow() {
	fmt.Println("--- Control Flow ---")
	engine := aether.New()
	defer engine.Close()

	code := `
		Func CHECK_NUMBER (N) {
			If (N > 0) {
				Return "positive"
			} Else {
				If (N < 0) {
					Return "negative"
				} Else {
					Return "zero"
				}
			}
		}
		
		Print CHECK_NUMBER(10)
		Print CHECK_NUMBER(-5)
		Print CHECK_NUMBER(0)
		
		CHECK_NUMBER(42)
	`

	result, err := engine.Eval(code)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Result: %s\n\n", result)
}

func arrays() {
	fmt.Println("--- Arrays ---")
	engine := aether.New()
	defer engine.Close()

	code := `
		Set NUMBERS [1, 2, 3, 4, 5]
		Set NAMES ["Alice", "Bob", "Charlie"]
		
		Print "Numbers:", NUMBERS
		Print "Length:", LENGTH(NUMBERS)
		Print "First:", FIRST(NUMBERS)
		Print "Last:", LAST(NUMBERS)
		
		Print "Names:", NAMES
		Print "Length:", LENGTH(NAMES)
		
		LENGTH(NUMBERS)
	`

	result, err := engine.Eval(code)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Result: %s\n\n", result)
}

func fibonacci() {
	fmt.Println("--- Fibonacci (Recursive) ---")
	engine := aether.New()
	defer engine.Close()

	code := `
		Func FIBONACCI (N) {
			If (N <= 1) {
				Return N
			}
			Return (FIBONACCI(N - 1) + FIBONACCI(N - 2))
		}
		
		Set RESULT FIBONACCI(10)
		Print "Fibonacci(10) =", RESULT
		RESULT
	`

	result, err := engine.Eval(code)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Result: %s\n\n", result)
}
