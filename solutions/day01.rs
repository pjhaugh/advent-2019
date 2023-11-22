use std::ops::Div;
use std::str::FromStr;

fn main() -> Result<(), ()> {
    let input = include_str!("../inputs/input-01-2019.txt");

    let prob_1a_answer: u32 = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(u32::from_str)
        .map(Result::unwrap)
        .map(|x| x.div(3) - 2)
        .sum::<u32>();

    println!("Problem 1a answer {}", prob_1a_answer);

    let prob_1b_answer = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(i32::from_str)
        .map(Result::unwrap)
        .map(|x| {
            let mut res = 0;
            let mut y = x.div(3) - 2;
            while y > 0 {
                res += y;
                y = y.div(3) - 2;
            }
            return res;
        })
        .sum::<i32>();

    println!("Problem 1b answer {}", prob_1b_answer);

    Ok(())
}
