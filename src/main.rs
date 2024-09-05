use std::collections::VecDeque;
use std::io::{self, Write};

pub mod basic_operations;

struct Stack<T> {
    elements: VecDeque<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: VecDeque::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.elements.push_back(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop_back()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn clear(&mut self) {
        self.elements.clear();
    }

    fn top(&self) -> Option<&T> {
        self.elements.back()
    }
}

enum Notation {
    Prefix,
    Postfix,
    Infix,
}

fn main() {
    println!("Please enter an expression in either Prefix, Postfix, or Infix Notation, and I will evaluate it for you.");
    println!("To specify the notation, use 'prefix:', 'postfix:', or 'infix:' at the beginning of your input.");

    loop {
        if let Some((notation, tokens)) = get_user_input() {
            let mut stack: Stack<i64> = Stack::new();
            let mut contains_operator = false;

            match notation {
                Notation::Postfix => {
                    evaluate_expression(&tokens, &mut stack, &mut contains_operator, false)
                }
                Notation::Prefix => {
                    evaluate_expression(&tokens, &mut stack, &mut contains_operator, true)
                }
                Notation::Infix => {
                    let postfix_tokens = infix_to_postfix(&tokens);
                    evaluate_expression(&postfix_tokens, &mut stack, &mut contains_operator, false);
                }
            }

            if contains_operator && stack.len() == 1 {
                println!("Result: {}", stack.top().unwrap());
            } else if contains_operator {
                println!("Error: The expression is malformed or incomplete.");
            } else {
                println!("Warning: The expression does not contain any operators.");
            }
        }
    }
}

fn get_user_input() -> Option<(Notation, Vec<String>)> {
    print!("Expression: ");
    io::stdout().flush().expect("Failed to flush stdout :/");

    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    let trimmed_expression = expression.trim();
    match trimmed_expression.to_lowercase().as_str() {
        expr if expr.starts_with("prefix:") => Some((Notation::Prefix, tokenize(&expr[7..]))),
        expr if expr.starts_with("postfix:") => Some((Notation::Postfix, tokenize(&expr[8..]))),
        expr if expr.starts_with("infix:") => Some((Notation::Infix, tokenize(&expr[6..]))),
        _ => {
            println!("Error: Please specify either 'prefix:', 'postfix:', or 'infix:' notation.");
            None
        }
    }
}

fn evaluate_expression(
    tokens: &[String],
    stack: &mut Stack<i64>,
    contains_operator: &mut bool,
    reverse: bool,
) {
    let token_iter: Box<dyn Iterator<Item = &String>> = if reverse {
        Box::new(tokens.iter().rev())
    } else {
        Box::new(tokens.iter())
    };

    for token in token_iter {
        match token.parse::<i64>() {
            Ok(value) => stack.push(value),
            Err(_) => {
                *contains_operator = true;
                if handle_operator(token, stack, reverse).is_err() {
                    stack.clear();
                    break;
                }
            }
        }
    }
}

fn handle_operator(operator: &str, stack: &mut Stack<i64>, reverse: bool) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Error: Not enough operands on the stack for the operation.".to_string());
    }

    let n2 = stack.pop().unwrap();
    let n1 = stack.pop().unwrap();

    let (left, right) = if reverse { (n2, n1) } else { (n1, n2) };

    let result = match operator {
        "+" => Some(basic_operations::addition(left, right)),
        "-" => Some(basic_operations::subtraction(left, right)),
        "*" => Some(basic_operations::multiply(left, right)),
        "/" => match basic_operations::divide(left, right) {
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
        stack.push(value);
        Ok(())
    } else {
        Err("Error in operation.".to_string())
    }
}

fn tokenize(expression: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for c in expression.chars() {
        if c.is_whitespace() {
            if !current_token.is_empty() {
                tokens.push(std::mem::take(&mut current_token));
            }
        } else if c.is_ascii_digit() {
            current_token.push(c);
        } else {
            if !current_token.is_empty() {
                tokens.push(std::mem::take(&mut current_token));
            }
            tokens.push(c.to_string());
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

fn infix_to_postfix(tokens: &[String]) -> Vec<String> {
    let mut output = Vec::new();
    let mut operators = VecDeque::new();

    let precedence = |op: &str| -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    };

    for token in tokens {
        if token.parse::<i64>().is_ok() {
            output.push(token.clone());
        } else if token == "(" {
            operators.push_back(token.clone());
        } else if token == ")" {
            while let Some(op) = operators.pop_back() {
                if op == "(" {
                    break;
                }
                output.push(op);
            }
        } else {
            while let Some(op) = operators.back() {
                if precedence(op) > precedence(token) || (precedence(op) == precedence(token)) {
                    output.push(operators.pop_back().unwrap());
                } else {
                    break;
                }
            }
            operators.push_back(token.clone());
        }
    }

    while let Some(op) = operators.pop_back() {
        output.push(op);
    }

    output
}
