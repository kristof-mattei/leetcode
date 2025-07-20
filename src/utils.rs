#[cfg(test)]
pub(crate) fn nearly_equal(a: f64, b: f64, epsilon: f64) -> bool {
    let abs_a = (a).abs();
    let abs_b = (b).abs();
    let diff = (a - b).abs();

    #[expect(clippy::float_cmp)]
    if a == b {
        // shortcut, handles infinities
        true
    } else if a == 0_f64 || b == 0_f64 || (abs_a + abs_b < f64::MIN_POSITIVE) {
        // a or b is zero or both are extremely close to it
        // relative error is less meaningful here
        diff < (epsilon * f64::MIN_POSITIVE)
    } else {
        // use relative error
        (diff / f64::min(abs_a + abs_b, f64::MAX)) < epsilon
    }
}

pub mod svec {
    #![macro_use]

    #[macro_export]
    macro_rules! svec {
        () => {{
            let v = Vec::<String>::new();
            v

        }};
        ($($elem:expr),+ $(,)?) => {{
            let v = vec![
                $( String::from($elem), )*
            ];
            v
        }};
    }
}
