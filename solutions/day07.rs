use std::ops::Rem;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StopCode {
    RUN,
    TERM,
}

#[derive(Debug)]
struct ProgramState {
    memory: Vec<i32>,
    func_ptr: usize,
    phase: i32,
    signal: i32,
    stop_code: StopCode,
    output: i32,
    send_signal: bool
}

fn main() -> Result<(), ()> {
    env_logger::init();

    let input = include_str!("../inputs/input-07-2019.txt");

    let program: Vec<i32> = input
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|l| l.split(',').map(|x| i32::from_str(x).unwrap()))
        .collect::<Vec<i32>>();


    let prob_1_answer = (0..5).permutations(5).map(|c| {
        let mut inp = 0;
        for phase in c {
            let mut state = ProgramState { memory: program.clone(), func_ptr: 0, phase, signal: inp, stop_code: StopCode::RUN, output: 0 , send_signal: false};
            process(&mut state);
            inp = state.output;
        }
        inp
    }).max().unwrap();

    println!("Problem 1 answer {}", prob_1_answer);

    let prob_2_answer = (5..10).permutations(5).map(|c| {
        let mut inp = 0;
        let mut states: Vec<ProgramState> = c.iter().map(|phase| ProgramState { memory: program.clone(), func_ptr: 0, phase: *phase, signal: inp, stop_code: StopCode::RUN, output: 0, send_signal: false }).collect();
        let mut i = 0;
        while states[i].stop_code == StopCode::RUN {
            states[i].signal = inp;
            process(&mut states[i]);
            inp = states[i].output;
            i = (i + 1).rem(5);
        }
        states[4].output
    }).max().unwrap();

    println!("Problem 2 answer {}", prob_2_answer);


    Ok(())
}

fn process(state: &mut ProgramState) -> () {

    loop {
        let instr = state.memory[state.func_ptr];
        match instr.rem(100) {
            1 | 2 => {
                three_param(state);
                state.func_ptr += 4;
            }
            3 => {
                let dest = state.memory[state.func_ptr + 1] as usize;
                state.memory[dest] = if state.send_signal { state.signal } else {
                    state.send_signal = true;
                    state.phase
                };
                state.func_ptr += 2;
            }
            4 => {
                let res = if instr >= 100 {
                    state.memory[state.func_ptr + 1]
                } else {
                    state.memory[state.memory[state.func_ptr + 1] as usize]
                };
                state.func_ptr += 2;
                state.output = res;
                return;
            }
            5 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { state.memory[state.func_ptr + 1] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { state.memory[state.func_ptr + 2] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 2] as usize] } //position
                };
                if param1 != 0 {
                    state.func_ptr = param2 as usize;
                } else {
                    state.func_ptr += 3;
                }
            }
            6 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { state.memory[state.func_ptr + 1] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { state.memory[state.func_ptr + 2] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 2] as usize] } //position
                };
                if param1 == 0 {
                    state.func_ptr = param2 as usize;
                } else {
                    state.func_ptr += 3;
                }
            }
            7 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { state.memory[state.func_ptr + 1] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { state.memory[state.func_ptr + 2] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 2] as usize] } //position
                };
                let dest = state.memory[state.func_ptr + 3] as usize;
                state.memory[dest] = if param1 < param2 {
                    1
                } else {
                    0
                };
                state.func_ptr += 4;
            }
            8 => {
                let param1_mode = instr.rem(1_000) >= 100;
                let param2_mode = instr.rem(10_000) >= 1_000;
                let param1 = match param1_mode {
                    true => { state.memory[state.func_ptr + 1] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 1] as usize] } //position
                };
                let param2 = match param2_mode {
                    true => { state.memory[state.func_ptr + 2] } //immediate
                    false => { state.memory[state.memory[state.func_ptr + 2] as usize] } //position
                };
                let dest = state.memory[state.func_ptr + 3] as usize;
                state.memory[dest] = if param1 == param2 {
                    1
                } else {
                    0
                };
                state.func_ptr += 4;
            }
            99 => {
                state.stop_code = StopCode::TERM;
                return;
            }
            _ => {
                println!("Bad instr {instr} at {0}", state.func_ptr);
                unreachable!()
            }
        }
    }
}

fn three_param(state: &mut ProgramState) -> () {
    let func_ptr = state.func_ptr;
    let opcode = state.memory[func_ptr];
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
        true => { state.memory[func_ptr + 1] } //immediate
        false => { state.memory[state.memory[func_ptr + 1] as usize] } //position
    };

    let param2 = match param2_mode {
        true => { state.memory[func_ptr + 2] } //immediate
        false => { state.memory[state.memory[func_ptr + 2] as usize] } //position
    };


    let dest = state.memory[func_ptr + 3] as usize;
    let res = func(param1, param2);
    state.memory[dest] = res;
    // println!("{opcode} {param1} {param2} wrote {res} to {dest}")
}
