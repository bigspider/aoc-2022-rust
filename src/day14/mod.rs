
use std::{io::{Stdin, BufRead}, cmp::{min, max}, collections::HashMap};

type Point = (i32, i32);

// fn segment(p: Point, q: Point) -> impl Iterator<Item = Point> {
//     if p.0 == q.0 {
//         (min(p.1, q.1)..max(p.1, q.1) + 1).map(move |y| (p.0, y)).
//     } else if p.1 == q.1 {
//         (min(p.0, q.0)..max(p.0, q.0) + 1).map(move |x| (x, p.1)).collect()
//     } else {
//         panic!("This function requires axis-aligned segments");
//     }
// }


// // TODO: why does Rust not like the nicer version above?
fn segment(p: Point, q: Point) -> impl Iterator<Item = Point> {
    if p.0 != q.0 && p.1 != q.1 {
        panic!("This function requires axis-aligned segments");
    }

    let (fixed, begin, end) = if p.0 == q.0 {
        (p.0, min(p.1, q.1), max(p.1, q.1) + 1)
    } else {
        (p.1, min(p.0, q.0), max(p.0, q.0) + 1)
    };

    (begin..end).map(move |coord| if p.0 == q.0 { (fixed, coord) } else { (coord, fixed) })
}

fn parse_point(point_str: &str) -> Point {
    let coords: Vec<i32> = point_str.split(',').map(|x| x.parse().expect("Invalid input")).collect();
    assert!(coords.len() == 2);

    (coords[0], coords[1])
}

fn read_input(stdin: &Stdin) -> HashMap<Point, char> {
    let mut h = HashMap::new();

    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        let points: Vec<Point> = content.split(" -> ").map(parse_point).collect();

        for i in 0..points.len() - 1 {
            for p in segment(points[i], points[i + 1]) {
                h.insert(p, '#');
            }
        }
    }
    h
}

pub fn part1(stdin: Stdin) {
    let mut h = read_input(&stdin);

    let max_y = h.keys().map(|&p| p.1).max().expect("Shouldn't be empty");

    let mut snake = vec![(500, 0)];
    loop {
        let head = *snake.last().expect("Shouldn't be empty");
        if head.1 >= max_y {
            break;
        }

        if !h.contains_key(&(head.0, head.1 + 1)) {
            snake.push((head.0, head.1 + 1));
        } else if !h.contains_key(&(head.0 - 1, head.1 + 1)) {
            snake.push((head.0 - 1, head.1 + 1));
        } else if !h.contains_key(&(head.0 + 1, head.1 + 1)) {
            snake.push((head.0 + 1, head.1 + 1));
        } else {
            h.insert(head, 'o');
            snake.pop();
        }
    }

    println!("{}", h.values().filter(|&&c| c == 'o').count());
}

pub fn part2(stdin: Stdin) {
    let mut h = read_input(&stdin);

    let max_y = h.keys().map(|&p| p.1).max().expect("Shouldn't be empty");

    let mut snake = vec![(500, 0)];
    while !snake.is_empty() {
        let head = *snake.last().expect("Shouldn't be empty");

        if head.1 == max_y + 1 {
            h.insert(head, 'o');
            snake.pop();
        } else if !h.contains_key(&(head.0, head.1 + 1)) {
            snake.push((head.0, head.1 + 1));
        } else if !h.contains_key(&(head.0 - 1, head.1 + 1)) {
            snake.push((head.0 - 1, head.1 + 1));
        } else if !h.contains_key(&(head.0 + 1, head.1 + 1)) {
            snake.push((head.0 + 1, head.1 + 1));
        } else {
            h.insert(head, 'o');
            snake.pop();
        }
    }

    println!("{}", h.values().filter(|&&c| c == 'o').count());
}
