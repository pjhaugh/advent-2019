use std::collections::{HashMap, HashSet};
use std::ops::{Add, Range, Rem};

use gcd::Gcd;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Asteroid {
    x: i32,
    y: i32,
}



fn get_asteroids(input: &str) -> HashSet<Asteroid> {
    let mut asteroids: HashSet<Asteroid> = Default::default();

    input.lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate()
                .for_each(|(x, c)| {
                    if c != '.' {
                        asteroids.insert(Asteroid { x: x as i32, y: y as i32 });
                    }
                })
        });

    return asteroids;
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/test-10-2019.txt");


    let asteroids = get_asteroids(input);
    let mut counts: HashMap<Asteroid, u32> = Default::default();

    for source in &asteroids {
        let mut count = 0;
        for dest in &asteroids {
            if not_visible(&asteroids, source, &dest) { continue; }
            count += 1;
        }
        counts.insert(source.clone(), count);
    }

    let ans_1 = counts.iter().max_by_key(|(_k, v)| **v).unwrap();
    println!("Answer 1: Base: {:?}, count: {}", ans_1.0, ans_1.1);


    let base = ans_1.0;

    let mut angles: HashMap<i32, Vec<&Asteroid>> = Default::default();

    for asteroid in &asteroids {
        if asteroid == base { continue; }
        let angle = get_angle(base, asteroid);
        match angles.get_mut(&angle) {
            None => {
                angles.insert(angle, vec![asteroid]);
            }
            Some(v) => { v.push(asteroid); }
        }
    }


    angles.values_mut().for_each(|v| v.sort_by_key(|a| -dist(base, *a)));

    let mut target_lines = angles.iter()
        .sorted_by_key(|(angle, _v)| **angle)
        .map(|x| {
            // println!("{:09}", x.0);
            x.1.clone()
        })
        .collect_vec();

    let target_count = target_lines.iter().map(|v| v.len()).sum::<usize>();
    assert_eq!(target_count, asteroids.len()-1);

    // assert_eq!(*ans_1.1, angles.len() as u32);

    for ast in &asteroids {
        if !not_visible(&asteroids, base, ast) {
            println!("V: {:?}", ast);
        }
    }

    for line in &target_lines{
        println!("{:?}", line);
    }


    let mut index: usize = 0;
    let mut destroyed = 0;

    loop {
        match target_lines.get_mut(index).unwrap().pop() {
            None => {}
            Some(ast) => {
                destroyed += 1;
                println!("Asteroid {destroyed} destroyed: {:?}", ast);
                if destroyed == 200 {
                    println!("Answer 2: {}{:02}", ast.x, ast.y);
                    // return Ok(());
                }
            }
        }
        index = (index + 1).rem(target_lines.len());
    }
}

fn get_angle(base: &Asteroid, asteroid: &Asteroid) -> i32 {
    let angle_rad = ((asteroid.y - base.y) as f32).atan2((asteroid.x - base.x) as f32);
    let angle_deg = angle_rad.to_degrees().add(270_f32 + 180.).rem(360_f32);
    let angle = (angle_deg * 1000000.) as i32;
    angle
}

fn not_visible(asteroids: &HashSet<Asteroid>, source: &Asteroid, dest: &Asteroid) -> bool {
    if dest == source { return true; }
    if dest.x == source.x {
        for y in walk(dest.y, dest.x) {
            if asteroids.contains(&Asteroid { x: dest.x, y }) {
                return true;
            }
        }
    }
    let (left, right) = if source.x < dest.x { (source, dest) } else { (dest, source) };
    let x_diff_full = right.x - left.x;
    let y_diff_full = right.y - left.y;
    let gcd = (x_diff_full as u32).gcd(y_diff_full.abs() as u32) as i32;
    let x_diff = x_diff_full / gcd;
    let y_diff = y_diff_full / gcd;
    let mut x = left.x + x_diff;
    let mut y = left.y + y_diff;
    while x < right.x {
        if asteroids.contains(&Asteroid { x, y }) {
            return true;
        }
        x += x_diff;
        y += y_diff;
    }
    false
}

fn dist(a1: &Asteroid, a2: &Asteroid) -> i32 {
    a1.x.abs_diff(a2.x) as i32 + a1.y.abs_diff(a2.y) as i32
}

fn walk(a: i32, b: i32) -> Range<i32> {
    if a < b {
        (a + 1)..b
    } else {
        (b + 1)..a
    }
}