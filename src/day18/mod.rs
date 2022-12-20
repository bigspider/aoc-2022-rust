
use std::{io::{Stdin, BufRead}, collections::{HashSet}, cmp::{min, max}};

fn read_input(stdin: &Stdin) -> Vec<(i32, i32, i32)> {
    let mut res: Vec<(i32, i32, i32)> = Vec::new();
    for line in stdin.lock().lines() {
        let cube: Vec<i32> = line.expect("IO Error").split(",").map(|x| x.parse::<i32>().expect("Parsing error")).collect();
        assert!(cube.len() == 3);
        res.push((cube[0], cube[1], cube[2]));
    }
    res
}

fn neighbours(cube: (i32, i32, i32), mins: (i32, i32, i32), maxs: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut res = Vec::new();
    if cube.0-1 >= mins.0 { res.push((cube.0-1, cube.1, cube.2)) }
    if cube.0+1 <= maxs.0 { res.push((cube.0+1, cube.1, cube.2)) }
    if cube.1-1 >= mins.1 { res.push((cube.0, cube.1-1, cube.2)) }
    if cube.1+1 <= maxs.1 { res.push((cube.0, cube.1+1, cube.2)) }
    if cube.2-1 >= mins.2 { res.push((cube.0, cube.1, cube.2-1)) }
    if cube.2+1 <= maxs.2 { res.push((cube.0, cube.1, cube.2+1)) }
    res
}

pub fn part1(stdin: Stdin) {
    let cubes = read_input(&stdin);

    let mut m: HashSet<(i32, i32, i32)> = HashSet::new();
    for &cube in &cubes {
        m.insert(cube);
    }

    let mut count = 0;
    for &cube in &cubes {
        let neigh = neighbours(cube, (i32::MIN, i32::MIN, i32::MIN), (i32::MAX, i32::MAX, i32::MAX));
        count += neigh.iter().filter(|&c| !m.contains(c)).count();
    }

    println!("{}", count);
}

pub fn part2(stdin: Stdin) {
    let cubes = read_input(&stdin);

    let mut m: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;
    for &cube in &cubes {
        m.insert(cube);
        min_x = min(min_x, cube.0);
        max_x = max(max_x, cube.0);
        min_y = min(min_y, cube.1);
        max_y = max(max_y, cube.1);
        min_z = min(min_z, cube.2);
        max_z = max(max_z, cube.2);
    }

    let mins = (min_x - 1, min_y - 1, min_z - 1);
    let maxs = (max_x + 1, max_y + 1, max_z + 1);

    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();

    fn visit(m: &HashSet<(i32, i32, i32)>, visited: &mut HashSet<(i32, i32, i32)>, cur: (i32, i32, i32), mins: (i32, i32, i32), maxs: (i32, i32, i32)) {
        visited.insert(cur);
        for n in neighbours(cur, mins, maxs) {
            if !m.contains(&n) && !visited.contains(&n) {
                visit(m, visited, n, mins, maxs);
            }
        }
    }

    visit(&m, &mut visited, mins, mins, maxs); // dfs from arbitrary water block

    let mut count = 0;
    for &cube in &cubes {
        count += neighbours(cube, mins, maxs).iter().filter(|&c| visited.contains(c)).count();
    }

    println!("{}", count);
}
