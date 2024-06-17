pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(multiplicand: i32, multiplier: i32) -> i32 {
    let mut result = 0;

    for _ in 0..multiplier.abs() {
        result += multiplicand;
    }

    if multiplier < 0 {
        result = -result;
    }

    result
}

pub fn divide(dividend: i32, divisor: i32) -> Result<(i32, i32), String> {
    if divisor == 0 {
        return Err("Cannot divide by zero".into());
    }

    let mut quotient = 0;
    let mut remainder = dividend;

    while remainder >= divisor {
        remainder -= divisor;
        quotient += 1;
    }

    Ok((quotient, remainder))
}

pub fn exponent(base: i32, exponent: i32) -> i32 {
    if base == 0 && exponent == 0 {
        return 1;
    }
    if exponent == 0 {
        return 1;
    }
    if base == 0 {
        return 0;
    }

    let mut result = 1;

    for _ in 0..exponent {
        let mut temp_result = 0;

        for _ in 0..base.abs() {
            temp_result += result;
        }

        result = temp_result;
    }

    if base < 0 && exponent % 2 != 0 {
        return -result;
    }

    result
}
