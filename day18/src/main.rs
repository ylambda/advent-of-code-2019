use std::collections::HashMap;
use std::ops::{Add,Sub};

use pathfinding::directed::bfs::bfs;
use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    let input = include_str!("./input_test_1.txt");
    let maze = Maze::from_input(&input);
    maze.render();

    println!("{}", part_1(&maze));
}

fn part_1(maze: &Maze) -> isize {
    let keys_to_keys: HashMap<String, KeyDistance> = HashMap::new();

    for (src_key, src_pos) in maze.keys.iter() {
        for (dest_key, dest_pos) in maze.keys.iter() {

            if dest_key == src_key {
                continue;
            }

            let name = if dest_key > src_key {
                dest_key.clone() + &src_key.clone()
            } else {
                src_key.clone() + &dest_key.clone()
            };

            if keys_to_keys.contains_key(&name) {
                continue;
            }

            let origin = Node(*src_pos, String::new());
            let (node, steps) = bfs(
                &origin,
                |Node(pos, keys)| get_successors(maze, &pos, &keys),
                |Node(pos, keys)| pos == dest_pos
                );

            println!("Key: {} Steps: {}", name, steps);
            keys_to_keys.insert(name, KeyDistance(steps, node.0));
        }
    }


    /*
    let start = Node(maze.entrance, String::new());
    let (_, steps) = dijkstra(
        &start,
        |Node(pos, keys)| get_successors(maze, pos, keys),
        |Node(pos, keys)| keys.len() == maze.keys.len()
        )
    .expect("Couldn't find all the keys");
    */

    0
}

fn get_successors(maze: &Maze, pos: &Coord, keys: &String) -> Vec<(Node, isize)> {
    get_neighbors(*pos)
        .iter()
        .filter_map(|neighbor| {
            match maze.grid.get(pos) {
                Some(Tile::Key(letter)) => {
                    let new_keys = if !keys.contains(letter) {
                        keys.clone() + letter
                    } else {
                        keys.clone()
                    };
                    Some((Node(*neighbor, new_keys), 1))
                },
                Some(tile) if can_traverse(tile, keys)  => {
                    Some((Node(*neighbor, keys.clone()), 1))
                },
                _ => None
            }
        })
        .collect::<Vec<_>>()
}

fn can_traverse(tile: &Tile, keys: &String) -> bool {


    match tile {
        Tile::Wall => false,
        Tile::Open => true,
        Tile::Key(_) => true,
        Tile::Door(letter) => {
            keys.contains(&letter.to_lowercase().to_string())
        }
    }

}

fn get_neighbors(pos: Coord) -> Vec<Coord> {
    return vec![
        pos + Coord::new(1, 0),
        pos + Coord::new(-1, 0),
        pos + Coord::new(0, 1),
        pos + Coord::new(0, -1)
    ];
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node(Coord, String);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct KeyDistance(isize, String);

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
    Door(String),
    Key(String)
}

struct Maze {
    grid: HashMap<Coord, Tile>,
    keys: HashMap<String, Coord>,
    doors: HashMap<String, Coord>,
    entrance: Coord,
    width: isize,
    height: isize
}

impl Maze {

    fn from_input(input: &str) -> Maze {
        let mut grid = HashMap::new();
        let mut keys: HashMap<String, Coord> = HashMap::new();
        let mut doors: HashMap<String, Coord> = HashMap::new();
        let mut entrance = Coord::new(0,0);

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

                if c == '@' {
                    entrance = pos;
                    grid.insert(pos, Tile::Open);
                }

                if c.is_ascii_uppercase() {
                    doors.insert(c.to_lowercase().to_string(), pos);
                    grid.insert(pos, Tile::Door(c.to_string()));
                }

                if c.is_ascii_lowercase() {
                    keys.insert(c.to_lowercase().to_string(), pos);
                    grid.insert(pos, Tile::Key(c.to_string()));
                }
            }
        }

        Maze { grid, doors, keys, width, height, entrance }
    }

    fn render(&self) {

        for y in 0..=self.height{
            let mut line = String::new();

            for x in 0..=self.width {
                let pos = Coord::new(x,y);
                match self.grid.get(&pos) {
                    Some(Tile::Wall) => line.push('#'),
                    Some(Tile::Open) => line.push('.'),
                    Some(Tile::Door(letter)) => line.push(letter.chars().next().unwrap()),
                    Some(Tile::Key(letter)) => line.push(letter.chars().next().unwrap()),
                    _ => line.push(' '),
                }
            }

            println!("{}", line);
        }
    }
}
