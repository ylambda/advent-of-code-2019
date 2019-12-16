use std::cmp::Ordering;

fn main() {
    let input: Vec<&str>  = include_str!("./input_a.txt").lines().collect();
    let mut asteroids: Vec<Point> = Vec::new();

    for (y, &line) in input.iter().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                asteroids.push(Point::new(x as f32, y as f32));
            }
        }
    }

    let mut counts: Vec<Vec<f32>> = Vec::new();

    for asteroid in asteroids.iter() {
        let mut set: Vec<f32> = Vec::new();

        for b in asteroids.iter() {
            if asteroid.x != b.x || asteroid.y != b.y {
                let angle = asteroid.angle(b);
                if !set.contains(&angle) {
                    set.push(angle);
                }
            }
        }

        counts.push(set);
    }

    let mut highest = 0;
    let mut station_idx = 0;
    for (i, c) in counts.iter().enumerate() {
        if c.len() > highest {
            highest = c.len();
            station_idx = i;
        }
    }

    let station = &asteroids[station_idx];
    println!("{}", highest);
    println!("{:?}", station);

    let mut station_angles: Vec<(&Point, f32, bool)> = Vec::new();
    for b in asteroids.iter() {
        if station.x == b.x && station.y == b.y {
        } else {
            let angle = station.angle(b);
            station_angles.push((b, angle, false));
        }
    }

    station_angles.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    let mut last_angle: f32 = -0.1;
    let mut count = 0;
    let mut idx = 0;

    while count < 203 {
        let mut el = &mut station_angles[idx];
        if el.1 != last_angle && !el.2{
            last_angle = el.1;
            count += 1;
            el.2 = true;
            println!("{} {:?}", count, el);
        } else {
            if !el.2 {
                println!("Skipping {:?}", el);
            }
        }

        idx = (idx + 1) % station_angles.len();
    }

    println!("{:?}", station_angles[idx-1]);
    println!("{:?}", (station_angles[idx-1].0.x * 100.0) + station_angles[idx-1].0.y)
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32 
}

impl Point {

    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    fn angle(&self, p2: &Point) -> f32 {
        let delta_x = p2.x - self.x;
        let delta_y = p2.y - self.y;

        let mut r = delta_x.atan2(delta_y) * (-180.0 / std::f32::consts::PI);
        r = r - 180.0;


        ((r % 360.0) + 360.0) % 360.0

    }
}
