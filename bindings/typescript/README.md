# Aether TypeScript/JavaScript Bindings

TypeScript and JavaScript bindings for the Aether DSL, compiled to WebAssembly.

## Installation

```bash
npm install @xiaozuhui/aether
```

Or with Yarn:

```bash
yarn add @xiaozuhui/aether
```

## Usage

### Basic Example

```typescript
import { Aether } from '@xiaozuhui/aether';

async function main() {
  // Create a new Aether engine
  const engine = await Aether.create();
  
  // Evaluate some code
  const result = engine.eval(`
    Set X 10
    Set Y 20
    (X + Y)
  `);
  
  console.log(result); // Output: 30
}

main();
```

### JavaScript (CommonJS)

```javascript
const { Aether } = require('@xiaozuhui/aether');

async function main() {
  const engine = await Aether.create();
  
  const result = engine.eval(`
    Set GREETING "Hello"
    Set NAME "World"
    (GREETING + " " + NAME)
  `);
  
  console.log(result); // Output: "Hello World"
}

main();
```

### JavaScript (ES Modules)

```javascript
import { Aether } from '@xiaozuhui/aether';

const engine = await Aether.create();

const result = engine.eval(`
  Func FACTORIAL (N) {
    If (N <= 1) {
      Return 1
    }
    Return (N * FACTORIAL(N - 1))
  }
  
  FACTORIAL(5)
`);

console.log(result); // Output: 120
```

### Working with Different Value Types

```typescript
const engine = await Aether.create();

// Numbers
const num = engine.eval('(10 + 20)');
console.log(num); // 30

// Strings
const str = engine.eval('"Hello " + "Aether"');
console.log(str); // "Hello Aether"

// Booleans
const bool = engine.eval('(10 > 5)');
console.log(bool); // true

// Arrays
const arr = engine.eval('[1, 2, 3, 4, 5]');
console.log(arr); // [1, 2, 3, 4, 5]

// Objects (Dictionaries)
const obj = engine.eval(`
  Set PERSON {"name": "Alice", "age": 30}
  PERSON
`);
console.log(obj); // { name: "Alice", age: 30 }
```

### Using with IO Permissions

```typescript
// Create engine with IO permissions enabled
const engine = await Aether.createWithPermissions();

const result = engine.eval(`
  Set DATA (READ_FILE("data.txt"))
  Print "Data:", DATA
  DATA
`);
```

### Quick Evaluation

For one-off evaluations, you can use the convenience function:

```typescript
import { evalCode } from '@xiaozuhui/aether';

const result = await evalCode('(10 + 20)');
console.log(result); // 30
```

Note: This creates a new engine for each call. For multiple evaluations,
create an engine instance instead:

```typescript
const engine = await Aether.create();
const result1 = engine.eval('Set X 10');
const result2 = engine.eval('(X + 20)'); // Can access X from previous eval
```

## API Reference

### `Aether.create(): Promise<Aether>`

Creates a new Aether engine with default (restricted) IO permissions.

### `Aether.createWithPermissions(): Promise<Aether>`

Creates a new Aether engine with all IO permissions enabled.

⚠️ **Warning**: Only use this when you trust the scripts being executed.

### `engine.eval(code: string): AetherValue`

Evaluates Aether code and returns the result.

**Parameters:**

- `code`: The Aether code to evaluate

**Returns:** The evaluation result (number, string, boolean, array, object, or null)

**Throws:** Error if the code fails to parse or encounters a runtime error

### `Aether.version(): string`

Returns the version string of the Aether engine.

### `evalCode(code: string): Promise<AetherValue>`

Convenience function to evaluate code without creating an engine instance.

## Types

```typescript
type AetherValue = 
  | number 
  | string 
  | boolean 
  | AetherValue[] 
  | { [key: string]: AetherValue } 
  | null;
```

## Building from Source

```bash
# Install dependencies
npm install

# Build WASM module and TypeScript
npm run build

# Run tests
npm test
```

### Prerequisites

- Rust (with `wasm-pack` installed)
- Node.js 16+
- npm or yarn

## Browser Support

This package uses WebAssembly and requires a modern browser:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Node.js Support

Requires Node.js 16 or higher with WebAssembly support.

## Examples

See the [examples](./examples) directory for more usage examples:

- Basic arithmetic
- String operations
- Functions
- Control flow
- Arrays and objects
- Recursive functions

## License

Apache-2.0

## Contributing

Contributions are welcome! Please see the main repository for guidelines.

## Links

- [GitHub Repository](https://github.com/xiaozuhui/aether)
- [Documentation](https://github.com/xiaozuhui/aether/tree/master/docs)
- [Report Issues](https://github.com/xiaozuhui/aether/issues)
