#[allow(unused_imports)]
use std::io::{self, stdin, stdout, BufWriter, Write, Stdout};
// use std::{collections::VecDeque};

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

const MOD: i64 = 1e9 as i64 + 7;

fn f(k: i32, dp: &mut Vec<i64>) -> i64 {
    if k == 1 || k == 0 {return 1;}
    if k < 0 {return 0;}
    if dp[(k as usize)] == -1 {
        dp[(k as usize)] = 0;
        for i in 1..7 {
            dp[(k as usize)] += f(k-i, dp);
        }
        dp[(k as usize)] %= MOD;
    }
    return dp[(k as usize)];
}

pub fn main() {
    let mut scan = Scanner::default();
    let mut out: BufWriter<Stdout> = BufWriter::new(stdout());

    let n: usize = scan.next();

    // Top-Down solution
    let mut dp: Vec<i64> = vec![-1; n + 1];
    let res = f(n as i32, &mut dp);

    write!(out, "{}", res).ok();

    // Bottom-Up solution
    // let mut dp: Vec<i64> = vec![0; n + 1];

    // dp[0] = 1;
    // for i in 0..n+1 {
    //     for j in 1..7 {
    //         let r = (i as i64) - (j as i64);
    //         if r >= 0 {
    //             dp[i] += dp[r as usize];
    //         }
    //         dp[i] %= MOD;
    //     }
    // }

    // write!(out, "{}", dp[n]).ok();
}