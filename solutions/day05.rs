use std::ops::Rem;
use std::str::FromStr;

fn main() -> Result<(), ()> {
    env_logger::init();

    let input = include_str!("../inputs/input-05-2019.txt");

    let mut program: Vec<i32> = input
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|l| l.split(',').map(|x| i32::from_str(x).unwrap()))
        .collect::<Vec<i32>>();

    let prob_1_answer = process(&mut program, || 1);

    println!("Problem 1 answer {}", prob_1_answer);

    let prob_2_answer = process(&mut program, || 5);

    println!("Problem 2 answer {}", prob_2_answer);


    Ok(())
}

fn process<F>(program: &mut Vec<i32>, f: F) -> i32
    where F: Fn() -> i32 {
    let mut func_ptr = 0;

    loop {
        let instr = program[func_ptr];
        match instr.rem(100) {
            1 | 2 => {
                three_param(func_ptr, program);
                func_ptr += 4;
            }
            3 => {
                let dest = program[func_ptr + 1] as usize;
                program[dest] = f();
                func_ptr += 2;
            }
            4 => {
                if instr >= 100 {
                    println!("Output: {}", program[func_ptr + 1])
                } else {
                    println!("Output: {}", program[program[func_ptr + 1] as usize])
                }
                func_ptr += 2;
            }
            5 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { program[func_ptr + 1] } //immediate
                    false => { program[program[func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { program[func_ptr + 2] } //immediate
                    false => { program[program[func_ptr + 2] as usize] } //position
                };
                if param1 != 0 {
                    func_ptr = param2 as usize;
                } else {
                    func_ptr += 3;
                }
            }
            6 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { program[func_ptr + 1] } //immediate
                    false => { program[program[func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { program[func_ptr + 2] } //immediate
                    false => { program[program[func_ptr + 2] as usize] } //position
                };
                if param1 == 0 {
                    func_ptr = param2 as usize;
                } else {
                    func_ptr += 3;
                }
            }
            7 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { program[func_ptr + 1] } //immediate
                    false => { program[program[func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { program[func_ptr + 2] } //immediate
                    false => { program[program[func_ptr + 2] as usize] } //position
                };
                let dest = program[func_ptr + 3] as usize;
                program[dest] = if param1 < param2 {
                    1
                } else {
                    0
                };
                func_ptr += 4;
            }
            8 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { program[func_ptr + 1] } //immediate
                    false => { program[program[func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { program[func_ptr + 2] } //immediate
                    false => { program[program[func_ptr + 2] as usize] } //position
                };
                let dest = program[func_ptr + 3] as usize;
                program[dest] = if param1 == param2 {
                    1
                } else {
                    0
                };
                func_ptr += 4;
            }
            99 => return program[0],
            _ => {
                println!("Bad instr {instr} at {func_ptr}");
                unreachable!()
            }
        }
    }
}

fn three_param(func_ptr: usize, program: &mut Vec<i32>) -> () {
    let opcode = program[func_ptr];
    let param1_mode = opcode.rem(1_000) >= 100;
    let param2_mode = opcode.rem(10_000) >= 1_000;
    let _param3_mode = opcode.rem(100_000) >= 10_000;

    let func = match opcode.rem(10) {
        1 => std::ops::Add::add,
        2 => std::ops::Mul::mul,
        x => {
            println!("Bad opcode {opcode} val {x}");
            unreachable!()
        }
    };

    let param1 = match param1_mode {
        true => { program[func_ptr + 1] } //immediate
        false => { program[program[func_ptr + 1] as usize] } //position
    };

    let param2 = match param2_mode {
        true => { program[func_ptr + 2] } //immediate
        false => { program[program[func_ptr + 2] as usize] } //position
    };


    let dest = program[func_ptr + 3] as usize;
    let res = func(param1, param2);
    program[dest] = res;
    // println!("{opcode} {param1} {param2} wrote {res} to {dest}")
}
