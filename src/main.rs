use std::io::{self, Write};

#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> Result<f64, &'static str> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b != 0.0 {
                Ok(a / b)
            } else {
                Err("Division by zero!")
            }
        }
    }
}

fn main() {
    println!("Simple Calculator");
    
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    print!("Enter operation (+, -, *, /): ");
    io::stdout().flush().unwrap();
    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    let operation = match op {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => panic!("Invalid operator"),
    };

    match calculate(operation) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}