fn main() {
    let input = include_str!("input.txt");

    let mut total = 0;

    for line in input.lines() {
        let mass: i32  = line.parse().unwrap();
        total += mass / 3 - 2;
    }

    println!("Part 1: {}", total);

    let mut part2 = 0;
    for line in input.lines() {
        let mass: i32 = line.parse().unwrap();
        part2 += fuel_required(mass);
    }

    println!("Part 2: {}", part2);
}

fn fuel_required(mass: i32) -> i32 {
    let mut total = 0;
    let mut subtotal = mass / 3 - 2;

    while subtotal > 0 {
        total += subtotal;
        subtotal = subtotal / 3 - 2;
    }

    total
}
