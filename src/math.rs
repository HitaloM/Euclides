pub fn addition(a: i64, b: i64) -> i64 {
    a + b
}

pub fn sub(a: i64, b: i64) -> i64 {
    a - b
}

pub fn multiply(multiplicand: i64, multiplier: i64) -> i64 {
    let mut result = 0;

    for _ in 0..multiplier.abs() {
        result += multiplicand;
    }

    if multiplier < 0 {
        result = -result;
    }

    result
}

pub fn divide(dividend: i64, divisor: i64) -> Result<(i64, i64), String> {
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

pub fn exponent(base: i64, exponent: i64) -> i64 {
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
