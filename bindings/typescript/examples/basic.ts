/**
 * Basic examples for Aether TypeScript bindings
 */

import { Aether, evalCode } from '../src/index';

async function basicArithmetic() {
    console.log('\n=== Basic Arithmetic ===');
    const engine = await Aether.create();

    const result = engine.eval(`
    Set X 10
    Set Y 20
    Set SUM (X + Y)
    Set PRODUCT (X * Y)
    Print "Sum:", SUM
    Print "Product:", PRODUCT
    PRODUCT
  `);

    console.log('Result:', result);
}

async function stringOperations() {
    console.log('\n=== String Operations ===');
    const engine = await Aether.create();

    const result = engine.eval(`
    Set GREETING "Hello"
    Set NAME "Aether"
    (GREETING + " " + NAME + "!")
  `);

    console.log('Result:', result);
}

async function functions() {
    console.log('\n=== Functions ===');
    const engine = await Aether.create();

    const result = engine.eval(`
    Func ADD (A, B) {
      Return (A + B)
    }
    
    Func MULTIPLY (A, B) {
      Return (A * B)
    }
    
    Set X ADD(5, 3)
    Set Y MULTIPLY(4, 7)
    Print "Addition:", X
    Print "Multiplication:", Y
    (X + Y)
  `);

    console.log('Result:', result);
}

async function controlFlow() {
    console.log('\n=== Control Flow ===');
    const engine = await Aether.create();

    const result = engine.eval(`
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
    CHECK_NUMBER(0)
  `);

    console.log('Result:', result);
}

async function arrays() {
    console.log('\n=== Arrays ===');
    const engine = await Aether.create();

    const result = engine.eval(`
    Set NUMBERS [1, 2, 3, 4, 5]
    Set NAMES ["Alice", "Bob", "Charlie"]
    
    Print "Numbers:", NUMBERS
    Print "Length:", LENGTH(NUMBERS)
    Print "First:", FIRST(NUMBERS)
    Print "Last:", LAST(NUMBERS)
    
    LENGTH(NAMES)
  `);

    console.log('Result:', result);
}

async function fibonacci() {
    console.log('\n=== Fibonacci (Recursive) ===');
    const engine = await Aether.create();

    const result = engine.eval(`
    Func FIBONACCI (N) {
      If (N <= 1) {
        Return N
      }
      Return (FIBONACCI(N - 1) + FIBONACCI(N - 2))
    }
    
    Set RESULT FIBONACCI(10)
    Print "Fibonacci(10):", RESULT
    RESULT
  `);

    console.log('Result:', result);
}

async function quickEval() {
    console.log('\n=== Quick Eval ===');

    // Using the convenience function
    const result = await evalCode('(15 + 15)');
    console.log('Quick eval result:', result);
}

// Run all examples
async function main() {
    console.log('Aether TypeScript Examples');
    console.log('Version:', Aether.version());

    try {
        await basicArithmetic();
        await stringOperations();
        await functions();
        await controlFlow();
        await arrays();
        await fibonacci();
        await quickEval();

        console.log('\n✅ All examples completed successfully!');
    } catch (error) {
        console.error('❌ Error:', error);
    }
}

// Export main function for use as a module
export { main };

// Auto-run when executed directly
main().catch(console.error);
