
use std::{io::{Stdin, BufRead}, collections::{HashMap, HashSet}, cmp::max};

fn dist(p: (i32, i32), q: (i32, i32)) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn read_input(stdin: &Stdin) -> HashMap<(i32, i32), (i32, i32)> {
    let mut h = HashMap::new();

    for line in stdin.lock().lines() {
        let content = line.expect("IO error");

        let parts = content.split(" ").collect::<Vec<&str>>();
        let sx = parts[2].trim_start_matches("x=").trim_end_matches(",").parse::<i32>().expect("Parse error");
        let sy = parts[3].trim_start_matches("y=").trim_end_matches(":").parse::<i32>().expect("Parse error");
        let bx = parts[8].trim_start_matches("x=").trim_end_matches(",").parse::<i32>().expect("Parse error");
        let by = parts[9].trim_start_matches("y=").trim_end_matches(":").parse::<i32>().expect("Parse error");

        h.insert((sx, sy), (bx, by));
    }
    h
}

pub fn part1(stdin: Stdin) {
    let h = read_input(&stdin);

    let mut beacons: HashSet<(i32, i32)> = HashSet::new();
    let mut all_objects: HashSet<(i32, i32)> = HashSet::new();

    for (S, B) in &h {
        beacons.insert(*B);
        all_objects.insert(*B);
        all_objects.insert(*S);
    }

    let min_x = all_objects.iter().map(|&p| p.0).min().expect("Shouldn't be empty");
    let max_x = all_objects.iter().map(|&p| p.0).max().expect("Shouldn't be empty");

    let y = 2000000;
    let mut count = 0;
    for x in min_x - max_x .. 2*max_x + 1 {
        if beacons.contains(&(x, y)) {
            continue;
        }

        for (S, B) in &h {
            let d = dist(*S, *B);
            if dist((x, y), *S) <= d {
                count += 1;
                break;
            }
        }
    }

    println!("{}", count);
}

fn get_forbidden_interval(S: (i32, i32), B: (i32, i32), y: i32) -> Option<(i32, i32)> {
    // Return the forbidden interval at coordinate y, if B is the beacon for S
    let d = dist(S, B);
    let dy = (S.1 - y).abs();
    let off = d - dy;

    if off < 0 {
        None
    } else {
        Some((S.0 - off, S.0 + off))
    }
}

pub fn part2(stdin: Stdin) {
    let h = read_input(&stdin);

    for y in 0..4000000+1 {
        let mut segs: Vec<(i32, i32)> =
            h.iter()
                .filter_map(|(S, B)| get_forbidden_interval(*S, *B, y))
                .collect();

        segs.sort_unstable_by_key(|&x| x.0);

        // merge overlapping segments until possible; if we merge less than all of them,
        // then the next uncovered x-coordinate is the x of the answer 
        let mut cur_seg_idx: usize = 0;
        let mut cur_end = segs[0].1;
        while cur_seg_idx < segs.len() - 1 && (segs[cur_seg_idx + 1].0 <= cur_end || segs[cur_seg_idx + 1].0 == cur_end + 1) {
            cur_seg_idx += 1;
            cur_end = max(cur_end, segs[cur_seg_idx].1);
        }

        if cur_seg_idx < segs.len() - 1 {
            println!("{}", ((cur_end + 1) as i64) * 4000000 + (y as i64));
            return;
        }
    }
    println!("Not found!");
}
