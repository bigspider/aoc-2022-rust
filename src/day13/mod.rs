
use std::{io::{Stdin}, str::FromStr, cmp::{Ordering, min}, num::ParseIntError};

#[derive(Debug, PartialEq, Clone)]
enum RecListItem {
    Num(u32),
    List(RecList),
}

#[derive(Debug, PartialEq, Clone)]
struct RecList {
    items: Vec<Box<RecListItem>>
}

#[derive(Debug)]
enum RecListParseError {
    ParseIntError,
    ParseRecListError,
}

impl From<ParseIntError> for RecListParseError {
    fn from(_: ParseIntError) -> Self {
        RecListParseError::ParseIntError
    }
}

impl From<u32> for RecList {
    fn from(num: u32) -> Self {
        RecList {
            items: vec![Box::new(RecListItem::Num(num))]
        }
    }
}

fn find_closing_bracket(s: &str, start_pos: usize) -> Option<usize> {
    if s.chars().nth(start_pos) != Some('[') { return None; }

    let mut depth = 1;
    for (pos, c) in s.chars().enumerate().skip(start_pos + 1) {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => (),
        }
        if depth == 0 {
            return Some(pos);
        }
    }

    None
}

impl PartialOrd for RecListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            RecListItem::Num(n_s) => match other {
                RecListItem::Num(n_o) => n_s.partial_cmp(n_o),
                RecListItem::List(list_o) => RecList::from(*n_s).partial_cmp(list_o),
            },
            RecListItem::List(list_s) => match other {
                RecListItem::Num(n_o) => list_s.partial_cmp(&RecList::from(*n_o)),
                RecListItem::List(list_o) => list_s.partial_cmp(list_o),
            },
        }
    }
}

impl PartialOrd for RecList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..min(self.items.len(), other.items.len()) {
            let item_cmp = (&*self.items[i]).partial_cmp(&other.items[i]);
            if item_cmp != Some(Ordering::Equal) {
                return item_cmp;
            }
        }
        // elements up to the common length elements are equal; compare lengths
        self.items.len().partial_cmp(&other.items.len())
    }
}

impl FromStr for RecList {
    type Err = RecListParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(0) != Some('[') || s.chars().nth(s.len() - 1) != Some(']') {
            return Err(RecListParseError::ParseRecListError);
        }
        let mut pos: usize = 1;
        let mut items: Vec<Box<RecListItem>> = Vec::new();
        // This has possibly quadratic complexity, but we won't bother optimizing.
        // We could precompute all the results for find_closing_bracket linear time
        while pos < s.len() - 1 {
            let item = match s.chars().nth(pos).expect("This can't happen") {
                '[' => {
                    let end_pos = match find_closing_bracket(s, pos) {
                        None => return Err(RecListParseError::ParseRecListError),
                        Some(pos) => pos,
                    };
                    let start_pos = pos;
                    pos = end_pos+1;
                    RecListItem::List(s[start_pos..end_pos+1].parse::<RecList>()?)
                },
                '0'..='9' => {
                    let num_end_pos = (pos..s.len()).find(|&i| !s.chars().nth(i).expect("").is_digit(10)).expect("This can't happen");
                    let num_start_pos = pos;
                    pos = num_end_pos;
                    RecListItem::Num(s[num_start_pos..num_end_pos].parse::<u32>()?)
                },
                _ => return Err(RecListParseError::ParseRecListError)
            };

            items.push(Box::new(item));

            // Expecting a comma if not yet done
            if pos < s.len() - 1 && s.chars().nth(pos) != Some(',') {
                return Err(RecListParseError::ParseRecListError);
            }

            pos += 1;
        }
        Ok(RecList { items })
    }
}

fn read_list_pair(stdin: &Stdin) -> Option<(RecList, RecList)> {
    let mut line = String::new();

    stdin.read_line(&mut line).ok()?;
    let left: RecList = match line.trim().parse() {
        Ok(r) => r,
        Err(_) => return None,
    };

    // Starting items
    line.clear();
    stdin.read_line(&mut line).ok()?;
    let right: RecList = match line.trim().parse() {
        Ok(r) => r,
        Err(_) => return None,
    };

    // skip the next line
    line.clear();
    stdin.read_line(&mut line).ok()?;

    Some((left, right))
}

pub fn part1(stdin: Stdin) {
    let mut count = 1;
    let mut result = 0;

    loop {
        let pair = read_list_pair(&stdin);
        match pair {
            Some((first, second)) => {
                if first < second {
                    result += count;
                }
            },
            None => break
        }
        count += 1;
    }
    println!("{}", result);
}

pub fn part2(stdin: Stdin) {
    let mut packets = Vec::new();

    loop {
        match read_list_pair(&stdin) {
            Some((first, second)) => {
                packets.push(first);
                packets.push(second);
            },
            None => break
        }
    }

    let divider_1 = "[[2]]".parse::<RecList>().expect("");
    let divider_2 = "[[6]]".parse::<RecList>().expect("");

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let pos1 = packets.iter().position(|r| *r == divider_1).expect("Not found");
    let pos2 = packets.iter().position(|r| *r == divider_2).expect("Not found");

    println!("{}", (1 + pos1) * (1 + pos2));
}
