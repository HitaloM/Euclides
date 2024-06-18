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
