use std::env;
use std::fs;
use std::collections::HashMap;

struct Cube {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum TerrainType {
    Lava,
    AirPocket,
    Air
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_cubes(input: &str) -> Vec<Cube> {
    return input.split('\n').filter(|l| l.len() > 0).map(|l| {
        let mut vertices = l.split(',');
        return Cube {
            x: vertices.next().unwrap().parse().unwrap(),
            y: vertices.next().unwrap().parse().unwrap(),
            z: vertices.next().unwrap().parse().unwrap()
        }
    }).collect();
}

fn problem_1(input: &str) -> u32 {
    let cubes = parse_cubes(input);
    let mut grid = HashMap::new();
    for c in cubes.into_iter() {
        grid.insert((c.x,c.y,c.z), c);
    }

    let mut sides = grid.len() as u32 * 6;

    for c in grid.values() {
        if grid.contains_key(&(c.x - 1, c.y, c.z)) {
            sides -= 1;
        }
        if grid.contains_key(&(c.x + 1, c.y, c.z)) {
            sides -= 1;
        }
        if grid.contains_key(&(c.x, c.y - 1, c.z)) {
            sides -= 1;
        }
        if grid.contains_key(&(c.x, c.y + 1, c.z)) {
            sides -= 1;
        }
        if grid.contains_key(&(c.x, c.y, c.z - 1)) {
            sides -= 1;
        }
        if grid.contains_key(&(c.x, c.y, c.z + 1)) {
            sides -= 1;
        }
    }

    sides
}

fn problem_2(input: &str) -> u32 {
    let cubes = parse_cubes(input);
    let mut grid = HashMap::new();
    for c in cubes.iter() {
        grid.insert((c.x,c.y,c.z), TerrainType::Lava);
    }

    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();

    for x in 0..=max_x {
        for y in 0..=max_y {
            for z in 0..=max_z {
                // make everything an air pocket
                if !grid.contains_key(&(x,y,z)) {
                    grid.insert((x,y,z), TerrainType::AirPocket);
                }
            }
        }
    }

    let mut cubes_to_scan = vec![];
    for x in 0..=max_x {
        for y in 0..=max_y {
            let pos = (x,y,0);
            if grid.get(&pos) == Some(&TerrainType::AirPocket) && !cubes_to_scan.contains(&pos) {
                cubes_to_scan.push(pos);
            }
        }
    }
    for x in 0..=max_x {
        for y in 0..=max_y {
            let pos = (x,y,max_z);
            if grid.get(&pos) == Some(&TerrainType::AirPocket) && !cubes_to_scan.contains(&pos) {
                cubes_to_scan.push(pos);
            }
        }
    }
    for z in 0..=max_z {
        for y in 0..=max_y {
            let pos = (0,y,z);
            if grid.get(&pos) == Some(&TerrainType::AirPocket) && !cubes_to_scan.contains(&pos) {
                cubes_to_scan.push(pos);
            }
        }
    }
    for z in 0..=max_z {
        for y in 0..=max_y {
            let pos = (max_x,y,z);
            if grid.get(&pos) == Some(&TerrainType::AirPocket) && !cubes_to_scan.contains(&pos) {
                cubes_to_scan.push(pos);
            }
        }
    }
    for x in 0..=max_x {
        for z in 0..=max_z {
            let pos = (x,0,z);
            if grid.get(&pos) == Some(&TerrainType::AirPocket) && !cubes_to_scan.contains(&pos) {
                cubes_to_scan.push(pos);
            }
        }
    }
    for x in 0..=max_x {
        for z in 0..=max_z {
            let pos = (x,max_y,z);
            if grid.get(&pos) == Some(&TerrainType::AirPocket) && !cubes_to_scan.contains(&pos) {
                cubes_to_scan.push(pos);
            }
        }
    }

    while let Some(cube_to_scan) = cubes_to_scan.pop() {
        // if there is a cube that is air to near me, queue it
        let directions = vec![
            (cube_to_scan.0 - 1, cube_to_scan.1, cube_to_scan.2),
            (cube_to_scan.0 + 1, cube_to_scan.1, cube_to_scan.2),
            (cube_to_scan.0, cube_to_scan.1 - 1, cube_to_scan.2),
            (cube_to_scan.0, cube_to_scan.1 + 1, cube_to_scan.2),
            (cube_to_scan.0, cube_to_scan.1, cube_to_scan.2 - 1),
            (cube_to_scan.0, cube_to_scan.1, cube_to_scan.2 + 1),
        ];

        for d in directions {
            // every cube visited is air, not an air pocket
            if grid.get(&d) == Some(&TerrainType::AirPocket) && !cubes_to_scan.contains(&d) {
                cubes_to_scan.push(d);
            }
        }

        grid.entry(cube_to_scan).and_modify(|g| { *g = TerrainType::Air });
    }

    let mut surface_area = 0;
    for lava_tile in grid.iter().filter(|(_pos, terrain)| **terrain == TerrainType::Lava) {
        let cube_to_scan = lava_tile.0;
        let directions = vec![
            (cube_to_scan.0 - 1, cube_to_scan.1, cube_to_scan.2),
            (cube_to_scan.0 + 1, cube_to_scan.1, cube_to_scan.2),
            (cube_to_scan.0, cube_to_scan.1 - 1, cube_to_scan.2),
            (cube_to_scan.0, cube_to_scan.1 + 1, cube_to_scan.2),
            (cube_to_scan.0, cube_to_scan.1, cube_to_scan.2 - 1),
            (cube_to_scan.0, cube_to_scan.1, cube_to_scan.2 + 1),
        ];

        for d in directions {
            let grid_item = grid.get(&d);
            if grid_item.is_none() || grid_item == Some(&TerrainType::Air) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
        assert_eq!(64, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
        assert_eq!(58, problem_2(&input));
    }
}