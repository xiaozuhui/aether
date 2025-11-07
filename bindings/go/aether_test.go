package aether

import (
	"strings"
	"testing"
)

func TestNew(t *testing.T) {
	engine := New()
	if engine == nil {
		t.Fatal("New() returned nil")
	}
	defer engine.Close()
}

func TestVersion(t *testing.T) {
	version := Version()
	if version == "" {
		t.Error("Version() returned empty string")
	}
	t.Logf("Aether version: %s", version)
}

func TestBasicArithmetic(t *testing.T) {
	engine := New()
	defer engine.Close()

	tests := []struct {
		name     string
		code     string
		expected string
	}{
		{
			name:     "addition",
			code:     "Set X 10\nSet Y 20\n(X + Y)",
			expected: "30",
		},
		{
			name:     "subtraction",
			code:     "(50 - 20)",
			expected: "30",
		},
		{
			name:     "multiplication",
			code:     "(6 * 5)",
			expected: "30",
		},
		{
			name:     "division",
			code:     "(60 / 2)",
			expected: "30",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := engine.Eval(tt.code)
			if err != nil {
				t.Fatalf("Eval() error = %v", err)
			}
			if result != tt.expected {
				t.Errorf("Expected %s, got %s", tt.expected, result)
			}
		})
	}
}

func TestStringOperations(t *testing.T) {
	engine := New()
	defer engine.Close()

	code := `
		Set GREETING "Hello"
		Set NAME "World"
		(GREETING + " " + NAME)
	`

	result, err := engine.Eval(code)
	if err != nil {
		t.Fatalf("Eval() error = %v", err)
	}

	if result != "Hello World" {
		t.Errorf("Expected 'Hello World', got %s", result)
	}
}

func TestFunctionDefinition(t *testing.T) {
	engine := New()
	defer engine.Close()

	code := `
		Func ADD (A, B) {
			Return (A + B)
		}
		
		ADD(15, 15)
	`

	result, err := engine.Eval(code)
	if err != nil {
		t.Fatalf("Eval() error = %v", err)
	}

	if result != "30" {
		t.Errorf("Expected 30, got %s", result)
	}
}

func TestIfStatement(t *testing.T) {
	engine := New()
	defer engine.Close()

	code := `
		Set X 30
		If (X > 20) {
			Return "big"
		} Else {
			Return "small"
		}
	`

	result, err := engine.Eval(code)
	if err != nil {
		t.Fatalf("Eval() error = %v", err)
	}

	if result != "big" {
		t.Errorf("Expected 'big', got %s", result)
	}
}

func TestRecursiveFunction(t *testing.T) {
	engine := New()
	defer engine.Close()

	code := `
		Func FACTORIAL (N) {
			If (N <= 1) {
				Return 1
			}
			Return (N * FACTORIAL(N - 1))
		}
		
		FACTORIAL(5)
	`

	result, err := engine.Eval(code)
	if err != nil {
		t.Fatalf("Eval() error = %v", err)
	}

	if result != "120" {
		t.Errorf("Expected 120, got %s", result)
	}
}

func TestArrayOperations(t *testing.T) {
	engine := New()
	defer engine.Close()

	code := `
		Set ARR [1, 2, 3, 4, 5]
		LENGTH(ARR)
	`

	result, err := engine.Eval(code)
	if err != nil {
		t.Fatalf("Eval() error = %v", err)
	}

	if result != "5" {
		t.Errorf("Expected 5, got %s", result)
	}
}

func TestRuntimeError(t *testing.T) {
	engine := New()
	defer engine.Close()

	code := `UNDEFINED_VARIABLE`

	_, err := engine.Eval(code)
	if err == nil {
		t.Error("Expected error for undefined variable, got nil")
	}

	if !strings.Contains(err.Error(), "Runtime error") {
		t.Errorf("Expected runtime error, got: %v", err)
	}
}

func TestParseError(t *testing.T) {
	engine := New()
	defer engine.Close()

	code := `Set X`

	_, err := engine.Eval(code)
	if err == nil {
		t.Error("Expected parse error, got nil")
	}
}

func TestCloseMultipleTimes(t *testing.T) {
	engine := New()
	engine.Close()
	engine.Close() // Should not panic
}

func TestEvalAfterClose(t *testing.T) {
	engine := New()
	engine.Close()

	_, err := engine.Eval("Set X 10")
	if err == nil {
		t.Error("Expected error when evaluating after close, got nil")
	}
}

func BenchmarkEval(b *testing.B) {
	engine := New()
	defer engine.Close()

	code := `
		Func FIB (N) {
			If (N <= 1) {
				Return N
			}
			Return (FIB(N - 1) + FIB(N - 2))
		}
		FIB(10)
	`

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := engine.Eval(code)
		if err != nil {
			b.Fatal(err)
		}
	}
}
