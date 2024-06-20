pub fn divide(dividend: i64, divisor: i64) -> Result<(i64, i64), String> {
    if divisor == 0 {
        return Err("Cannot divide by zero".to_string());
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
