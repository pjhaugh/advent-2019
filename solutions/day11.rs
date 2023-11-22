use std::collections::HashMap;
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

#[derive(Debug, Copy, Clone)]
enum Facing { UP, DOWN, LEFT, RIGHT }

impl Facing {
    fn left(self: &Self) -> Self {
        match self {
            Facing::UP => { Facing::LEFT }
            Facing::DOWN => { Facing::RIGHT }
            Facing::LEFT => { Facing::DOWN }
            Facing::RIGHT => { Facing::UP }
        }
    }

    fn right(self: &Self) -> Self {
        match self {
            Facing::UP => { Facing::RIGHT }
            Facing::DOWN => { Facing::LEFT }
            Facing::LEFT => { Facing::UP }
            Facing::RIGHT => { Facing::DOWN }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Color { WHITE, BLACK }

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

impl TryFrom<i64> for Color {
    type Error = anyhow::Error;
    fn try_from(value: i64) -> anyhow::Result<Color> {
        match value {
            0 => Ok(Color::BLACK),
            1 => Ok(Color::WHITE),
            _ => bail!("Bad color {value}")
        }
    }
}

impl From<Color> for i64 {
    fn from(value: Color) -> Self {
        match value {
            Color::WHITE => {1}
            Color::BLACK => {0}
        }
    }
}

type Position = (i64, i64);


struct Robot {
    state: ProgramState,
    map: HashMap<Position, Color>,
    pos: Position,
    facing: Facing,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();


    let input = include_str!("../inputs/input-11-2019.txt");

    let program: Vec<i64> = input
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|l| l.split(',').map(|x| i64::from_str(x).unwrap()))
        .collect::<Vec<i64>>();

    let mut robot = Robot {
        state: ProgramState { memory: program.clone(), ..Default::default() },
        map: Default::default(),
        pos: (0, 0),
        facing: Facing::UP,
    };

    while robot.state.stop_code == StopCode::RUN {
        let mut color = robot.map.get(&robot.pos).unwrap_or(&Color::BLACK).clone();
        robot.state.input = color.try_into()?;
        process(&mut robot.state)?;
        if robot.state.stop_code == StopCode::TERM {break}
        color = Color::try_from(robot.state.output)?;
        robot.map.insert(robot.pos, color);
        robot.state.input = color.try_into()?;
        process(&mut robot.state)?;
        match robot.state.output {
            0 => {robot.facing = robot.facing.left()}
            1 => {robot.facing = robot.facing.right()}
            _ => {bail!("Bad turn")}
        }
        robot.pos = match robot.facing {
            Facing::UP => {(robot.pos.0, robot.pos.1 + 1)}
            Facing::DOWN => {(robot.pos.0, robot.pos.1 - 1)}
            Facing::LEFT => {(robot.pos.0 - 1, robot.pos.1)}
            Facing::RIGHT => {(robot.pos.0 + 1, robot.pos.1)}
        }
    }

    println!("Ans 1: {}", robot.map.len());

    robot = Robot {
        state: ProgramState { memory: program.clone(), ..Default::default() },
        map: Default::default(),
        pos: (0, 0),
        facing: Facing::UP,
    };

    robot.map.insert((0, 0), Color::WHITE);

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);

    while robot.state.stop_code == StopCode::RUN {
        let mut color = robot.map.get(&robot.pos).unwrap_or(&Color::BLACK).clone();
        robot.state.input = color.try_into()?;
        process(&mut robot.state)?;
        if robot.state.stop_code == StopCode::TERM {break}
        color = Color::try_from(robot.state.output)?;
        robot.map.insert(robot.pos, color);
        robot.state.input = color.try_into()?;
        process(&mut robot.state)?;
        match robot.state.output {
            0 => {robot.facing = robot.facing.left()}
            1 => {robot.facing = robot.facing.right()}
            _ => {bail!("Bad turn")}
        }
        robot.pos = match robot.facing {
            Facing::UP => {(robot.pos.0, robot.pos.1 + 1)}
            Facing::DOWN => {(robot.pos.0, robot.pos.1 - 1)}
            Facing::LEFT => {(robot.pos.0 - 1, robot.pos.1)}
            Facing::RIGHT => {(robot.pos.0 + 1, robot.pos.1)}
        };
        min_x = min_x.min(robot.pos.0);
        max_x = max_x.max(robot.pos.0);
        min_y = min_y.min(robot.pos.1);
        max_y = max_y.max(robot.pos.1);
    }

    for y in (min_y..=max_y).rev() {
        let mut line = "".to_owned();
        for x in min_x..=max_x {
            line.push_str(match robot.map.get(&(x, y)) {
                Some(Color::WHITE) => {"█"}
                _ => " "
            });
        }
        println!("{line}");
    }

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
        7 => |x, y| (x < y) as i64,
        8 => |x, y| (x == y) as i64,
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
        Mode::RELATIVE => { Ok(access(state, (state.relative_base as i64 + access(state, state.func_ptr + offset)) as usize)) }
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
        None => { 0 }
        Some(x) => { *x }
    }
}
