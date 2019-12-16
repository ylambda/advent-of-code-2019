use std::collections::HashSet;

#[derive(Debug)]
enum CardinalDirection {Up, Down, Left, Right}

#[derive(Debug)]
struct Instruction {
    direction: CardinalDirection,
    distance: isize
}

impl Instruction {
    fn new(inst: String) -> Instruction {
        let direction = match inst.get(0..1).unwrap() {
            "U" => CardinalDirection::Up,
            "D" => CardinalDirection::Down,
            "L" => CardinalDirection::Left,
            "R" => CardinalDirection::Right,
            _ => panic!("Unknown direction")
        };

        let distance = inst[1..].parse().unwrap();

        Instruction {
            direction,
            distance
        }
    }
}

fn main() {
    let input = include_str!("./input.txt").lines().collect();
    let best = part1(&input);
    println!("Best: {}", best);
    let p2 = part2(&input);
    println!("P2 Best: {}", p2);
}

fn part1(lines: &Vec<&str>) -> isize {
    let wire1: Vec<Instruction> = lines[0].split(",").map(|x| Instruction::new(x.to_string())).collect();
    let wire2: Vec<Instruction> = lines[1].split(",").map(|x| Instruction::new(x.to_string())).collect();
    let mut grid1: HashSet<(isize,isize)> = HashSet::new();
    let mut grid2: HashSet<(isize,isize)> = HashSet::new();
    let mut wire1_order: Vec<(isize, isize)> = vec!();
    let mut wire2_order: Vec<(isize, isize)> = vec!();

    crawl_wire(&wire1, &mut grid1, &mut wire1_order);
    crawl_wire(&wire2, &mut grid2, &mut wire1_order);

    let mut best = 999999;
    for (x,y) in grid2.intersection(&grid1) {
        let distance = x.abs() + y.abs();
        if distance < best {
            best = distance;
        }
    }

    best
}

fn part2(lines: &Vec<&str>) -> isize {
    let wire1: Vec<Instruction> = lines[0].split(",").map(|x| Instruction::new(x.to_string())).collect();
    let wire2: Vec<Instruction> = lines[1].split(",").map(|x| Instruction::new(x.to_string())).collect();
    let mut grid1: HashSet<(isize,isize)> = HashSet::new();
    let mut grid2: HashSet<(isize,isize)> = HashSet::new();
    let mut wire1_order: Vec<(isize, isize)> = vec!();
    let mut wire2_order: Vec<(isize, isize)> = vec!();

    crawl_wire(&wire1, &mut grid1, &mut wire1_order);
    crawl_wire(&wire2, &mut grid2, &mut wire2_order);

    let mut best: isize = 999999;
    for (x,y) in grid2.intersection(&grid1) {
        let wire1_steps = wire1_order.iter().position(|&coord| coord == (*x,*y)).unwrap();
        let wire2_steps = wire2_order.iter().position(|&coord| coord == (*x,*y)).unwrap();
        let total = (wire1_steps + wire2_steps) as isize;
        if total < best {
            best = total
        }
    }

    best
}

fn crawl_wire(wire: &Vec<Instruction>, grid: &mut HashSet<(isize,isize)>, order: &mut Vec<(isize, isize)>) {
    let mut x: isize = 0;
    let mut y: isize = 0;

    for segment in wire {
        let mut distance = 0;
        let original_x = x;
        let original_y = y;
        while distance < segment.distance {
            match segment.direction {
                CardinalDirection::Up => y += 1,
                CardinalDirection::Down => y -= 1,
                CardinalDirection::Left => x -= 1,
                CardinalDirection::Right => x += 1
            };

            let coord = (x,y);
            let cell = grid.insert(coord);
            order.push((x, y));

            distance += 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_1() {
        let lines: Vec<&str> = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83".lines().collect();
        let actual = part1(lines);
        let expected = 159;
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_2() {
        let lines: Vec<&str> = "R8,U5,L5,D3
U7,R6,D4,L4".lines().collect();
        let actual = part1(lines);
        let expected = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_3() {
        let lines: Vec<&str> = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".lines().collect();
        let actual = part1(lines);
        let expected = 135;
        assert_eq!(actual, expected);
    }

}
