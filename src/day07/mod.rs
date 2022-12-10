
use std::{io::{Stdin, BufRead}, collections::HashMap};

struct Directory<'a> {
    subdirs: HashMap<&'a str, Box<Directory<'a>>>,
    files: HashMap<&'a str, u64>,
}

impl<'a> Directory<'a> {
    fn new() -> Directory<'a> { 
        Directory {
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

fn process<'a, 'b:'a>(dir: &'a mut Directory<'b>, lines: &'b Vec<String>, cur_pos: usize) -> usize {
    let mut new_pos = cur_pos; 
    while new_pos < lines.len() {
        if lines[new_pos].starts_with("$ ls") {
            new_pos += 1;
            while new_pos < lines.len() && !lines[new_pos].starts_with("$") {
                let parts: Vec<&str> = lines[new_pos].split(' ').collect();
                assert!(parts.len() == 2);

                if parts[0] == "dir" {
                    let dir_name = parts[1];
                    if !dir.subdirs.contains_key(dir_name) {
                        let new_sub: Directory = Directory::new();
                        dir.subdirs.insert(dir_name, Box::new(new_sub));
                    }
                } else {
                    // parse it as a file
                    let file_size = parts[0].parse::<u64>().expect("Invalid file size");
                    dir.files.insert(parts[1], file_size);
                }
                new_pos += 1;
            }
        } else if lines[new_pos].starts_with("$ cd ..") {
            new_pos += 1;
            return new_pos - cur_pos;
        } else if  lines[new_pos].starts_with("$ cd") {
            let parts: Vec<&str> = lines[new_pos].split(' ').collect();

            assert!(parts.len() == 3);

            let subdir = dir.subdirs.get_mut(parts[2]).expect("I don't know this subdir");

            new_pos += 1;
            new_pos += process(subdir, lines, new_pos);
        } else {
            panic!("Unexpected command");
        }
    } 

    new_pos - cur_pos
}

fn collect_sizes(result: &mut Vec<u64>, dir: &Directory) -> u64 {
    let files_size: u64 = dir.files.values().sum();
    let dirs_size: u64 = dir.subdirs.values().map(|d| collect_sizes(result, d)).sum();
    let total_size = files_size + dirs_size;

    result.push(total_size);

    total_size
}

pub fn part1(stdin: Stdin) {
    let input: Vec<String> = stdin.lock().lines()
        .map(|l| l.expect("Failed to read input"))
        .collect();

    let mut root = Box::new(Directory::new());

    process(&mut root, &input, 1); // skip the initial "cd /"

    let mut sizes: Vec<u64> = Vec::new();
    collect_sizes(&mut sizes, &root);
    let result: u64 = sizes.iter().filter(|&&x| x <= 100000).sum();

    println!("{}", result);
}


pub fn part2(stdin: Stdin) {
    let input: Vec<String> = stdin.lock().lines()
        .map(|l| l.expect("Failed to read input"))
        .collect();

    let mut root = Box::new(Directory::new());

    process(&mut root, &input, 1); // skip the initial "cd /"

    let mut sizes: Vec<u64> = Vec::new();
    let total_size = collect_sizes(&mut sizes, &root);

    // find the smallest dir with size at least total_size - 30000000
    let space_to_free = 30000000 - (70000000 - total_size);
    let result = sizes.iter()
                           .map(|x| *x)
                           .filter(|&x| x >= space_to_free)
                           .min()
                           .expect("Not found");

    println!("{}", result);
}
