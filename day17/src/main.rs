use std::collections::HashMap;

fn main() {
    let mut source : Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut robot = include_str!("./robot.txt");

    //let source : Vec<&str> = include_str!("./input_test.txt").lines().collect();

    let mut a_program = Program::new(source[0], 99999999);
    let mut grid: HashMap<(isize,isize), char> = HashMap::new();
    let mut output = Ok(0);
    let height = 49;
    let width = 44;
    //let height = 43;
    //let width = 39;
    let size = height * width;
    let mut output_ctr = 0;
    let mut outputs: Vec<u8> = vec![32; size];
    let r: Vec<isize> = robot.to_string().into_bytes().iter().map(|x| *x as isize).collect();
    let input = &r;

    loop {

        while output.is_ok() {

            output = run_program(&mut a_program, &input);

            if output.is_err() {
                break;
            }

            if output.unwrap() < 256 {
                outputs[output_ctr] = output.unwrap() as u8;
                output_ctr += 1;
            } else {
                println!("OUTPUT: {}", output.unwrap());
            }

            if output_ctr % size == 0 {
                paint_map(outputs.clone());
                output_ctr = 0;
            }
        }

        if output == Err(3) {
            println!("Asking for input");
        }

        if output == Err(99) {
            println!("Halt!");
            break;
        }
    }

    /*
    let g = String::from_utf8(outputs).unwrap();
    let mut map: Vec<char> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in g.lines() {
        if width < line.len() {
            width = line.len()
        }

        let mut c = line.clone().chars().collect();
        map.append(&mut c);

        height += 1;

        println!("{}", line);
    }

    let mut cross: Vec<(usize, usize)> = Vec::new();

    'crossSearch: for (i, &c) in map.iter().enumerate() {
        if c != '#' {
            continue;
        }

        let x = i as isize  % width as isize;
        let y = i as isize / width as isize;

        let directions = vec![
            (x, y-1),
            (x, y+1),
            (x+1, y),
            (x-1, y)
        ];

        for direction in directions.iter() {
            if direction.0 < 0
                || direction.0 + 1 >= width as isize
                || direction.1 < 0
                || direction.1 + 1 >= height as isize {
                continue 'crossSearch;
            }

            let index = direction.1 * (width as isize) + direction.0;
            if map[index as usize] != '#' {
                continue 'crossSearch;
            }
        }

        cross.push((x as usize, y as usize));
    }

    let answer = cross.iter().fold(0, |acc, x| acc + x.0 * x.1);

    println!("{}", answer);
    */
}

fn paint_map(outputs: Vec<u8>) {
    print!("{}[2J", 27 as char);
    let g = String::from_utf8(outputs).unwrap();
    let mut map: Vec<char> = Vec::new();
    for line in g.lines() {
        let mut c = line.clone().chars().collect();
        map.append(&mut c);
        println!("{}", line);
    }
}

fn paint_grid(grid: &HashMap<(isize, isize), char>) {
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

    for y in min_y..=max_y {
        let mut line: Vec<char> = vec![];

        for x in min_x..=max_x {
            let item = grid.get(&(x,y));
            if item.is_some() {
                line.push(*item.unwrap());
            } else {
                line.push('?');
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
