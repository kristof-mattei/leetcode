impl Solution {
    #[must_use]
    #[expect(clippy::too_many_arguments)]
    #[expect(clippy::similar_names)]
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2)
    }
}

#[expect(clippy::too_many_arguments)]
#[expect(clippy::similar_names)]
fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1)
        - (ax2.min(bx2) - ax1.max(bx1)).max(0) * (ay2.min(by2) - ay1.max(by1)).max(0)
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0223::compute_area;

    #[test]
    fn test_1() {
        assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    }
    #[test]
    fn test_2() {
        assert_eq!(compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
}
