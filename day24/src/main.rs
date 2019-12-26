use std::collections::{HashMap,HashSet};
use std::ops::{Add, Sub};

fn main() {
    let input = include_str!("./input.txt");
    let mut planet = Eris::from_str(input);

    //planet.render();
    for _ in 0..100 {
        planet.update();

        //let hash = planet.hash();
        //if planet.hash.contains(&hash) {
            //planet.render();
            //println!("Bio: {}", planet.biodiversity());
            //panic!("Done");
        //} else {
            //planet.hash.insert(hash);
        //}

        //planet.render();
        println!("");
    }

    let r = planet.grid.values().fold(0, |acc, &c| {
        return acc + match c {
            Tile::Bug => 1,
            _ => 0
        }
    });

    print!("{}", r);
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Bug
}


#[derive(Debug, Clone)]
struct Eris {
    grid: HashMap<GridPosition, Tile>,
    hash: HashSet<String>,
    height: isize,
    width: isize,
    ticks: usize
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
struct GridPosition(isize, Coord);

impl Eris {

    fn from_str(input: &str) -> Eris {
        let mut grid: HashMap<GridPosition, Tile> = HashMap::new();
        let hash: HashSet<String> = HashSet::new();
        let mut height = 0;
        let mut width = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                if x > width {
                    width = x;
                }

                if y > height {
                    height = y;
                }

                let pos = Coord::new(x, y as isize);

                match c {
                    '#' => grid.insert(GridPosition(0, pos), Tile::Bug),
                    _ => grid.insert(GridPosition(0, pos), Tile::Empty)
                };
            }
        }

        Eris { grid, height, width, ticks: 0, hash }
    }

    fn update(&mut self) {
        let mut grid2: HashMap<GridPosition, Tile> = HashMap::new();

        for (&pos, &tile) in self.grid.iter() {
            let count = self.get_neighbors(pos).iter().fold(0, |acc, p| {
                acc + match self.grid.get(&p) {
                    Some(Tile::Empty) => 0,
                    Some(Tile::Bug) => 1,
                    _ => 0
                }
            });

            match tile {
                Tile::Empty => {
                    match count {
                        1|2 => grid2.insert(pos, Tile::Bug),
                        _ => grid2.insert(pos, tile)
                    }
                },
                Tile::Bug => {
                    match count {
                        0|2..=4 => grid2.insert(pos, Tile::Empty),
                        _ => grid2.insert(pos, tile)
                    }
                }
            };
        }

        self.grid = grid2;
        self.ticks += 1;
    }

    fn get_neighbors(&self, gp: GridPosition) -> Vec<GridPosition> {
        let level = gp.0;
        let pos = gp.1;
        let mut neighbors: Vec<GridPosition> = vec![];

        if pos.x == 1 && pos.y == 2 {
            for y in 0..=self.height {
                neighbors.push(GridPosition(level+1, Coord::new(0, y)));
            }
        } else if pos.x == self.width {
            neighbors.push(GridPosition(level-1, Coord::new(3,2)));
        } else {
            neighbors.push(GridPosition(level, pos + Coord::new(1,0)));
        }

        if pos.x == 3 && pos.y == 2 {
            for y in 0..=self.height {
                neighbors.push(GridPosition(level+1, Coord::new(self.width, y)));
            }
        } else if pos.x == 0 {
            neighbors.push(GridPosition(level-1, Coord::new(1,2)));
        } else {
            neighbors.push(GridPosition(level, pos + Coord::new(-1,0)));
        }

        if pos.x == 2 && pos.y == 1 {
            for x in 0..=self.height {
                neighbors.push(GridPosition(level+1, Coord::new(x, 0)));
            }
        } else if pos.y == self.height {
            neighbors.push(GridPosition(level-1, Coord::new(2,3)));
        } else {
            neighbors.push(GridPosition(level, pos + Coord::new(0,1)));
        }

        if pos.x == 2 && pos.y == 3 {
            for x in 0..=self.height {
                neighbors.push(GridPosition(level+1, Coord::new(x, self.height)));
            }
        } else if pos.y == 0 {
            neighbors.push(GridPosition(level-1, Coord::new(2, 1)));
        } else {
            neighbors.push(GridPosition(level, pos + Coord::new(0,-1)));
        }

        println!("Origin: {:?} Neighbors: {:?}", pos, neighbors);

        neighbors

    }

    /*
    fn hash(&self) -> String {
        let mut s = String::new();

        for x in 0..=self.width {
            for y in 0..=self.height {
                let pos = Coord::new(x, y);
                let tile = self.grid.get(&pos).unwrap();
                match tile {
                    Tile::Empty => s.push('.'),
                    Tile::Bug => s.push('#')
                }
            }
        }

        s
    }

    fn biodiversity(&self) -> isize {
        let mut num = 0;

        for i in 0..((self.width + 1) * (self.height + 1)) {
            let x = i % (self.width + 1);
            let y = i / (self.width + 1);
            let pos = Coord::new(x, y);
            let tile = self.grid.get(&pos).unwrap();
            num = match tile {
                Tile::Empty => num,
                Tile::Bug => num + 2_isize.pow(i as u32)
            };
        }

        num
    }

    fn render(&self) {

        for y in 0..=self.height{
            let mut line = String::new();

            for x in 0..=self.width {
                let pos = Coord::new(x,y);
                match self.grid.get(&pos) {
                    Some(Tile::Bug) => line.push('#'),
                    Some(Tile::Empty) => line.push('.'),
                    _ => line.push(' ')
                }
            }

            println!("{}", line);
        }
    }
    */

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
