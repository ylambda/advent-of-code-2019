use std::ops::Range;

fn main() {
    let input = include_str!("./input.txt").lines().next().unwrap();
    //let input = include_str!("./input_test.txt").lines().next().unwrap();
    let mut phase: Vec<isize> = input.chars().map(|c| c.to_digit(10).unwrap() as isize).collect();
    let pattern: Vec<isize> = vec![0,1,0,-1];
    let offset = input[0..7].parse::<usize>().unwrap();

    for _ in 1..10_000 {
        for i in 0..input.len() {
            phase.push(phase[i]);
        }
    }

    //println!("{:?}", phase);
    println!("{:?}", offset);
    let mut phase2 = &mut phase[offset..];

    for _ in 0..100 {
        for i in (0..phase2.len()).rev() {
            let prev = if (i+1) == phase2.len() {
                0
            } else {
                phase2[i + 1]
            };

            phase2[i] = (prev + phase2[i]).abs() % 10;
        }
    }

    for i in 0..8 {
        println!("{}", phase2[i]);
    }
}


fn fft(input: &Vec<isize>, base_pattern: &Vec<isize>) -> Vec<isize> {

    let mut output = vec![0; input.len()];
    let mut lines: Vec<String> = Vec::new();

    for idx in 0..input.len() {
        let mut s: Vec<String> = Vec::new();

        let base = idx + 1;
        let mut prev_base = 0;
        let mut ranges: Vec<Range<usize>> = Vec::new();
        for i in 0..(input.len() / base) {
            let start = prev_base + i*base;
            let end = (i*base) + prev_base;
            prev_base = end;
            ranges.push(Range { start, end });
        }

        let r = input.iter().enumerate().fold(0, |acc, (i, &item)| {
            let p =  (i+1) / (idx + 1);
            let modifier = base_pattern[p % base_pattern.len()];
            s.push(format!("{}*{}", item, modifier));
            acc + (item * modifier)
        }).abs() % 10;

        output[idx] = r;
    }
    output
}
