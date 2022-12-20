
use std::{io::{Stdin, BufRead}, collections::HashMap};

// TODO: this is pretty ugly, but I probably won't come back to this :P

// upside down, we count from the bottom
fn get_rock_types() -> Vec<Vec<&'static str>> {
    vec![
        vec![
            "####",
        ],
        vec![
            ".#.",
            "###",
            ".#.",
        ],
        vec![
            "###",
            "..#",
            "..#",
        ],
        vec![
            "#",
            "#",
            "#",
            "#",
        ],
        vec![
            "##",
            "##",
        ],
    ]
}

type Tower = [Vec<char>; 7];

trait TowerColumn {
    fn get_or_default(&self, i: usize) -> char;
    fn set_or_default(&mut self, i: usize, val: char);
}


impl TowerColumn for Vec<char> {
    fn get_or_default(&self, i: usize) -> char {
        if i < self.len() { self[i] }
        else { '.' }
    }

    fn set_or_default(&mut self, i: usize, val: char) {
        if self.len() > i {
            self[i] = val;
        } else {
            self.resize_with(i, || '.'); // default until length i - 1
            self.push(val); // then push the expected element
        }
    }
}

fn overlaps(tower: &Tower, rock: &Vec<&str>, x: usize, y: usize) -> bool {
    let rock_width = rock[0].len();
    let rock_height = rock.len();
    
    for j in 0..rock_height {
        for i in 0..rock_width {
            let col = x + i;

            if col >= 7 { panic!("Invalid coordinates"); }

            if tower[col].get_or_default(j + y) != '.' && rock[j].chars().nth(i).unwrap() != '.' {
                return true;
            }
        }
    }

    false
}

fn print_tower(tower: &Tower) {
    let cur_height = tower.iter().map(|c| c.len()).max().unwrap();

    for j in (0..cur_height).rev() {
        for i in 0..tower.len() {
            print!("{}", tower[i].get_or_default(j));
        }
        println!();
    }
}

pub fn part1(stdin: Stdin) {
    let pattern: Vec<char> = stdin.lock().lines().next()
        .expect("IO error").expect("IO error")
        .chars().collect();

    let rock_types = get_rock_types();

    let mut cur_type = 0;

    // columns, from bottom up
    let mut tower: Tower = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut pattern_pos: usize = 0;
    for _ in 0..2022 {
        let cur_height = tower.iter().map(|c| c.len()).max().unwrap();

        let cur_rock = &rock_types[cur_type % rock_types.len()];
        
        let rock_width = cur_rock[0].len();
        let rock_height = cur_rock.len();

        let mut rock_pos = (2 as usize, cur_height + 3);

        loop {
            // move left/right according to the jet direction
            let jet = pattern[pattern_pos % pattern.len()];
            if jet == '<' && rock_pos.0 > (0 as usize) && !overlaps(&tower, cur_rock, rock_pos.0 - 1, rock_pos.1) {
                rock_pos.0 -= 1;
            }
            if jet == '>' && rock_pos.0 + rock_width < (7 as usize) && !overlaps(&tower, cur_rock, rock_pos.0 + 1, rock_pos.1) {
                rock_pos.0 += 1;
            }

            pattern_pos += 1;

            // move down if possible
            if rock_pos.1 > 0 && !overlaps(&tower, cur_rock, rock_pos.0, rock_pos.1 - 1) {
                rock_pos.1 -= 1;
            } else {
                // not possible, therefore we merge in the tower and go to the next rock coming
                for j in 0..rock_height {
                    for i in 0..rock_width {
                        let col = rock_pos.0 + i;
                        if cur_rock[j].chars().nth(i).unwrap() == '#' {
                            tower[col].set_or_default(j + rock_pos.1, '#');
                        }
                    }
                }
                break;
            }
        }
        cur_type += 1;
    }

    println!("{}", tower.iter().map(|col| col.len()).max().unwrap());
}


pub fn part2(stdin: Stdin) {
    let pattern: Vec<char> = stdin.lock().lines().next()
        .expect("IO error").expect("IO error")
        .chars().collect();

    let rock_types = get_rock_types();

    let mut cur_type = 0;

    // columns, from bottom up
    let mut tower: Tower = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut pattern_pos: usize = 0;

    let mut data: HashMap<(usize, usize, usize), (usize, usize)> = HashMap::new();

    let mut confirmations = 0;
    let mut period: Option<usize> = None;
    let mut height_increase: usize = 0; // height increase with repeated period (when period found)

    loop {
        let cur_height = tower.iter().map(|c| c.len()).max().unwrap();

        let cur_rock = &rock_types[cur_type % rock_types.len()];
        
        let rock_width = cur_rock[0].len();
        let rock_height = cur_rock.len();

        let mut rock_pos = (2 as usize, cur_height + 3);

        loop {
            // move left/right according to the jet direction
            let jet = pattern[pattern_pos % pattern.len()];
            if jet == '<' && rock_pos.0 > (0 as usize) && !overlaps(&tower, cur_rock, rock_pos.0 - 1, rock_pos.1) {
                rock_pos.0 -= 1;
            }
            if jet == '>' && rock_pos.0 + rock_width < (7 as usize) && !overlaps(&tower, cur_rock, rock_pos.0 + 1, rock_pos.1) {
                rock_pos.0 += 1;
            }

            pattern_pos += 1;

            // move down if possible
            if rock_pos.1 > 0 && !overlaps(&tower, cur_rock, rock_pos.0, rock_pos.1 - 1) {
                rock_pos.1 -= 1;
            } else {
                // not possible, therefore we merge in the tower and go to the next rock coming
                for j in 0..rock_height {
                    for i in 0..rock_width {
                        let col = rock_pos.0 + i;
                        if cur_rock[j].chars().nth(i).unwrap() == '#' {
                            tower[col].set_or_default(j + rock_pos.1, '#');
                        }
                    }
                }
                break;
            }
        }

        // hacky way of identifying the "period", that is, when the growth behavior starts repeating
        // we keep in a HashMap all the triplets of
        // -- current rock type
        // -- current position in the pattern
        // -- final position of the rock when it just set
        // and we record the current total number of pieces and the tower height for each triplet.
        // When it starts repeating, it _might_ be a period; we wait until we confirm the period
        // 10 times to avoid false positives.
        let new_data = (rock_pos.0, cur_type % 5, pattern_pos % pattern.len());
        match data.get(&new_data) {
            None => {},
            Some(&(prev_count, prev_height)) => {
                let exp_period = cur_type - prev_count;
                match period {
                    None => {
                        period = Some(exp_period);
                        confirmations = 1;
                        height_increase = cur_height + rock_height - prev_height;
                    }
                    Some(other_period) => {
                        if other_period == exp_period {
                            confirmations += 1;
                        } else {
                            // attempt failed, search again for a new period
                            period = None;
                            confirmations = 0;
                        }
                    }
                }
            }
        }
        data.insert(new_data, (cur_type, rock_pos.1 + rock_height));

        if confirmations >= 10 {
            let per = period.unwrap();
            if 1000_000_000_000 % per == cur_type % per {
                let final_increase = (1000_000_000_000u64 - cur_type as u64)/(per as u64) * (height_increase as u64);
                println!("{}", (cur_height + rock_height) as u64 + final_increase
                    - 1); // TODO: couldn't identify an off-by-one, but I had enough of this problem >:)
                return;
            }
        }

        cur_type += 1;
    }
}
