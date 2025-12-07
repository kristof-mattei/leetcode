impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        add_operators(&num, target)
    }
}

fn add_operators(num: &str, target: i32) -> Vec<String> {
    let num = num.chars().collect::<Vec<char>>();

    let mut possibilities = vec![];

    back_tracking(
        &num,
        0,
        (0, 0),
        i64::from(target),
        String::new(),
        &mut possibilities,
    );

    possibilities
}

fn back_tracking(
    num: &Vec<char>,
    offset: usize,
    pieces: (i64, i64),
    target: i64,
    history: String,
    possibilities: &mut Vec<String>,
) {
    if offset == num.len() {
        if pieces.0 + pieces.1 == target {
            possibilities.push(history);
        }
        return;
    }

    let n = num.len();

    let mut temp = 0;

    let mut operation = String::new();

    for index in offset..n {
        operation.push(num[index]);

        temp = 10 * temp + i64::from(num[index] as u8 - b'0');

        if offset == 0 {
            back_tracking(
                num,
                index + 1,
                (0, temp),
                target,
                operation.clone(),
                possibilities,
            );

            if num[offset] == '0' {
                break;
            }

            continue;
        }

        {
            let mut plus = history.clone();
            plus.push('+');
            plus += &operation;
            back_tracking(
                num,
                index + 1,
                (pieces.0 + pieces.1, temp),
                target,
                plus,
                possibilities,
            );
        }

        {
            let mut minus = history.clone();
            minus.push('-');
            minus += &operation;
            back_tracking(
                num,
                index + 1,
                (pieces.0 + pieces.1, -temp),
                target,
                minus,
                possibilities,
            );
        }

        {
            let mut times = history.clone();
            times.push('*');
            times += &operation;
            back_tracking(
                num,
                index + 1,
                (pieces.0, pieces.1 * temp),
                target,
                times,
                possibilities,
            );
        }

        if num[offset] == '0' {
            break;
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use hashbrown::HashSet;

    use crate::problem_0282::add_operators;

    #[test]
    fn test_1() {
        let result = add_operators("123", 6);
        let result = HashSet::<_>::from_iter(result.iter().map(|s| &**s).collect::<Vec<&str>>());

        let expected = ["1*2*3", "1+2+3"];
        let expected = HashSet::<_>::from_iter(expected);

        assert_eq!(result, expected);
    }
}
