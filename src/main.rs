use console::style;
use std::time::Instant;
#[derive(Default, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    val: i8,
    saldo: bool,
    hist: Vec<Point>,
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        }
    }
}

fn get_neigh<'a>(maze: &'a Vec<Vec<Point>>, point: &Point) -> Vec<&'a Point> {
    let mut neighbors: Vec<&'a Point> = vec![];
    if point.x != 0 {
        neighbors.push(&maze[point.y as usize][point.x as usize - 1]);
    }
    if point.x != maze[0].len() as i32 - 1 {
        neighbors.push(&maze[point.y as usize][point.x as usize + 1]);
    }
    if point.y != 0 {
        neighbors.push(&maze[point.y as usize - 1][point.x as usize]);
    }
    if point.y != maze.len() as i32 - 1 {
        neighbors.push(&maze[point.y as usize + 1][point.x as usize]);
    }
    neighbors
}

fn build_maze(maze: &Vec<Vec<i8>>) -> Vec<Vec<Point>> {
    let track_maze: &mut Vec<Vec<Point>> = &mut vec![vec![]; maze.len()];
    for (y, vector) in maze.iter().enumerate() {
        for (x, point) in vector.iter().enumerate() {
            track_maze[y].push(Point {
                x: x as i32,
                y: y as i32,
                val: *point,
                ..Point::default()
            });
        }
    }
    track_maze.clone()
}

fn print_maze(maze: &Vec<Vec<Point>>, hist: &Vec<Point>) {
    for vector in maze {
        for point in vector {
            if hist.contains(point) {
                print!("{:2}", style(point.val).green());
            } else {
                print!("{:2}", point.val);
            }
        }
        print!("\n");
    }
}

pub fn answer(maze: Vec<Vec<i8>>) -> Option<i32> {
    let start = Instant::now();
    let track_maze = build_maze(&maze);
    let first: &Point = &track_maze[0][0];
    let mut queue: Vec<Point> = vec![first.clone()];
    let end = (maze[0].len() as i32 - 1, maze.len() as i32 - 1);

    while !queue.is_empty() {
        let active = queue.pop().unwrap();
        if (active.x, active.y) == end {
            let duration = start.elapsed();
            let mut final_hist = active.hist.clone();
            final_hist.push(Point {
                x: active.x,
                y: active.y,
                ..Point::default()
            });
            print_maze(&track_maze, &final_hist);
            println!("Time was: {:?}", duration);
            return Some(final_hist.len() as i32 - 1);
        } else {
            for neighbor in get_neigh(&track_maze, &active) {
                if !active.hist.contains(&neighbor) {
                    let mut temp = neighbor.clone();
                    temp.hist = active.hist.clone();
                    temp.hist.push(Point {
                        x: active.x,
                        y: active.y,
                        ..Point::default()
                    });
                    if neighbor.val == 0 {
                        temp.saldo = active.saldo;
                        queue.insert(0, temp);
                    } else if active.saldo == false {
                        temp.hist.push(active.clone());
                        temp.saldo = true;
                        queue.insert(0, temp);
                    }
                }
            }
        }
    }
    None
}

fn main() {
    assert_eq!(
        answer(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
        ])
        .unwrap(),
        21
    );
}

#[test]
fn test1() {
    assert_eq!(
        answer(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
        ])
        .unwrap(),
        21
    );
}
#[test]
fn test3() {
    assert_eq!(
        answer(vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 0],
        ])
        .unwrap(),
        8
    );
}

#[test]
fn test5() {
    assert_eq!(
        answer(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
        ])
        .unwrap(),
        11
    );
}

#[test]
fn test6() {
    assert_eq!(
        answer(vec![
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ])
        .unwrap(),
        39
    );
}
