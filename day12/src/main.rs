use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let mut lines = include_str!("./input.txt").lines();
    let mut x_states: HashSet<String> = HashSet::new();
    let mut y_states: HashSet<String> = HashSet::new();
    let mut z_states: HashSet<String> = HashSet::new();

    let mut moons = [
        Moon::from_str(lines.next().unwrap()),
        Moon::from_str(lines.next().unwrap()),
        Moon::from_str(lines.next().unwrap()),
        Moon::from_str(lines.next().unwrap())
    ];

    let mut i = 0;
    let mut moon_x_repeat = None;
    let mut moon_y_repeat = None;
    let mut moon_z_repeat = None;

    while moon_x_repeat.is_none() || moon_y_repeat.is_none() || moon_z_repeat.is_none() {
        update_velocity(&mut moons);
        update_position(&mut moons);

        let (mut moon_str_x, mut moon_str_y, mut moon_str_z)  = Moon::to_str(&moons);

        if x_states.contains(&moon_str_x) && moon_x_repeat.is_none() {
            moon_x_repeat = Some(i);
        }

        if y_states.contains(&moon_str_y) && moon_y_repeat.is_none() {
            moon_y_repeat = Some(i);
        }

        if z_states.contains(&moon_str_z) && moon_z_repeat.is_none() {
            moon_z_repeat = Some(i);
        }

        if (i == 44) {
            print_moon(&moons);
        }

        if i % 100 == 0 {
            println!("{} {} {}", moon_str_x, moon_str_y, moon_str_z);
        }

        x_states.insert(moon_str_x);
        y_states.insert(moon_str_y);
        z_states.insert(moon_str_z);

        if i % 100 == 0 {
            println!("Iteration {}", i);
            println!("Moon States X: {} Repeat: {:?}", x_states.len(), moon_x_repeat);
            println!("Moon States Y: {} Repeat: {:?}", y_states.len(), moon_y_repeat);
            println!("Moon States Z: {} Repeat: {:?}", z_states.len(), moon_z_repeat);
        }

        i += 1;
    }

    //let energy = calculate_energy(&moons);
    //println!("Energy: {}", energy);
    println!("Moon States X: {} Repeat: {:?}", x_states.len(), moon_x_repeat);
    println!("Moon States Y: {} Repeat: {:?}", y_states.len(), moon_y_repeat);
    println!("Moon States Z: {} Repeat: {:?}", z_states.len(), moon_z_repeat);
}

fn print_moon(moons: &[Moon]) {
    for moon in moons.iter() {
        println!("{:?}", moon);
    }
}

fn calculate_energy(moons: &[Moon]) -> isize {
    moons.iter().fold(0, |acc, moon| {
        let potential = moon.x.abs() + moon.y.abs() + moon.z.abs();
        let kinetic = moon.vx.abs() + moon.vy.abs() + moon.vz.abs();
        acc + (potential * kinetic)
    })
}

fn update_velocity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            let dx = Moon::cmp_axis(moons[i].x, moons[j].x);
            moons[i].vx += dx;
            moons[j].vx -= dx;
            let dy = Moon::cmp_axis(moons[i].y, moons[j].y);
            moons[i].vy += dy;
            moons[j].vy -= dy;
            let dz = Moon::cmp_axis(moons[i].z, moons[j].z);
            moons[i].vz += dz;
            moons[j].vz -= dz;
        }
    }
}

fn update_position(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        moons[i].x += moons[i].vx;
        moons[i].y += moons[i].vy;
        moons[i].z += moons[i].vz;
    }
}

#[derive(Debug)]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize
}

impl Moon {

    fn new(x: isize, y: isize, z: isize) -> Moon {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0
        }
    }

    fn from_str(s: &str) -> Moon {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let x: isize = caps.get(1).unwrap().as_str().parse().unwrap();
        let y: isize = caps.get(2).unwrap().as_str().parse().unwrap();
        let z: isize = caps.get(3).unwrap().as_str().parse().unwrap();

        return Moon::new(x, y, z);
    }

    fn cmp_axis(a1: isize, a2: isize) -> isize {
        let mut r = 0;
        if a1 > a2 {
            r = -1;
        }

        if a1 < a2 {
            r = 1;
        }

        r

    }

    fn to_str(moons: &[Moon]) -> (String, String, String) {
        let mut s_x = String::new();
        let mut s_y = String::new();
        let mut s_z = String::new();

        for moon in moons.iter() {
            s_x.push_str(&format!("|{}|{}", moon.x, moon.vx));
            s_y.push_str(&format!("|{}|{}", moon.y, moon.vy));
            s_z.push_str(&format!("|{}|{}", moon.z, moon.vz));
        }

        (s_x, s_y, s_z)
    }
}
