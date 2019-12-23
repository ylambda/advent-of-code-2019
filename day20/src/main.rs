use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::iter::FromIterator;

fn main() {
    let input = include_str!("./input.txt");
    let maze = Maze::from_input(input);
    maze.render();
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {

    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

}

impl Add for Coord {
    type Output = Self;

    fn add(self: Coord, other: Coord) -> Coord {
        Coord { 
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self: Coord, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

}

impl Default for Coord {

    fn default() -> Self {
        Coord::new(0, 0)
    }

}

enum Tile {
    Open,
    Wall,
    Portal(String, bool)
}

struct Maze {
    grid: HashMap<Coord, Tile>,
    portals: HashMap<String, Vec<Coord>>,
    width: isize,
    height: isize
}

impl Maze {

    fn from_input(input: &str) -> Maze {
        let mut grid = HashMap::new();
        let mut portals: HashMap<String, Vec<Coord>> = HashMap::new();
        let mut portal_letters: HashMap<Coord,char> = HashMap::new();

        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;

                let pos = Coord::new(x, y);

                if x as isize > width {
                    width = x;
                }

                if y as isize > height {
                    height = y;
                }

                if c == '#' {
                    grid.insert(pos, Tile::Wall);
                }

                if c == '.' {
                    grid.insert(pos, Tile::Open);
                }

                if c.is_ascii_uppercase() {
                    portal_letters.insert(pos, c);
                }
            }
        }

        for (&pos, &letter) in portal_letters.iter() {
            for step in get_steps() {
                let other_pos = pos + step;
                if let Some(&other_letter) = portal_letters.get(&other_pos) {
                    let open_pos = pos - step;
                    if let Some(Tile::Open) = grid.get(&open_pos) {
                        let name = if pos.manhattan_distance() < other_pos.manhattan_distance() {
                            String::from_iter(vec![letter, other_letter])
                        } else {
                            String::from_iter(vec![other_letter, letter])
                        };

                        let outside = pos.x < 2 || pos.y < 2 || pos.x > width - 2 || pos.y > height - 2;
                        grid.insert(pos, Tile::Portal(name.clone(), outside));
                        portals.entry(name).or_default().push(open_pos)
                    }
                }
            }
        }

        Maze { grid, portals, width, height }
    }

    fn render(&self) {

        for y in 0..self.height{
            let mut line = String::new();

            for x in 0..self.width {
                let pos = Coord::new(x,y);
                match self.grid.get(&pos) {
                    Some(Tile::Wall) => line.push('#'),
                    Some(Tile::Open) => line.push('.'),
                    _ => line.push(' ')
                }
            }

            println!("{}", line);
        }
    }
}

fn get_steps() -> Vec<Coord> {

    vec![
        Coord::new(1, 0),
        Coord::new(-1, 0),
        Coord::new(0, 1),
        Coord::new(0, -1)
    ]

}

