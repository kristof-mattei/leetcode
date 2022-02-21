#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::let_and_return)]

use shared::Problem;

mod shared;
mod utils;

mod problem_10;
fn main() {
    problem_10::Solution::test();
}
