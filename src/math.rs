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

    if (multiplicand < 0 && multiplier > 0) || (multiplicand > 0 && multiplier < 0) {
        result = -result;
    }

    result
}

pub fn divide(dividend: i64, divisor: i64) -> Result<(i64, i64), String> {
    if divisor == 0 {
        return Err("Cannot divide by zero".into());
    }

    let mut quotient = 0;
    let mut remainder = dividend.abs();

    while remainder >= divisor.abs() {
        remainder -= divisor.abs();
        quotient += 1;
    }

    if (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0) {
        quotient = -quotient;
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

    let mut is_exponent_odd = false;
    let mut count = 0;

    while count < exponent.abs() {
        count += 1;
        is_exponent_odd = !is_exponent_odd;
    }

    let mut result = 1;

    for _ in 0..exponent {
        let mut temp_result = 0;

        for _ in 0..base.abs() {
            temp_result += result;
        }

        result = temp_result;
    }

    if base < 0 && is_exponent_odd {
        return -result;
    }

    result
}
