use std::io;
use std::collections::HashMap;

fn main() {
    let mut source : Vec<&str> = include_str!("./input.txt").lines().collect();
    //let source : Vec<&str> = include_str!("./input_test.txt").lines().collect();

    let mut a_program = Program::new(source[0], 100);
    let mut grid: HashMap<(isize,isize), char> = HashMap::new();
    let mut outputs: Vec<isize> = Vec::new();
    let mut output_count = 0;
    let mut count = 0;
    let mut output = Ok(0);
    let mut input = vec![];
    let mut input_x = 0;
    let mut input_y = 0;
    let mut box_x = 0;
    let mut box_y = 0;
    let mut box_height = 100;
    let mut box_width = 100;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut size = 10;
    let mut width = 10_000;
    let mut height = 10_000;
    let mut row_start = 0;
    let mut found_row = false;
    let mut prev_row_x = 0;
    let mut prev_row_y = 0;

    loop {

        while output.is_ok() {

            output = run_program(&mut a_program, &input);
            output_count += 1;

            if output.is_err() {
                break;
            }

            let c  = match output.unwrap() {
                1 => '#',
                _ => '.'
            };

            grid.insert((input_x, input_y), c);

            if c == '#' {
                found_row = true;
            }

            if c == '#' && row_start == 0 {
                row_start = input_x;
            }

            if c == '.' && found_row {
                count = (input_y+1) * width + row_start;
                row_start = 0;
                found_row = false;
                prev_row_x = input_x - 1;
                prev_row_y = input_y - 1;
            }

            if c == '.' && input_x > (prev_row_x + 10) {
                count = (input_y+1) * width + row_start;
                row_start = 0;
                found_row = false;
            }
        }

        if output == Err(3) {
            output = Ok(0);

            input_x = count % height;
            input_y = count / height;
            count += 1;

            input.push(input_x);
            input.push(input_y);

            if grid.len() % 1000 == 0 {
                let found = search_box(&grid, size);
                if found {
                    size += 10;
                }
            }
        }

        if output == Err(99) {
            if count >= width * height {
                break;
            }

            a_program = Program::new(source[0], 100);
            output = Ok(0);
            input.clear();
        }
    }
}

fn search_box(grid: &HashMap<(isize, isize), char>, size: isize) -> bool {
    let mut result = false;
    let mut pos = (9999, 9999);

    'coord: for (coord, &c) in grid.iter() {
        if c != '#' {
            continue;
        }


        let item = grid.get(&(coord.0 + size, coord.1 + size-1));
        if item.is_some() {
            if *item.unwrap() != '#' {
                continue 'coord;
            }
        } else {
            continue 'coord;
        }

        let item = grid.get(&(coord.0, coord.1 + size-1));
        if item.is_some() {
            if *item.unwrap() != '#' {
                continue 'coord;
            }
        } else {
            continue 'coord;
        }

        let item = grid.get(&(coord.0 + size-1, coord.1));
        if item.is_some() {
            if *item.unwrap() != '#' {
                continue 'coord;
            }
        } else {
            continue 'coord;
        }

        println!("({},{}) SIZE: {}!", coord.0, coord.1, size);
        if pos.0 > coord.0 || pos.1 > coord.1 {
            pos.0 = coord.0;
            pos.1 = coord.1;
        }

        result = true;
    }

    if result {
        paint_grid(&grid, pos.0 - 20, pos.1 - 20, pos);
        println!("({},{}) SIZE: {}!", pos.0, pos.1, size);
    }


    result
}

