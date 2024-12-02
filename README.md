```markdown
# Simple Rust Calculator

A command-line calculator implemented in Rust that performs basic arithmetic operations using enums and pattern matching.

## Features

- Basic arithmetic operations (+, -, *, /)
- Floating-point number support
- Error handling for division by zero
- User-friendly command-line interface

## Setup

1. Clone the repository
2. Navigate to project directory
3. Run the following commands:

```bash
cargo init
cargo build
cargo run
```

## Usage

The calculator prompts for:
1. First number
2. Operation (+, -, *, /)
3. Second number

Example:
```
Enter first number: 10
Enter operation (+, -, *, /): +
Enter second number: 5
Result: 15
```

## Error Handling

- Handles division by zero
- Validates operator input
- Manages invalid number inputs
```
