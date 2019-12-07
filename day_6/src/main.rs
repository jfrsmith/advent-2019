use std::collections::HashMap;

struct SolarSystem {
    planets : HashMap<String, String>
}

impl SolarSystem {
    fn new(in_map: &'static str) -> SolarSystem {
        let mut map = HashMap::new();

        for l in in_map.lines() {
            let split = l.split(')').map(|s| s.trim()).collect::<Vec<&str>>();
            map.entry(split[1].to_string()).or_insert(split[0].to_string());
        }

        SolarSystem {
            planets : map
        }
    }

    fn get_path(&self, from: &str, to: &str) -> Vec<String> {
        let mut path = vec!(from.to_string());
        let mut parent = self.planets.get(from).unwrap();
    
        while parent != to {
            path.push(parent.to_string());
            parent = self.planets.get(parent).unwrap();
        }
    
        path
    }

    fn find_lca(&self, a: &str, b: &str) -> String {
        let path_a = self.get_path(a, "COM");
        let path_b = self.get_path(b, "COM");

        let mut lca = "COM";

        for (i, (a, b)) in path_a.iter().rev().zip(path_b.iter().rev()).enumerate() {
            if a != b {
                lca = &path_a[path_a.len() - i];
                break;
            }
        }

        lca.to_string()
    }

    fn get_orbital_transfer_count(&self, from: &str, to: &str) -> u32 {
        let lca = self.find_lca(from, to);
        (self.get_path(from, &lca).len() as u32 - 1) + (self.get_path(to, &lca).len() as u32 - 1)
    }

    fn get_total_orbit_count(&self) -> u32 {
        self.planets.keys().fold(0, |acc, p| acc + self.get_path(p, "COM").len() as u32)
    }
}

fn main() {
    let solar_system = SolarSystem::new(include_str!("../input/day_6.txt"));
    println!("Part 1 => {}", solar_system.get_total_orbit_count());
    println!("Part 2 => {}", solar_system.get_orbital_transfer_count("YOU", "SAN"));
}

#[test]
fn part_1() {
    let orbits = "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L";

    assert_eq!(SolarSystem::new(orbits).get_total_orbit_count(), 42);
}

#[test]
fn part_1_complete() {
    assert_eq!(SolarSystem::new(include_str!("../input/day_6.txt")).get_total_orbit_count(), 315757);
}

#[test]
fn part_2() {
    let orbits = "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L
    K)YOU
    I)SAN";
    
    assert_eq!(SolarSystem::new(orbits).get_orbital_transfer_count("YOU", "SAN"), 4);
}

#[test]
fn part_2_complete() {
    assert_eq!(SolarSystem::new(include_str!("../input/day_6.txt")).get_orbital_transfer_count("YOU", "SAN"), 481);
}