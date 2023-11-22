use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

fn part_a_check(x: u32) -> bool {
    let s = x.to_string();
    let bts = s.as_bytes();

    let repeat_check = bts.windows(2).any(|y| y[0] == y[1]);

    let increasing_check = bts.windows(2).all(|y| y[0] <= y[1]);

    repeat_check & increasing_check
}

fn part_b_check(x: u32) -> bool {
    let s = x.to_string();
    let bts = s.as_bytes();

    let repeat_check = bts.windows(2).any(|y| y[0] == y[1]);

    let increasing_check = bts.windows(2).all(|y| y[0] <= y[1]);

    let mut group_check = false;

    for (_, g) in &bts.iter().group_by(|m| **m) {
        group_check |= g.count() == 2;
    }

    repeat_check & increasing_check & group_check
}


fn main() -> anyhow::Result<()> {
    let mut input = include_str!("../inputs/input-04-2019.txt")
        .split("-")
        .map(|s| u32::from_str(s.trim()).unwrap());

    let lower = input.next().context("short")?;
    let upper = input.next().context("short")?;

    println!("{lower}, {upper}");

    let part_a_ans: usize = (lower..=upper).map(part_a_check).filter(|b| *b).count();

    println!("Part A: {part_a_ans}");

    let part_b_ans: usize = (lower..=upper).map(part_b_check).filter(|b| *b).count();

    println!("Part A: {part_b_ans}");

    Ok(())
}
