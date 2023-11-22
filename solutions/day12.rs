use std::str::FromStr;

use nom::bytes::complete::{tag, take_until};
use nom::combinator::map_res;
use nom::sequence::preceded;

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

type Velocity = Position;


#[derive(Debug)]
struct Moon {
    pos: Position,
    vel: Velocity,
}

impl Moon {
    fn total_energy(self: &Self) -> i64 {
        (self.pos.x.abs()
            + self.pos.y.abs()
            + self.pos.z.abs())
            * (self.vel.x.abs()
            + self.vel.y.abs()
            + self.vel.z.abs())
    }

    fn apply_grav(self: &Self, other: &Self) -> Velocity{
        let mut x = self.vel.x;
        let mut y = self.vel.y;
        let mut z = self.vel.z;
        if self.pos.x < other.pos.x {
            x += 1;
        } else if self.pos.x > other.pos.x {
            x -= 1;
        }
        if self.pos.y < other.pos.y {
            y += 1;
        } else if self.pos.y > other.pos.y {
            y -= 1;
        }
        if self.pos.z < other.pos.z {
            z += 1;
        } else if self.pos.z > other.pos.z {
            z -= 1;
        }
        Velocity{x, y, z}
    }

    fn apply_velocity(self: &mut Self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}

// <x=-9, y=-1, z=-1>

fn parse_moon(input: &str) -> nom::IResult<&str, Moon> {
    let (input, x) = preceded(tag("<x="), map_res(take_until(", "), i64::from_str))(input)?;
    let (input, y) = preceded(tag(", y="), map_res(take_until(", "), i64::from_str))(input)?;
    let (input, z) = preceded(tag(", z="), map_res(take_until(">"), i64::from_str))(input)?;

    Ok((input, Moon { pos: Position { x, y, z }, vel: Default::default() }))
}

fn step(moons: &mut Vec<Moon>){
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {continue}
            let other = &moons[j];
            moons[i].vel = moons[i].apply_grav(other);
        }
    }
    for moon in moons {
        moon.apply_velocity();
    }
}

fn get_xs(moons: &Vec<Moon>) -> Vec<(i64, i64)> {
    moons.iter().map(|m| (m.pos.x, m.vel.x)).collect()
}
fn get_ys(moons: &Vec<Moon>) -> Vec<(i64, i64)> {
    moons.iter().map(|m| (m.pos.y, m.vel.y)).collect()
}
fn get_zs(moons: &Vec<Moon>) -> Vec<(i64, i64)> {
    moons.iter().map(|m| (m.pos.z, m.vel.z)).collect()
}

fn get_periods(moons: &mut Vec<Moon>) -> (usize, usize, usize) {
    let (mut x, mut y, mut z) = (None, None, None);
    let initial_xs = get_xs(moons);
    let initial_ys = get_ys(moons);
    let initial_zs = get_zs(moons);
    let mut steps = 0;

    while x.is_none() || y.is_none() || z.is_none() {
        step(moons);
        steps += 1;
        if let None = x {
            let current_xs = get_xs(moons);
            if initial_xs == current_xs {
                x = Some(steps);
            }
        }
        if let None = y {
            let current_ys = get_ys(moons);
            if initial_ys == current_ys {
                y = Some(steps);
            }
        }
        if let None = z {
            let current_zs = get_zs(moons);
            if initial_zs == current_zs {
                z = Some(steps);
            }
        }
    }

    (x.unwrap(), y.unwrap(), z.unwrap())
}

fn lcm(x: u64, y: u64, z: u64) -> u64 {
    let (mut a, mut b, mut c) = (x, y, z);
     while a != b || a != c {
         if a <= b && a <= c {
             a += x;
         } else if b <= a && b <= c {
             b += y;
         } else {
             c += z;
         }
     }
    a
}

fn main() -> anyhow::Result<()> {
    let inp = include_str!("../inputs/input-12-2019.txt");

    let mut moons: Vec<Moon> = inp.lines()
        .map(parse_moon)
        .map(|r| Ok(r?.1))
        .collect::<anyhow::Result<Vec<Moon>>>()?;

    for _t in 0..1000 {
        // println!("After {t} steps: ");
        // for m in &moons {
        //     println!("{m:?}");
        // }
        // println!();
        step(&mut moons);
    }

    println!("Answer 1: {}", moons.iter().map(Moon::total_energy).sum::<i64>());

    let mut moons: Vec<Moon> = inp.lines()
        .map(parse_moon)
        .map(|r| Ok(r?.1))
        .collect::<anyhow::Result<Vec<Moon>>>()?;

    let (x, y, z) = get_periods(&mut moons);

    println!("Answer 2: {}", lcm(x as u64, y as u64, z as u64));

    Ok(())
}

