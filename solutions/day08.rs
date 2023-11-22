use std::str::FromStr;

use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Line = Vec<u32>;
type Layer = Vec<Line>;


fn main() -> Result<(), ()> {
    let input = include_str!("../inputs/input-08-2019.txt");

    let pic = input.chars()
        .map(|c| u32::from_str(c.to_string().as_str()).unwrap())
        .batching(|it| {
            let res = it.take(WIDTH).collect::<Line>();
            if res.is_empty() {return None}
            Some(res)
        })
        .batching(|it| {
            let res = it.take(HEIGHT).collect::<Layer>();
            if res.is_empty() {return None}
            Some(res)
        })
        .collect::<Vec<Layer>>();

    let min_zero_layer = pic.iter().min_by_key(|l| {count(l, 0)}).unwrap();


    let ans_1 = count(min_zero_layer, 1) * count(min_zero_layer, 2);

    println!("Answer 1: {ans_1}");

    let mut work_space = pic[0].clone();

    for layer in pic.iter().skip(1) {
        for (line_index, line) in layer.iter().enumerate() {
            for (pixel_index, pixel) in line.iter().enumerate() {
                match work_space[line_index][pixel_index] {
                    2 => {
                        work_space[line_index][pixel_index] = pixel.clone();
                    }
                    1 | 0 => {}
                    _ => unreachable!()
                }
            }
        }
    }

    for line in work_space {
        let s = line.iter().map(|x| if *x == 0 {" "} else {"█"}).join("");
        println!("{s}");
    }

    Ok(())
}

fn count(layer: &Layer, item: u32) -> usize {
    layer.iter().map(|line| line.iter().filter(|x| x.eq(&&item)).count()).sum::<usize>()
}