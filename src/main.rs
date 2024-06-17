use math::divide;

pub mod math;

fn main() {
    println!("Hello, world!");

    let a = 6;
    let b = 2;

    let result = divide(a, b);
    match result {
        Ok(value) => {
            println!("Result: {}", value.0);
            println!("Remainder: {}", value.1);
        }
        Err(e) => println!("Error: {}", e),
    }
}
