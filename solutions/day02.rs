use std::str::FromStr;
use log::debug;

fn main() -> Result<(), ()> {

    env_logger::init();

    let input = include_str!("../inputs/input-02-2019.txt");

    let mut program: Vec<usize> = input
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|l| l.split(',').map(|x| usize::from_str(x).unwrap()))
        .collect::<Vec<usize>>();

    program[1] = 12;
    program[2] = 2;

    let prob_1a_answer = process(&mut program);

    println!("Problem 1a answer {}", prob_1a_answer);

    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            program = input
                .lines()
                .filter(|s| !s.is_empty())
                .flat_map(|l| l.split(',').map(|x| usize::from_str(x).unwrap()))
                .collect::<Vec<usize>>();

            program[1] = noun;
            program[2] = verb;

            if process(&mut program) == 19690720 {
                println!("Problem 1b answer {noun}{verb}");
                break 'outer;
            }
        }
        debug!("{noun}/99");
    }

    Ok(())
}

fn process(program: &mut Vec<usize>) -> usize {
    let mut func_ptr = 0;

    loop {
        log::debug!("{:?} {:?}", func_ptr, program);
        match program[func_ptr] {
            1 => {
                let x = program[program[func_ptr + 1]];
                let y = program[program[func_ptr + 2]];
                let dest = program[func_ptr + 3];
                log::debug!("{x} {y} {dest}");
                program[dest] = x + y;
                func_ptr += 4;
            }
            2 => {
                let x = program[program[func_ptr + 1]];
                let y = program[program[func_ptr + 2]];
                let dest = program[func_ptr + 3];
                log::debug!("{x} {y} {dest}");
                program[dest] = x * y;
                func_ptr += 4;
            }
            99 => return program[0],
            _ => {unreachable!()}
        }
    }

}
