use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<&str> = include_str!("./input_a.txt").lines().collect();
    let mut db: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut rdb: HashMap<&str, &str> = HashMap::new();

    for line in input.iter() {
        let orbit: Vec<&str> = line.split(")").collect();
        let target = orbit[0];
        let subject = orbit[1];

        let mut entry = db.entry(target).or_insert(vec![]);
        entry.push(subject);
        db.entry(subject).or_insert(vec![]);

        rdb.entry(subject).or_insert(target);
    }

    let mut total = 0;
    for orbit in db.keys() {
        total += count_orbits(&db, orbit);
    }

    println!("Total: {}", total);

    let mut san = "SAN";
    let mut you = "YOU";
    let mut san_path: HashSet<&str> = HashSet::new();
    let mut you_path: HashSet<&str> = HashSet::new();

    loop {
        if san == "COM" {
            san_path.insert(san);
            break;
        }
        san = rdb.get(san).unwrap();
        san_path.insert(san);
    }
    
    loop {
        if you == "COM" {
            you_path.insert(you);
            break;
        }

        you = rdb.get(you).unwrap();
        you_path.insert(you);
    }

    let diff2: HashSet<_> = you_path.symmetric_difference(&san_path).collect();
    println!("Total 2: {:?}", diff2.len());
}

fn count_orbits(db: &HashMap<&str, Vec<&str>>, id: &str) -> usize {
    let orbits = db.get(id).unwrap();

    let mut count = 0;
    for orbit in orbits {
        count += count_orbits(&db, orbit);
    }

    return count + orbits.len();
}
