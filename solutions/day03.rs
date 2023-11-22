use std::str::FromStr;

use nom::{IResult, Parser};
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;

type Point = (i32, i32);

type Wire = Vec<Point>;

#[derive(Debug, Copy, Clone)]
enum Dir {
    L,
    R,
    U,
    D,
}

fn parse_dir(input: &str) -> IResult<&str, Dir> {
    let (input, c) = take(1usize)(input)?;
    match c {
        "L" => { Ok((input, Dir::L)) }
        "D" => { Ok((input, Dir::D)) }
        "U" => { Ok((input, Dir::U)) }
        "R" => { Ok((input, Dir::R)) }
        _ => unreachable!()
    }
}

fn parse_move(input: &str) -> IResult<&str, (Dir, i32)> {
    let (input, dir) = parse_dir(input)?;
    let (input, num) = map_res(digit1, i32::from_str).parse(input)?;
    Ok((input, (dir, num)))
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    let mut point = (0, 0);
    let mut acc = vec![point.clone()];
    let (input, moves) = separated_list1(tag(","), parse_move).parse(input)?;
    for (dir, len) in moves {
        match dir {
            Dir::L => { point = (point.0 - len, point.1) }
            Dir::R => { point = (point.0 + len, point.1) }
            Dir::U => { point = (point.0, point.1 + len) }
            Dir::D => { point = (point.0, point.1 - len) }
        }
        acc.push(point.clone());
    }
    Ok((input, acc))
}

fn get_line_segments(wire: &Wire) -> Vec<(Point, Point)> {
    wire.windows(2).map(|a| (a[0], a[1])).collect::<Vec<(Point, Point)>>()
}

fn intersect(l1: &(Point, Point), l2: &(Point, Point)) -> Option<Point> {
    let s1 = (l1.1.0 - l1.0.0, l1.1.1 - l1.0.1);
    let s2 = (l2.1.0 - l2.0.0, l2.1.1 - l2.0.1);

    let s = (-s1.1 * (l1.0.0 - l2.0.0) + s1.0 * (l1.0.1 - l2.0.1)) as f32 / ((-s2.0 * s1.1 + s1.0 * s2.1) as f32);
    let t = (s2.0 * (l1.0.1 - l2.0.1) - s2.1 * (l1.0.0 - l2.0.0)) as f32 / ((-s2.0 * s1.1 + s1.0 * s2.1) as f32);

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        return Some((l1.0.0 + (t * s1.0 as f32) as i32, l1.0.1 + (t * s1.1 as f32) as i32));
    }
    None
}


fn get_intersections(wire1: &Wire, wire2: &Wire) -> Vec<Point> {
    let seg1 = get_line_segments(wire1);
    let seg2 = get_line_segments(wire2);

    let mut intersections: Vec<Point> = Default::default();

    for l1 in &seg1 {
        for l2 in &seg2 {
            match intersect(l1, l2) {
                None => {}
                Some(p) => { intersections.push(p) }
            }
        }
    }

    return intersections;
}

fn distance_along(wire: &Wire, point: &Point) -> Option<u32> {
    let mut head = (0, 0);

    let mut dist = 0;

    for bend in wire {
        if bend.0 == head.0 && bend.0 == point.0 {
            let range = if head.1 < bend.1 { head.1..=bend.1 } else { bend.1..=head.1 };
            if range.contains(&point.1) {
                dist = dist + head.1.abs_diff(point.1);
                return Some(dist);
            }
        } else if bend.1 == head.1 && bend.1 == point.1 {
            let range = if head.0 < bend.0 { head.0..=bend.0 } else { bend.0..=head.0 };
            if range.contains(&point.0) {
                dist = dist + head.0.abs_diff(point.0);
                return Some(dist);
            }
        }
        let diff = head.0.abs_diff(bend.0) + head.1.abs_diff(bend.1);
        dist = dist + diff;
        head = bend.clone();
    }
    println!("Could not find point {point:?}");
    None
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let input = include_str!("../inputs/input-03-2019.txt");

    let mut wire_iter = input.lines()
        .map(parse_wire);

    let wire1 = wire_iter.next().unwrap()?.1;
    let wire2 = wire_iter.next().unwrap()?.1;

    let intersections = get_intersections(&wire1, &wire2);

    let prob_1 = intersections.iter().filter_map(|p|
        if *p == (0, 0) { None } else { Some(p.0 + p.1) }
    ).min().unwrap();

    println!("Answer 1: {prob_1}");

    let prob_2 = intersections.iter()
        .filter_map(|p| Some(distance_along(&wire1, p)? + distance_along(&wire2, p)?))
        .filter(|d| *d > 0u32)
        .min().unwrap();

    println!("Answer 2: {prob_2}");


    Ok(())
}
