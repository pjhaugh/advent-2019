use std::collections::{HashMap, HashSet};

fn main() -> Result<(), ()> {

    let input = include_str!("../inputs/input-06-2019.txt");

    let mut orbits: HashMap<&str, &str> = Default::default();

    input.lines().for_each(|s| {
        let mut spl = s.split(")");
        let center = spl.next().unwrap();
        let satellite = spl.next().unwrap();
        orbits.insert(satellite, center);
    });

    let prob_1_ans: u32 = orbits.keys().map(|k| count(&orbits, k, "COM")).sum();

    println!("Prob 1: {prob_1_ans}");

    let prob_2_ans: u32 = get_route(&orbits, "YOU", "SAN");

    println!("Prob 2: {prob_2_ans}");

    Ok(())
}


fn count(orbits: &HashMap<&str, &str>, start: &str, dest: &str) -> u32 {
    let mut curr = start;
    let mut count = 0;
    while !curr.eq(dest) {
        count += 1;
        curr = orbits.get(curr).unwrap();
    }
    return count;
}

fn get_route(orbits: &HashMap<&str, &str>, start: &str, dest: &str) -> u32{
    let mut curr = start;
    let mut visits: HashSet<&str> = Default::default();
    while !curr.eq("COM") {
        curr = orbits.get(curr).unwrap();
        visits.insert(curr);
    }

    curr = dest;
    while !curr.eq("COM") {
        curr = orbits.get(curr).unwrap();
        if visits.contains(curr) {
            return count(orbits, start, curr) + count(orbits, dest, curr) - 2;
        }
    }
    0
}