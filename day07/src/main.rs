use std::collections::HashSet;

fn main() {
    let source : Vec<&str> = include_str!("./input.txt").lines().collect();
    //let source : Vec<&str> = include_str!("./input_test.txt").lines().collect();
    let mut final_out = 0;

    let phases = make_phases(5, 9);
    for phase in phases.iter() {
        let mut program_ctr = 0;
        let mut a_program = parse_source(source[0]);
        let mut b_program = parse_source(source[0]);
        let mut c_program = parse_source(source[0]);
        let mut d_program = parse_source(source[0]);
        let mut e_program = parse_source(source[0]);

        let mut output = Ok(0);

        while output.is_ok() {
            let amp = program_ctr % 5;
            let mut program = match amp {
                0 => &mut a_program,
                1 => &mut b_program,
                2 => &mut c_program,
                3 => &mut d_program,
                4 => &mut e_program,
                _ => panic!("Unknown")
            };

            output = run_program(&mut program, &[phase[amp], output.unwrap()]);

            if program_ctr % 5 == 4 {
                if output.is_ok() && output.unwrap() > final_out {
                    final_out = output.unwrap();
                    println!("Final out is {}", final_out);
                }
            }

            program_ctr += 1;

        }
    }

    println!("Output: {}", final_out);
}

fn parse_source(source: &str) -> (Vec<isize>, usize, usize) {
    let mut positions: Vec<isize> = source.split(",").map(|x| x.parse().unwrap()).collect();
    let mut instruction_ptr = 0;
    let mut input_ctr = 0;

    (positions, instruction_ptr, input_ctr)
}

fn run_program(program: &mut(Vec<isize>, usize, usize), inputs: &[isize]) -> Result<isize, isize> {
    let mut positions = &mut program.0;
    let mut instruction_ptr = program.1;
    let mut input_ctr = program.2;
    let mut output = Ok(0);

    while instruction_ptr < positions.len() {
        let instruction = positions[instruction_ptr];
        let opcode = parse_opcode(instruction);

        match opcode.op {
            Op::Add => {
                let param1 = get_param_value(opcode.param1, positions[instruction_ptr+1], &positions);
                let param2 = get_param_value(opcode.param2, positions[instruction_ptr+2], &positions);
                let param3 = positions[instruction_ptr+3];

                positions[param3 as usize] = param1 + param2;
                instruction_ptr += 4;
            },
            Op::Multiply => {
                let param1 = get_param_value(opcode.param1, positions[instruction_ptr+1], &positions);
                let param2 = get_param_value(opcode.param2, positions[instruction_ptr+2], &positions);
                let param3 = positions[instruction_ptr+3];

                positions[param3 as usize] = param1 * param2;
                instruction_ptr += 4;
            },
            Op::Input => {
                let param1 = positions[instruction_ptr+1];
                let input = inputs[input_ctr];

                if input_ctr == 0 {
                    input_ctr = 1;
                }

                //println!("Input: {}", input);
                positions[param1 as usize] = input;
                instruction_ptr += 2;
            },
            Op::Print => {
                let param1 = get_param_value(opcode.param1, positions[instruction_ptr+1], &positions);
                //println!("Print: {}", param1);
                output = Ok(param1);
                instruction_ptr += 2;
                break;
            },
            Op::JumpTrue => {
                let param1 = get_param_value(opcode.param1, positions[instruction_ptr+1], &positions);
                let param2 = get_param_value(opcode.param2, positions[instruction_ptr+2], &positions);

                if param1 > 0 {
                    instruction_ptr = param2 as usize;
                } else {
                    instruction_ptr += 3;
                }
            },
            Op::JumpFalse => {
                let param1 = get_param_value(opcode.param1, positions[instruction_ptr+1], &positions);
                let param2 = get_param_value(opcode.param2, positions[instruction_ptr+2], &positions);

                if param1 == 0 {
                    instruction_ptr = param2 as usize;
                } else {
                    instruction_ptr += 3;
                }
            },
            Op::LessThan => {
                let param1 = get_param_value(opcode.param1, positions[instruction_ptr+1], &positions);
                let param2 = get_param_value(opcode.param2, positions[instruction_ptr+2], &positions);
                let param3 = positions[instruction_ptr+3] as usize;

                if param1 < param2 {
                    positions[param3] = 1;
                } else {
                    positions[param3] = 0;
                }

                instruction_ptr += 4;
            },
            Op::GreaterThan => {
                let param1 = get_param_value(opcode.param1, positions[instruction_ptr+1], &positions);
                let param2 = get_param_value(opcode.param2, positions[instruction_ptr+2], &positions);
                let param3 = positions[instruction_ptr+3] as usize;

                if param1 == param2 {
                    positions[param3] = 1;
                } else {
                    positions[param3] = 0;
                }

                instruction_ptr += 4;
            },
            Op::Halt => {
                output = Err(99);
                println!("done");
                break;
            }
        }
    }

    program.1 = instruction_ptr;
    program.2 = input_ctr;

    output
}

#[derive(Debug)]
enum Op {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Print = 4,
    JumpTrue = 5,
    JumpFalse= 6,
    LessThan = 7,
    GreaterThan = 8,
    Halt = 99
}

#[derive(Debug)]
enum ParamMode { Position = 0, Immediate = 1 }

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
    let codes = [filler, digits].concat();

    operation.op = parse_op(codes[3], codes[4]);
    operation.param1 = parse_param(codes[2]);
    operation.param2 = parse_param(codes[1]);
    operation.param3 = parse_param(codes[0]);

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
        8 => Op::GreaterThan,
        99 => Op::Halt,
        _ => panic!("Uknown")
    }
}

fn parse_param(param: u32) -> ParamMode {
    return match param {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        _ => panic!("Uknown")
    }
}

fn get_param_value(param: ParamMode, value: isize, positions: &Vec<isize>) -> isize {
    return match param {
        ParamMode::Position => positions[value as usize],
        ParamMode::Immediate => value
    }
}

fn make_phases(start: isize, end: isize) -> Vec<Vec<isize>> {
    let mut result = vec![];
    for a in start..=end {
        for b in start..=end {
            for c in start..=end {
                for d in start..=end {
                    for e in start..=end {

                        let mut set: HashSet<isize> = HashSet::new();
                        set.insert(a);
                        set.insert(b);
                        set.insert(c);
                        set.insert(d);
                        set.insert(e);

                        if set.len() == 5 {
                            result.push(vec![a,b,c,d,e]);
                        }
                    }
                }
            }
        }
    }

    result
}
