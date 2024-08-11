#!/bin/bash
if [ -z "${1}" ]; then
    echo "Please invoke with ${0} problem_number, e.g. ${0} 30"
    exit 1
fi

printf -v folder_name "problem_%04d" "$1"

mkdir "src/${folder_name}"

cat > "src/${folder_name}/mod.rs" << EOF
impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn main_fn(input: Vec<String>) -> i32 {
        sub_fn(&input)
    }
}

fn sub_fn(input: &[String]) -> i32 {
    input.len() as i32
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::${folder_name}::sub_fn;

    #[test]
    fn test_1() {
        assert_eq!(
            sub_fn(
                &["a", "b", "c", "d", "e"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            5
        );
    }
}


EOF

echo "pub mod ${folder_name};" >> src/lib.rs

rustfmt "src/${folder_name}/mod.rs"
rustfmt "src/lib.rs"
