
use std::{io::{Stdin, BufRead}, collections::{HashMap}, cmp::{max}};

trait BitVector {
    fn get_bit(&self, i: usize) -> bool;
    fn set_bit(&self, i: usize, value: bool) -> Self;
}

impl BitVector for u64 {
    fn get_bit(&self, i: usize) -> bool {
        if i >= 64 { panic!("i must be at most 63") }

        (self >> i) & 1 == 1
    }

    fn set_bit(&self, i: usize, value: bool) -> Self {
        if i >= 64 { panic!("i must be at most 63") }

        if value { self | (1 << i) }
        else { self & !(1 << i)} 
    }
}

fn read_input(stdin: &Stdin) -> (Vec<Vec<usize>>, Vec<i32>, usize) {
    let mut names_to_index: HashMap<String, usize> = HashMap::new();

    let mut rates: Vec<i32> = Vec::new();
    let mut successors: Vec<Vec<usize>> = Vec::new();

    for line in stdin.lock().lines() {
        let content = line.expect("IO error");
        
        let parts = content.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let name = &parts[1];
        let rate = parts[4].trim_start_matches("rate=").trim_end_matches(";").parse::<i32>().expect("Parse error");

        let src = match names_to_index.get(name) {
            Some(&idx) => {
                rates[idx] = rate;
                idx
            },
            None => {
                let new_idx = rates.len();
                names_to_index.insert(name.clone(), new_idx);
                rates.push(rate);
                successors.push(Vec::new());

                new_idx
            },
        };


        let successor_names: Vec<String> = parts.iter().skip(9).map(|x| String::from(x.trim_end_matches(","))).collect();

        for succ in successor_names {
            let dst = match names_to_index.get(&succ) {
                Some(&succ_idx) => succ_idx,
                None => {
                    let new_succ_idx = rates.len();
                    names_to_index.insert(succ, new_succ_idx);
                    successors.push(Vec::new());
                    rates.push(-1);

                    new_succ_idx
                },
            };

            successors[src].push(dst);
        }
    }

    (successors, rates, *names_to_index.get("AA").expect("AA not found"))
}

fn find_best_strat(n: usize, successors: &Vec<Vec<usize>>, rates: &Vec<i32>, start_pos: usize) -> i32 {
    // dynamic programming over:
    // - your position (index of the valve)
    // - the set of open valves (only opening if non-zero rate)
    // - the remaining time left
    // O(n 2^v T) subproblems.
    fn find_best_strat_helper(n: usize,
        successors: &Vec<Vec<usize>>,
        rates: &Vec<i32>,
        cur_pos: usize,
        used: u64,
        time_left: i32,
        memo: &mut HashMap<(u64, usize, i32), i32>) -> i32
    {
        if time_left <= 1 {
            return 0;
        }

        if let Some(&res) = memo.get(&(used, cur_pos, time_left)) {
            return res;
        }

        let mut best = 0;

        // try opening the current valve, if possible
        if !used.get_bit(cur_pos) && rates[cur_pos] > 0 {
            let partial = find_best_strat_helper(n, successors, rates, cur_pos, used.set_bit(cur_pos, true), time_left - 1, memo);
            best = max(best, partial + (time_left - 1) * rates[cur_pos]);
        }

        for &succ in successors[cur_pos].iter() {
            let partial = find_best_strat_helper(n, successors, rates, succ, used, time_left - 1, memo);
            best = max(best, partial)
        }

        memo.insert((used, cur_pos, time_left), best);

        best
    }

    let mut memo: HashMap<(u64, usize, i32), i32> = HashMap::new();

    find_best_strat_helper(n, successors, rates, start_pos, 0, 30, &mut memo)
}

pub fn part1(stdin: Stdin) {
    let (successors, rates, start_pos) = read_input(&stdin);
    let n = rates.len();

    println!("{}", find_best_strat(n, &successors, &rates, start_pos));
}


fn find_best_strat_2(n: usize, successors: &Vec<Vec<usize>>, rates: &Vec<i32>, start_pos: usize) -> i32 {
    // dynamic programming over:
    // - your position, and that of the elephant
    // - the set of open valves (only opening if non-zero rate)
    // - the remaining time left
    // O(n^2 2^v T) subproblems.
    fn find_best_strat_helper(n: usize,
        successors: &Vec<Vec<usize>>,
        rates: &Vec<i32>,
        cur_pos_me: usize,
        cur_pos_elephant: usize,
        used: u64,
        time_left: i32,
        memo: &mut HashMap<(u64, usize, usize, i32), i32>) -> i32
    {
        if time_left <= 1 {
            return 0;
        }

        if let Some(&res) = memo.get(&(used, cur_pos_me, cur_pos_elephant, time_left)) {
            return res;
        }

        let mut best = 0;

        for me_open_valve in [false, true] {
            if me_open_valve && (used.get_bit(cur_pos_me) || rates[cur_pos_me] == 0) { continue; }
            for el_open_valve in [false, true] {                
                if el_open_valve && (used.get_bit(cur_pos_elephant) || rates[cur_pos_elephant] == 0) { continue; }

                if cur_pos_me == cur_pos_elephant && me_open_valve && el_open_valve {
                    continue; // only you or the elephant can open the current valve, not both!
                }

                let mut new_used = used;

                let me_next_coords = if me_open_valve {
                    vec![cur_pos_me] // if opening the valve, not movig
                } else {
                    successors[cur_pos_me].clone() // otherwise, go to a neighbour; never a reason to stay still
                };
                if me_open_valve {
                    new_used = new_used.set_bit(cur_pos_me, true)
                }

                // same reasoning above for the elephant
                let el_next_coords = if el_open_valve {
                    vec![cur_pos_elephant]
                } else {
                    successors[cur_pos_elephant].clone()
                };
                if el_open_valve {
                    new_used = new_used.set_bit(cur_pos_elephant, true)
                }

                // if opening one or two valves, compute the pressure released (after 1 time unit)
                let new_valve_me_score = if me_open_valve { rates[cur_pos_me] * (time_left - 1) } else { 0 };
                let new_valve_el_score = if el_open_valve { rates[cur_pos_elephant] * (time_left - 1) } else { 0 };

                for &next_pos_me in &me_next_coords {
                    for &next_pos_el in &el_next_coords {
                        let partial = find_best_strat_helper(n, successors, rates, next_pos_me, next_pos_el, new_used, time_left - 1, memo);
                        best = max(best, partial + new_valve_me_score + new_valve_el_score);
                    }
                }
            }
        }

        memo.insert((used, cur_pos_me, cur_pos_elephant, time_left), best);

        best
    }

    let mut memo: HashMap<(u64, usize, usize, i32), i32> = HashMap::new();

    find_best_strat_helper(n, successors, rates, start_pos, start_pos, 0, 26, &mut memo)
}


pub fn part2(stdin: Stdin) {
    let (successors, rates, start_pos) = read_input(&stdin);
    let n = rates.len();

    println!("{}", find_best_strat_2(n, &successors, &rates, start_pos));
}
