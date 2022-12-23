
use std::{io::{Stdin, BufRead}, cmp::max, collections::HashMap};

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore_cost: u16,
    clay_robot_ore_cost: u16,
    obs_robot_ore_cost: u16,
    obs_robot_clay_cost: u16,
    geo_robot_ore_cost: u16,
    geo_robot_obs_cost: u16,
}

fn read_input(stdin: &Stdin) -> Vec<Blueprint> {
    let mut res: Vec<Blueprint> = Vec::new();
    for line in stdin.lock().lines() {
        let content = line.expect("IO Error");
        let parts: Vec<&str> = content.split(' ').collect();

        res.push(Blueprint {
            ore_robot_ore_cost: parts[6].parse().unwrap(),
            clay_robot_ore_cost: parts[12].parse().unwrap(),
            obs_robot_ore_cost: parts[18].parse().unwrap(),
            obs_robot_clay_cost: parts[21].parse().unwrap(),
            geo_robot_ore_cost: parts[27].parse().unwrap(),
            geo_robot_obs_cost: parts[30].parse().unwrap(),
        })
    }
    res
}

fn max_geodes(blueprint: &Blueprint, available_time: u16) -> u16 {
    // TODO: might simplify with a Struct to hold the dynamic programming state

    fn max_geodes_helper(
        blueprint: &Blueprint,
        remaining_time: u16,
        ore_robots: u16,
        clay_robots: u16,
        obsidian_robots: u16,
        geode_robots: u16,
        ore: u16,
        clay: u16,
        obsidian: u16,
        geodes: u16,
        memo: &mut HashMap<[u16; 9], u16>,
        best_found: &mut u16,
    ) -> u16 {

        let mut best = 0;

        if remaining_time == 0 {
            return geodes;
        }

        // upper bound on how many more geodes this solution could build even if we built a new geode robot every minute
        let max_geodes = geodes + geode_robots * remaining_time + remaining_time * (remaining_time - 1) / 2;
        if max_geodes <= *best_found {
            return 0; // can't improve the best found, give up
        }

        if let Some(&result) = memo.get(&[remaining_time, ore_robots, clay_robots, obsidian_robots, geode_robots, ore, clay, obsidian, geodes]) {
            return result;
        }

        // no move
        best = max(best, max_geodes_helper(blueprint,
            remaining_time - 1,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            ore + ore_robots,
            clay + clay_robots,
            obsidian + obsidian_robots,
            geodes + geode_robots,
            memo,
            best_found));
        

        // try building an ore robot if possible
        if remaining_time >= 5 && blueprint.ore_robot_ore_cost <= ore {
            best = max(best, max_geodes_helper(blueprint,
                remaining_time - 1,
                ore_robots + 1,
                clay_robots,
                obsidian_robots,
                geode_robots,
                ore + ore_robots - blueprint.ore_robot_ore_cost,
                clay + clay_robots,
                obsidian + obsidian_robots,
                geodes + geode_robots,
                memo,
                best_found));
        }

        // try building a clay robot if possible
        if remaining_time >= 4 && blueprint.clay_robot_ore_cost <= ore {
            best = max(best, max_geodes_helper(blueprint,
                remaining_time - 1,
                ore_robots,
                clay_robots + 1,
                obsidian_robots,
                geode_robots,
                ore + ore_robots - blueprint.clay_robot_ore_cost,
                clay + clay_robots,
                obsidian + obsidian_robots,
                geodes + geode_robots,
                memo,
                best_found));
        }

        // try building an obsidian robot if possible
        if remaining_time >= 3 && blueprint.obs_robot_ore_cost <= ore && blueprint.obs_robot_clay_cost <= clay {
            best = max(best, max_geodes_helper(blueprint,
                remaining_time - 1,
                ore_robots,
                clay_robots,
                obsidian_robots + 1,
                geode_robots,
                ore + ore_robots - blueprint.obs_robot_ore_cost,
                clay + clay_robots - blueprint.obs_robot_clay_cost,
                obsidian + obsidian_robots,
                geodes + geode_robots,
                memo,
                best_found));
        }

        // try building a geode-cracking robot if possible
        if remaining_time >= 2 && blueprint.geo_robot_ore_cost <= ore && blueprint.geo_robot_obs_cost <= obsidian {
            best = max(best, max_geodes_helper(blueprint,
                remaining_time - 1,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots + 1,
                ore + ore_robots - blueprint.geo_robot_ore_cost,
                clay + clay_robots,
                obsidian + obsidian_robots - blueprint.geo_robot_obs_cost,
                geodes + geode_robots,
                memo,
                best_found));
        }

        if remaining_time >= 7 { // not enough RAM to memoize everything, let's put a cap
            memo.insert([remaining_time, ore_robots, clay_robots, obsidian_robots, geode_robots, ore, clay, obsidian, geodes], best);
        }

        if best >= *best_found {
            *best_found = best;
        }

        best
    }

    let mut memo: HashMap<[u16; 9], u16> = HashMap::new();
    let mut best_found: u16 = 0;

    max_geodes_helper(blueprint,
        available_time, 1, 0, 0, 0, 0, 0, 0, 0, &mut memo, &mut best_found)
}

pub fn part1(stdin: Stdin) {
    let blueprints = read_input(&stdin);

    let result: u16 = blueprints
        .iter().enumerate()
        .map(|(i, bp)| max_geodes(bp, 24) * (i + 1) as u16)
        .sum();

    println!("{}", result);
}

pub fn part2(stdin: Stdin) {
    let blueprints = read_input(&stdin);

    let mut result: u32 = 1;
    for bp in blueprints.iter().take(3) {
        result = result * max_geodes(bp, 32) as u32;
    }

    println!("{}", result);
}
