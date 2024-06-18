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
