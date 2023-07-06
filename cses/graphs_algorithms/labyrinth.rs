#[allow(unused_imports)]
use std::io::{self, stdin, stdout, BufWriter, Write, Stdout};
use std::{collections::VecDeque};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

static DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Clone)]
struct Pair<T1, T2> {
    first: T1,
    second: T2
}

impl<T1: Clone, T2: Clone> Pair<T1, T2> {
    fn get_tuple(&self) -> (T1, T2) {
        (self.first.clone(), self.second.clone())
    }
}

fn bfs(source: Pair<usize, usize>, target: Pair<usize, usize>,
    board: &mut Vec<Vec<bool>>,
    path: &mut Vec<Vec<i32>>,
    out: &mut BufWriter<Stdout>
) {
    let mut q: VecDeque<Pair<Pair<usize,usize>,i32>> = VecDeque::new();
    q.push_back(
        Pair {
            first: source.clone(),
            second: 0
        }
    );
    let (rs, cs) = source.get_tuple();
    path[rs][cs] = 0;
    let mut solved = false;
    while !q.is_empty() {
        let (p, d) = q.front().unwrap().get_tuple();
        q.pop_front();
        let (r, c) = p.get_tuple();
        
        if solved {break;}
        
        for i in 0..4 {
            let (dr, dc) = DIRS[i];
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nc < 0 || nc >= board[0].len() as i32 || nr < 0 || nr >= board.len() as i32 {continue;}
            let nc: usize = nc as usize; let nr: usize = nr as usize;
            if board[nr][nc] {
                board[nr][nc] = false;
                path[nr][nc] = d + 1;
                q.push_back(
                    Pair {
                        first: Pair { first: nr, second: nc },
                        second: d + 1
                    }
                );
            }
            let (rt, ct) = target.get_tuple();
            if nr == rt && nc == ct {
                solved = true;
                break;
            }
        }
    }

    let (mut r, mut c) = target.get_tuple();
    if solved {
        writeln!(out, "YES\n{}", path[r][c]).ok();
        build_path(&mut r, &mut c, &path, out);
    } else {
        writeln!(out, "NO").ok();
    }
}

fn build_path(r: &mut usize, c: &mut usize, map: &Vec<Vec<i32>>, out: &mut BufWriter<Stdout>) {
    let mut s = map[*r][*c];
    let mut path: Vec<char> = vec![' '; s as usize];

    while s != 0 {
        for i in 0..4 {
            let (dr, dc) = DIRS[i];
            let nr = *r as i32 + dr;
            let nc = *c as i32 + dc;
            if nc < 0 || nc >= map[0].len() as i32 || nr < 0 || nr >= map.len() as i32 {continue;}
            let nc: usize = nc as usize; let nr: usize = nr as usize;
            if map[nr][nc] == s - 1 {
                path[s as usize - 1] = match i {
                    0 => 'L',
                    1 => 'R',
                    2 => 'U',
                    3 => 'D',
                    _ => ' '
                };
                s -= 1;
                *r = nr as usize;
                *c = nc as usize;
                break;
            }
        }
    }

    for c in path {
        write!(out, "{}", c).ok();
    }
}

pub fn main() {
    let mut scan = Scanner::default();
    let mut out = BufWriter::new(stdout());

    let n: usize = scan.next();
    let m: usize = scan.next();

    let mut source: Pair<usize, usize> = Pair { first: 0, second: 0 };
    let mut target: Pair<usize, usize> = Pair { first: 0, second: 0 };

    let board: Vec<String> = (0..n).map(|_| scan.next::<String>()).collect();
    let mut r: usize = 0;
    let mut board = board.iter().map(
        |row| {row.chars().map(
            |c| {
                if c == 'A' {
                    source = Pair{ first: (r / m), second: row.find(c).unwrap()};
                } else if c == 'B' {
                    target = Pair{ first: (r / m), second: row.find(c).unwrap()};
                    return true;
                }
                r += 1;
                return c == '.'
            })}
            .collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let mut path: Vec<Vec<i32>> = vec![vec![-1; m]; n];

    bfs(source, target, &mut board, &mut path, &mut out);
}