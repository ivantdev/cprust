#[allow(unused_imports)]
use std::io::{self, stdin, stdout, BufWriter, Write};
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

struct Pair {
    first: usize,
    second: usize
}

impl Pair {
    fn get_tuple(&self) -> (usize, usize) {
        (self.first, self.second)
    }
}

fn bfs(
    board: &mut Vec<Vec<bool>>,
    pair: Pair
) {
    let mut q: VecDeque<Pair> = VecDeque::new();
    q.push_back(pair);
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap().get_tuple();
        if !board[y][x] {continue;}
        board[y][x] = false;
        for k in 0..4 {
            let (dx, dy) = DIRS[k];
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= board[0].len() as i32 || ny < 0 || ny >= board.len() as i32 {continue;}
            let nx: usize = nx as usize; let ny: usize = ny as usize;
            if board[ny][nx] {
                q.push_back(Pair { first: nx, second: ny });
            }
        }
    }
}

pub fn main() {
    let mut scan = Scanner::default();
    let mut out = BufWriter::new(stdout());

    let n: usize = scan.next();
    let m: usize = scan.next();
    let board: Vec<String> = (0..n).map(|_| scan.next::<String>()).collect();
    let mut board = board.iter().map(
        |row| row.chars().map(
            |c| c == '.')
            .collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let mut counter = 0;

    for i in 0..n {
        for j in 0..m {
            if board[i][j] {
                bfs(&mut board, Pair { first: j, second: i });
                counter += 1;
            }
        }
    }

    write!(out, "{}", counter).ok();
}