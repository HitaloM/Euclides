use std::io::{self, Write};

pub mod basic_operations;

struct Stack<T> {
    elements: Vec<T>,
}

fn main() {
    println!(
        "Please enter an expression in Reverse Polish Notation, and I will evaluate it for you."
    );

    loop {
        print!("Expression: ");
        io::stdout().flush().expect("Failed to flush stdout :/");

        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("There's something very wrong with your expression.");

        let mut stack: Stack<i64> = Stack {
            elements: Vec::new(),
        };

        let tokens = expression.split_whitespace();

        let mut contains_operator = false;

        for token in tokens {
            match token.parse::<i64>() {
                Ok(value) => stack.elements.push(value),
                Err(_) => {
                    contains_operator = true;
                    if let Err(_) = handle_operator(token, &mut stack) {
                        stack.elements.clear();
                        break;
                    }
                }
            }
        }

        if contains_operator && stack.elements.len() == 1 {
            println!("Result: {}", stack.elements.first().unwrap());
        } else if contains_operator {
            println!("Error: The expression is malformed or incomplete.");
        } else {
            println!("Warning: The expression does not contain any operators.");
        }
    }
}

fn handle_operator(operator: &str, stack: &mut Stack<i64>) -> Result<(), String> {
    if stack.elements.len() < 2 {
        return Err(
            "Error: There are not enough operands on the stack for the operation.".to_string(),
        );
    }

    let n2 = stack.elements.pop().unwrap();
    let n1 = stack.elements.pop().unwrap();

    let result = match operator {
        "+" => Some(basic_operations::addition(n1, n2)),
        "-" => Some(basic_operations::subtraction(n1, n2)),
        "*" => Some(basic_operations::multiply(n1, n2)),
        "/" => match basic_operations::divide(n1, n2) {
            Ok(result) => Some(result.0),
            Err(e) => {
                println!("{}", e);
                None
            }
        },
        _ => {
            return Err(format!("Unknown operator: {}", operator));
        }
    };

    if let Some(value) = result {
        stack.elements.push(value);
        Ok(())
    } else {
        Err("Error in operation.".to_string())
    }
}
