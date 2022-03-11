#[cfg(test)]
pub(crate) fn nearly_equal(mut a: f64, mut b: f64, epsilon: f64) -> bool {
    a = (a).abs();
    b = (b).abs();
    let diff = (a - b).abs();

    #[allow(clippy::float_cmp)]
    if a == b {
        // shortcut, handles infinities
        true
    } else if a == 0f64 || b == 0f64 || (a + b < f64::MIN_POSITIVE) {
        // a or b is zero or both are extremely close to it
        // relative error is less meaningful here
        diff < (epsilon * f64::MIN_POSITIVE)
    } else {
        // use relative error
        (diff / f64::min(a + b, f64::MAX)) < epsilon
    }
}
