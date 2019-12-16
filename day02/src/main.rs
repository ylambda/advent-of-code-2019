fn main() {

    let answer = 19690720;

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            if run_program(noun, verb) == answer {
                println!("{}{}", noun, verb);
                break 'outer;
            }
        }
    }
}

fn run_program(noun: usize, verb: usize) -> usize {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let input = lines[0];

    let mut positions: Vec<usize> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let mut instruction_ptr = 0;

    // Modify the program
    positions[1] = noun;
    positions[2] = verb;

    while instruction_ptr < positions.len() {
        let instruction = positions[instruction_ptr];

        if instruction == 99 {
            break;
        }

        let input_a_ptr = positions[instruction_ptr+1];
        let input_b_ptr = positions[instruction_ptr+2];
        let dest = positions[instruction_ptr+3];

        if instruction == 1 {
            positions[dest] = positions[input_a_ptr] + positions[input_b_ptr];
        }

        if instruction == 2 {
            positions[dest] = positions[input_a_ptr] * positions[input_b_ptr];
        }

        instruction_ptr += 4;

    }

    positions[0]
}