fn paint_grid(grid: &HashMap<(isize, isize), char>, x_off: isize, y_off: isize, pos: (isize, isize)) {
    // find dimensions
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for item in grid.keys() {
        if item.0 < min_x {
            min_x = item.0;
        }

        if item.0 > max_x {
            max_x = item.0;
        }

        if item.1 < min_y {
            min_y = item.1;
        }

        if item.1 > max_y {
            max_y = item.1;
        }
    }

    for y in min_y+y_off..=max_y {
        let mut line: Vec<char> = vec![];

        for x in min_x+x_off..=max_x {
            if x == pos.0 && y == pos.1 {
                line.push('@');
            } else {

            let item = grid.get(&(x,y));
            if item.is_some() {
                line.push(*item.unwrap());
            } else {
                line.push('?');
            }
            }
        }

        let s: String = line.into_iter().collect();
        println!("{}", s);
    }
}

struct Program {
    positions: Vec<isize>,
    instruction_ptr: usize,
    input_ctr: usize,
    relative_base: usize,
    ticks: usize
}

impl Program {
    fn new(source: &str, extra: usize) -> Program {
        let mut positions: Vec<isize> = source.split(",").map(|x| x.parse().unwrap()).collect();
        let mut memory = vec![0; extra];
        positions.append(&mut memory);

        let mut instruction_ptr = 0;
        let mut input_ctr = 0;
        let mut relative_base = 0;
        let mut ticks = 0;

        Program {
            positions,
            instruction_ptr,
            input_ctr,
            relative_base,
            ticks
        }
    }
}


fn run_program(program: &mut Program, inputs: &[isize]) -> Result<isize, isize> {
    let mut output = Ok(0);

    while program.instruction_ptr < program.positions.len() {
        let instruction = program.positions[program.instruction_ptr];
        let opcode = parse_opcode(instruction);
        program.ticks += 1;

        match opcode.op {
            Op::Add => {
                let param1 = get_param_value(&opcode.param1, program, 1);
                let param2 = get_param_value(&opcode.param2, program, 2);
                let param3 = get_param(&opcode.param3, program, 3);
                //debug_op("Add", &program, 3, (param1, param2, get_param_value(&opcode.param3, program, 3)));

                program.positions[param3 as usize] = param1 + param2;
                program.instruction_ptr += 4;
            },
            Op::Multiply => {
                let param1 = get_param_value(&opcode.param1, program, 1);
                let param2 = get_param_value(&opcode.param2, program, 2);
                let param3 = get_param(&opcode.param3, program, 3);
                //debug_op("Multiply", &program, 3, (param1, param2, get_param_value(&opcode.param3, program, 3)));

                program.positions[param3 as usize] = param1 * param2;
                program.instruction_ptr += 4;
            },
            Op::Input => {
                if program.input_ctr >= inputs.len() {
                    output = Err(3);
                    break;
                }

                let param1 = get_param(&opcode.param1, program, 1);
                let input = inputs[program.input_ctr];
                //debug_op("Input", &program, 1, (get_param_value(&opcode.param1, program, 1)));

                program.input_ctr += 1;
                program.positions[param1 as usize] = input;
                program.instruction_ptr += 2;
            },
            Op::Print => {
                let param1 = get_param_value(&opcode.param1, program, 1);
                //debug_op("Output", &program, 1, (param1));

                output = Ok(param1);

                program.instruction_ptr += 2;
                break;
            },
            Op::JumpTrue => {
                let param1 = get_param_value(&opcode.param1, program, 1);
                let param2 = get_param_value(&opcode.param2, program, 2);
                //debug_op("Jump True", &program, 1, (param1, param2));

                if param1 > 0 {
                    program.instruction_ptr = param2 as usize;
                } else {
                    program.instruction_ptr += 3;
                }
            },
            Op::JumpFalse => {
                let param1 = get_param_value(&opcode.param1, program, 1);
                let param2 = get_param_value(&opcode.param2, program, 2);
                //debug_op("Jump If False", &program, 2, (param1, param2));

                if param1 == 0 {
                    program.instruction_ptr = param2 as usize;
                } else {
                    program.instruction_ptr += 3;
                }
            },
            Op::LessThan => {
                let param1 = get_param_value(&opcode.param1, program, 1);
                let param2 = get_param_value(&opcode.param2, program, 2);
                let param3 = get_param(&opcode.param3, program, 3);
                //debug_op("Less Than", &program, 3, (param1, param2, get_param_value(&opcode.param3, program, 3)));

                if param1 < param2 {
                    program.positions[param3 as usize] = 1;
                } else {
                    program.positions[param3 as usize] = 0;
                }

                program.instruction_ptr += 4;
            },
            Op::EqualTo=> {
                let param1 = get_param_value(&opcode.param1, program, 1);
                let param2 = get_param_value(&opcode.param2, program, 2);
                let param3 = get_param(&opcode.param3, program, 3);
                //debug_op("Equal To", &program, 3, (param1, param2, get_param_value(&opcode.param3, program, 3)));

                if param1 == param2 {
                    program.positions[param3 as usize] = 1;
                } else {
                    program.positions[param3 as usize] = 0;
                }

                program.instruction_ptr += 4;
            },
            Op::RelativeBase => {
                let param1 = get_param_value(&opcode.param1, program, 1);
                //debug_op("Adjust RB", &program, 3, (param1));

                program.relative_base = (program.relative_base as isize + param1) as usize;
                program.instruction_ptr += 2;

            },
            Op::Halt | _ => {
                //debug_op("Halt", &program, 3, ());
                output = Err(99);
                break;
            }
        }
    }

    output
}

