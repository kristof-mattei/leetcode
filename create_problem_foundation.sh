#!/bin/bash
if [ -z "${1}" ]; then
    echo "Please invoke with ${0} problem_number, e.g. ${0} 30"
    exit 1
fi

printf -v folder_name "problem_%04d" "$1"

mkdir "src/${folder_name}"

cat << 'EOF' > src/${folder_name}/mod.rs
impl Solution {
    #[must_use]
    // #[allow(clippy::needless_pass_by_value)]
    pub fn XXXXXXXXXXXXXXXXXXXX() {}
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::${folder_name}::XXXXXXXXXXXXXXXXXXXX;

    #[test]
    fn test_1() {}
}
EOF

echo "pub mod ${folder_name};" >> src/lib.rs

rustfmt "src/${folder_name}/mod.rs"
rustfmt "src/lib.rs"
