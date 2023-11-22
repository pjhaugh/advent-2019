use std::ops::{Div, Rem};
use std::str::FromStr;

use anyhow::bail;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StopCode {
    RUN,
    TERM,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Mode {
    POSITION,
    IMMEDIATE,
    RELATIVE,
}

// #[derive(Debug, Snafu)]
// enum Error {
//     #[snafu(display("Could not process opcode {opcode}"))]
//     BadOpcode {opcode: i64},
//     #[snafu(display("Unsupported mode"))]
//     BadMode
// }


#[derive(Debug)]
struct ProgramState {
    memory: Vec<i64>,
    func_ptr: usize,
    input: i64,
    stop_code: StopCode,
    output: i64,
    relative_base: usize,
}

impl Default for ProgramState {
    fn default() -> Self {
        ProgramState {
            memory: vec![],
            func_ptr: 0,
            input: 0,
            stop_code: StopCode::RUN,
            output: 0,
            relative_base: 0,
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();


    let input = include_str!("../inputs/input-09-2019.txt");

    let program: Vec<i64> = input
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|l| l.split(',').map(|x| i64::from_str(x).unwrap()))
        .collect::<Vec<i64>>();

    let mut state_1 = ProgramState {memory: program.clone(), input: 1, ..Default::default()};

    while state_1.stop_code == StopCode::RUN {
        process(&mut state_1)?;
    }

    println!("Answer 1: {}", state_1.output);

    let mut state_2 = ProgramState {memory: program.clone(), input: 2, ..Default::default()};

    while state_2.stop_code == StopCode::RUN {
        process(&mut state_2)?;
    }

    println!("Answer 2: {}", state_2.output);
    Ok(())
}

fn process(state: &mut ProgramState) -> anyhow::Result<()> {
    loop {
        let instr = state.memory[state.func_ptr];
        match instr.rem(100) {
            1 | 2 | 7 | 8 => {
                three_param(state)?;
                state.func_ptr += 4;
            }
            3 => {
                let dest = get_param_dest(state, 1)?;
                state.memory[dest] = state.input;
                state.func_ptr += 2;
            }
            4 => {
                let res = get_param_value(state, 1)?;
                state.func_ptr += 2;
                state.output = res;
                return Ok(());
            }
            5 => {
                let param1 = get_param_value(state, 1)?;
                let param2 = get_param_value(state, 2)?;
                if param1 != 0 {
                    state.func_ptr = param2 as usize;
                } else {
                    state.func_ptr += 3;
                }
            }
            6 => {
                let param1 = get_param_value(state, 1)?;
                let param2 = get_param_value(state, 2)?;
                if param1 == 0 {
                    state.func_ptr = param2 as usize;
                } else {
                    state.func_ptr += 3;
                }
            }
            9 => {
                let param1 = get_param_value(state, 1)?;
                state.relative_base = if param1.is_negative() {
                    state.relative_base - param1.wrapping_abs() as usize
                } else {
                    state.relative_base + param1 as usize
                };
                state.func_ptr += 2;
            }
            99 => {
                state.stop_code = StopCode::TERM;
                return Ok(());
            }
            _ => {
                println!("Bad instr {instr} at {0}", state.func_ptr);
                unreachable!()
            }
        }
    }
}

fn three_param(state: &mut ProgramState) -> anyhow::Result<()> {
    let func_ptr = state.func_ptr;
    let opcode = state.memory[func_ptr];
    let _param3_mode = get_mode(opcode, 3)?;

    let func = match opcode.rem(10) {
        1 => std::ops::Add::add,
        2 => std::ops::Mul::mul,
        7 => |x, y| (x < y) as i64 ,
        8 => |x, y| (x == y) as i64 ,
        x => {
            println!("Bad opcode {opcode} val {x}");
            unreachable!()
        }
    };

    let param1 = get_param_value(state, 1)?;
    let param2 = get_param_value(state, 2)?;

    let dest = get_param_dest(state, 3)?;
    let res = func(param1, param2);
    state.memory[dest] = res;
    Ok(())
    // println!("{opcode} {param1} {param2} wrote {res} to {dest}")
}

fn get_mode(opcode: i64, pos: usize) -> anyhow::Result<Mode> {
    match opcode.div(10_i64 * 10_i64.pow(pos as u32)).rem(10) {
        0 => Ok(Mode::POSITION),
        1 => Ok(Mode::IMMEDIATE),
        2 => Ok(Mode::RELATIVE),
        n => bail!("Unrecognized mode [{n}] in opcode [{opcode}]")
    }
}

fn get_param_value(state: &ProgramState, offset: usize) -> anyhow::Result<i64> {
    let mode = get_mode(state.memory[state.func_ptr], offset)?;
    match mode {
        Mode::IMMEDIATE => { Ok(access(state, state.func_ptr + offset)) }
        Mode::POSITION => { Ok(access(state, access(state, state.func_ptr + offset) as usize)) }
        Mode::RELATIVE => { Ok(access(state, (state.relative_base as i64 + access(state, state.func_ptr + offset)) as usize))}
        // _ => { bail!("Unsupported mode {mode:?}") }
    }
}

fn get_param_dest(state: &mut ProgramState, offset: usize) -> anyhow::Result<usize> {
    let mode = get_mode(state.memory[state.func_ptr], offset)?;
    let result = match mode {
        Mode::POSITION => { access(state, state.func_ptr + offset) as usize }
        Mode::RELATIVE => { (state.relative_base as i64 + access(state, state.func_ptr + offset)) as usize }
        Mode::IMMEDIATE => bail!("Can not write in Immediate mode")
    };

    if state.memory.len() <= result { state.memory.resize(result + 1, 0) }
    Ok(result)
}

fn access(state: &ProgramState, addr: usize) -> i64 {
    match state.memory.get(addr) {
        None => {0}
        Some(x) => {*x}
    }
}