fn debug_op(op: &str, program: &Program, size: usize, param2: impl std::fmt::Debug) {
    let params = &program.positions[program.instruction_ptr..=program.instruction_ptr+size];
    println!("{}: {} {:?} {:?} RB: {}", program.instruction_ptr, op, params, param2, program.relative_base);
}

#[derive(Debug)]
enum Op {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Print = 4,
    JumpTrue = 5,
    JumpFalse = 6,
    LessThan = 7,
    EqualTo = 8,
    RelativeBase = 9,
    Halt = 99
}

#[derive(Debug)]
enum ParamMode { Position = 0, Immediate = 1, Relative = 2 }

#[derive(Debug)]
struct Opcode {
    op: Op,
    param1: ParamMode,
    param2: ParamMode,
    param3: ParamMode,
}

fn parse_opcode(opcode: isize) -> Opcode {
    let mut operation = Opcode { op: Op::Halt, param1: ParamMode::Position, param2: ParamMode::Position, param3: ParamMode::Position};
    let digits: Vec<u32> = opcode.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
    let fill_amount = 5 - digits.len();
    let filler = vec![0;fill_amount];
    let codes: Vec<u32> = [filler, digits].concat().into_iter().rev().collect();

    operation.op = parse_op(codes[1], codes[0]);
    operation.param1 = parse_param(codes[2]);
    operation.param2 = parse_param(codes[3]);
    operation.param3 = parse_param(codes[4]);

    operation
}

fn parse_op(d1: u32, d2: u32) -> Op {
    let code = (d1 * 10) + d2;
    return match code {
        1 => Op::Add,
        2 => Op::Multiply,
        3 => Op::Input,
        4 => Op::Print,
        5 => Op::JumpTrue,
        6 => Op::JumpFalse,
        7 => Op::LessThan,
        8 => Op::EqualTo,
        9 => Op::RelativeBase,
        99 => Op::Halt,
        _ => panic!("Uknown")
    }
}

fn parse_param(param: u32) -> ParamMode {
    return match param {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        2 => ParamMode::Relative,
        _ => panic!("Uknown")
    }
}

fn get_param(param: &ParamMode, program: &Program, offset: usize) -> isize {
    let value = program.positions[program.instruction_ptr + offset];

    return match param {
        ParamMode::Position => value,
        ParamMode::Immediate => value,
        ParamMode::Relative => program.relative_base as isize + value
    }
}

fn get_param_value(param: &ParamMode, program: &Program, offset: usize) -> isize {
    let value = get_param(param, program, offset);

    return match param {
        ParamMode::Position => program.positions[value as usize],
        ParamMode::Immediate => value,
        ParamMode::Relative => program.positions[value as usize]
    }
}
