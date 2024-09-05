pub fn divide(dividend: i64, divisor: i64) -> Result<(i64, i64), String> {
    if divisor == 0 {
        return Err("Division by zero".to_string());
    }

    let mut quotient = 0;
    let mut remainder = dividend.abs();
    let divisor_abs = divisor.abs();

    while remainder >= divisor_abs {
        remainder -= divisor_abs;
        quotient += 1;
    }

    if (dividend < 0) ^ (divisor < 0) {
        quotient = -quotient;
    }

    if dividend < 0 {
        remainder = -remainder;
    }

    Ok((quotient, remainder))
}
