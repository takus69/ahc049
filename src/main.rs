use proconio::input;
use std::time::{Duration, Instant};

struct Solver {
    n: usize,
    w: Vec<Vec<usize>>,
    d: Vec<Vec<usize>>,
    board: Vec<Vec<bool>>,
    r: usize,
    t: usize,
    ans: Vec<char>,
    timer: Instant,
}

impl Solver {
    fn new() -> Self {
        input! {
            n: usize,
            w: [[usize; n]; n],
            d: [[usize; n]; n],
        }

        let mut board: Vec<Vec<bool>> = vec![vec![true; n]; n];
        board[0][0] = false;

        let t = 0;
        let r = n*n - 1;
        let ans: Vec<char> = Vec::new();
        let timer = Instant::now();

        Self { n, w, d, board, t, r, ans, timer }
    }

    fn duration(&self) -> usize {
        self.timer.elapsed().as_millis() as usize
    }

    fn is_time(&self, duration: usize) -> bool {
        self.duration() <= duration
    }
    
    fn carry(&mut self, i: usize, j: usize, carry: &mut Vec<(usize, usize, usize)>, ans: &mut Vec<char>) {
        // 消費耐久力の更新
        let lost = self.w[i][j]*(i+j);
        for i in 0..carry.len() {
            carry[i].2 += lost;
        }
        
        ans.push('1');
        carry.push((self.w[i][j], self.d[i][j], 0));  // (重さ、耐久力、消消耐久力)
        self.board[i][j] = false;
    }

    fn r#move(&mut self, i: usize, j: usize, i2: usize, j2: usize, ans: &mut Vec<char>) -> (usize, usize) {
        for _ in 0..i.abs_diff(i2) {
            if i < i2 {
                ans.push('D');
            } else {
                ans.push('U');
            }
            self.t += 1;
        }
        for _ in 0..j.abs_diff(j2) {
            if j < j2 {
                ans.push('R');
            } else {
                ans.push('L');
            }
            self.t += 1;
        }
        
        (i2, j2)
    }

    fn carriable(&self, i: usize, j: usize, carry: &Vec<(usize, usize, usize)>) -> bool {
        if !self.board[i][j] { return false; }

        let lost = self.w[i][j]*(i+j);
        for &(_, d, lost_d) in carry.iter() {
            if d <= lost_d + lost {
                return false;
            }
        }
        
        true
    }

    fn solve(&mut self) {
        eprintln!("solve start: {} ms", self.duration());

        // 一番右下から順に処理する
        let mut ans: Vec<char> = Vec::new();
        for s in (1..(self.n+self.n-1)).rev() {
            for i in 0..=(s.min(self.n-1)) {
                let j = s-i;
                if j >= self.n { continue; }
                let mut carry: Vec<(usize, usize, usize)> = Vec::new();  // (重さ、耐久力、消消耐久力)
                let mut now_i = 0;
                let mut now_j = 0;
                if !self.carriable(i, j, &carry) { continue; }

                // 運び出し開始
                (now_i, now_j) = self.r#move(now_i, now_j, i, j, &mut ans);
                self.carry(i, j, &mut carry, &mut ans);
                // 左に移動
                let mut j2 = now_j;
                for _ in 0..j {
                    j2 -= 1;
                    (now_i, now_j) = self.r#move(now_i, now_j, now_i, j2, &mut ans);
                    if self.carriable(now_i, now_j, &carry) {
                        self.carry(now_i, now_j, &mut carry, &mut ans);
                    }
                }
                // 上に移動
                let mut i2 = now_i;
                for _ in 0..i {
                    i2 -= 1;
                    (now_i, now_j) = self.r#move(now_i, now_j, i2, now_j, &mut ans);
                    if self.carriable(now_i, now_j, &carry) {
                        self.carry(now_i, now_j, &mut carry, &mut ans);
                    }
                }
                self.r -= carry.len();
            }
        }
        self.ans = ans;

        eprintln!("solve end: {} ms", self.duration());
    }

    fn score(&self) -> usize {
        if self.r > 0 {
            self.n*self.n - self.r
        } else {
            self.n*self.n + 2*self.n*self.n*self.n - self.t
        }
    }

    fn ans(&self) {
        for a in self.ans.iter() {
            println!("{}", a);
        }
    }

    fn result(&self) {
        eprintln!("{{ \"n\": {}, \"score\": {} }}", self.n, self.score());
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
    solver.ans();
    solver.result();
}
