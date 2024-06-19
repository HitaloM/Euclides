use std::io::{self, Write};

pub mod basic_operations;

struct Stack<T> {
    elements: Vec<T>,
}

fn main() {
    println!(
        "Please enter an expression in Reverse Polish Notation, and I will evaluate it for you."
    );
    print!("Expression: ");
    io::stdout().flush().expect("Failed to flush stdout :/");

    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("There's something very wrong with your expression.");

    let mut stack: Stack<i64> = Stack {
        elements: Vec::new(),
    };

    for token in expression.split_whitespace() {
        match token.parse::<i64>() {
            Ok(value) => stack.elements.push(value),
            Err(_) => handle_operator(token, &mut stack),
        }
    }

    for element in stack.elements {
        println!("Result: {element}");
    }
}

fn handle_operator(operator: &str, stack: &mut Stack<i64>) {
    if stack.elements.len() < 2 {
        println!("Error: there are not enough operands on the stack for the operation.");
        return;
    }

    while stack.elements.len() > 1 {
        let n2 = stack.elements.pop().unwrap();
        let n1 = stack.elements.pop().unwrap();

        let result = match operator {
            "+" => Some(basic_operations::addition(n1, n2)),
            "-" => Some(basic_operations::subtraction(n1, n2)),
            "*" => Some(basic_operations::multiply(n1, n2)),
            "/" => match basic_operations::divide(n1, n2) {
                Ok(result) => Some(result.0),
                Err(e) => {
                    println!("{e}");
                    None
                }
            },
            _ => {
                println!("Unknown operator: {operator}");
                return;
            }
        };

        if let Some(value) = result {
            stack.elements.push(value);
        }
    }
}
