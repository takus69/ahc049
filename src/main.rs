use proconio::input;
use std::time::{Duration, Instant};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solver {
    n: usize,
    w: Vec<Vec<usize>>,
    d: Vec<Vec<usize>>,
    board: Vec<Vec<bool>>,
    r: usize,
    t: usize,
    ans: Vec<char>,
    score: usize,
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
        let score = 0;
        let timer = Instant::now();

        Self { n, w, d, board, t, r, ans, score, timer }
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
        self.ans = ans.clone();
        self.score = self.score();
        let mut opt_score = self.score();
        let mut opt_ans = ans;

        // solverをリセット
        self.board = vec![vec![true; self.n]; self.n];
        self.board[0][0] = false;
        self.t = 0;
        self.r = self.n*self.n - 1;
        self.ans = Vec::new();

        let mut ans: Vec<char> = Vec::new();
        let mut cnt: usize = 0;
        for s in (1..(self.n+self.n-1)).rev() {
            for i in 0..=(s.min(self.n-1)) {
                let j = s-i;
                if j >= self.n { continue; }
                if !self.carriable(i, j, &Vec::new()) { continue; }

                // ビームサーチ
                let beam_width = 100;
                let mut beams: BinaryHeap<(Reverse<usize>, Vec<(usize, usize)>, Vec<(usize, usize, usize)>)> = BinaryHeap::new();
                beams.push((Reverse(0), vec![(i, j)], vec![(self.w[i][j], self.d[i][j], 0)]));
                let mut opt_beams: BinaryHeap<(usize, Vec<(usize, usize)>)> = BinaryHeap::new();
                while let Some((Reverse(eval), order, carry)) = beams.pop() {
                    let mut next_beams: BinaryHeap<(Reverse<usize>, Vec<(usize, usize)>, Vec<(usize, usize, usize)>)> = BinaryHeap::new();
                    let &(i, j) = order.last().unwrap();
                    // 次に乗せる箱の候補を確認
                    for s in (0..(i+j)).rev() {
                        for i2 in 0..=(s.min(i)) {
                            let j2 = s-i2;
                            if j2 > j { continue; }
                            if !self.carriable(i2, j2, &carry) { continue; }
                            let mut next_order = order.clone();
                            next_order.push((i2, j2));
                            let mut next_carry = carry.clone();
                            // 消費耐久力の更新
                            let lost = self.w[i2][j2]*(i2+j2);
                            for i in 0..carry.len() {
                                next_carry[i].2 += lost;
                            }
                            next_carry.push((self.w[i2][j2], self.d[i2][j2], 0));
                            next_beams.push((Reverse(eval+(i2+j2)*2), next_order, next_carry));
                            if next_beams.len() > beam_width { next_beams.pop(); }
                        }
                    }
                    // eprintln!("next_beams: {:?}", next_beams);
                    if beams.is_empty() {
                        opt_beams.push((eval, order.clone()));
                        beams = next_beams;
                    }
                }
                
                // 評価値が一番高い処理を実施
                // eprintln!("opt_beams: {:?}", opt_beams);
                let (_, order) = opt_beams.pop().unwrap();

                // 運び出し開始
                let mut carry: Vec<(usize, usize, usize)> = Vec::new();  // (重さ、耐久力、消消耐久力)
                let mut now_i = 0;
                let mut now_j = 0;
                // eprintln!("order: {}, {:?}", order.len(), order);
                for &(i, j) in order.iter() {
                    (now_i, now_j) = self.r#move(now_i, now_j, i, j, &mut ans);
                    self.carry(i, j, &mut carry, &mut ans);
                }
                (now_i, now_j) = self.r#move(now_i, now_j, 0, 0, &mut ans);
                self.r -= carry.len();
                cnt += 1;
            }
        }
        self.ans = ans.clone();

        let score = self.score();
        eprintln!("score: {}, cnt: {}", score, cnt);
        if opt_score > score {
            self.ans = opt_ans.clone();
        } else {
            opt_score = score;
            opt_ans = ans;
            self.score = score;
        }

        // solverをリセット
        self.board = vec![vec![true; self.n]; self.n];
        self.board[0][0] = false;
        self.t = 0;
        self.r = self.n*self.n - 1;
        self.ans = Vec::new();

        // 耐久力が高い箱から実施
        let mut heap: BinaryHeap<(usize, usize, usize, usize)> = BinaryHeap::new();
        for i in 0..self.n {
            for j in 0..self.n {
                if self.w[i][j] == 0 { continue; }
                heap.push(((i+j)*(i+j)*self.d[i][j], self.w[i][j], i, j));
            }
        }

        let mut ans: Vec<char> = Vec::new();
        let mut cnt: usize = 0;
        while let Some((_, _, i, j)) = heap.pop() {
            if !self.carriable(i, j, &Vec::new()) { continue; }

            // ビームサーチ
            let beam_width = 100;
            let mut beams: BinaryHeap<(Reverse<usize>, Vec<(usize, usize)>, Vec<(usize, usize, usize)>)> = BinaryHeap::new();
            beams.push((Reverse(0), vec![(i, j)], vec![(self.w[i][j], self.d[i][j], 0)]));
            let mut opt_beams: BinaryHeap<(usize, Vec<(usize, usize)>)> = BinaryHeap::new();
            while let Some((Reverse(eval), order, carry)) = beams.pop() {
                let mut next_beams: BinaryHeap<(Reverse<usize>, Vec<(usize, usize)>, Vec<(usize, usize, usize)>)> = BinaryHeap::new();
                let &(i, j) = order.last().unwrap();
                // 次に乗せる箱の候補を確認
                for s in (0..(i+j)).rev() {
                    for i2 in 0..=(s.min(i)) {
                        let j2 = s-i2;
                        if j2 > j { continue; }
                        if !self.carriable(i2, j2, &carry) { continue; }
                        let mut next_order = order.clone();
                        next_order.push((i2, j2));
                        let mut next_carry = carry.clone();
                        // 消費耐久力の更新
                        let lost = self.w[i2][j2]*(i2+j2);
                        for i in 0..carry.len() {
                            next_carry[i].2 += lost;
                        }
                        next_carry.push((self.w[i2][j2], self.d[i2][j2], 0));
                        next_beams.push((Reverse(eval+(i2+j2)*2), next_order, next_carry));
                        if next_beams.len() > beam_width { next_beams.pop(); }
                    }
                }
                // eprintln!("next_beams: {:?}", next_beams);
                if beams.is_empty() {
                    opt_beams.push((eval, order.clone()));
                    beams = next_beams;
                }
            }
            
            // 評価値が一番高い処理を実施
            // eprintln!("opt_beams: {:?}", opt_beams);
            let (_, order) = opt_beams.pop().unwrap();

            // 運び出し開始
            let mut carry: Vec<(usize, usize, usize)> = Vec::new();  // (重さ、耐久力、消消耐久力)
            let mut now_i = 0;
            let mut now_j = 0;
            // eprintln!("order: {}, {:?}", order.len(), order);
            for &(i, j) in order.iter() {
                (now_i, now_j) = self.r#move(now_i, now_j, i, j, &mut ans);
                self.carry(i, j, &mut carry, &mut ans);
            }
            (now_i, now_j) = self.r#move(now_i, now_j, 0, 0, &mut ans);
            self.r -= carry.len();
            cnt += 1;
        }
        self.ans = ans.clone();

        let score = self.score();
        eprintln!("score: {}, cnt: {}", score, cnt);
        if opt_score > score {
            self.ans = opt_ans.clone();
        } else {
            opt_score = score;
            opt_ans = ans;
            self.score = score;
        }

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
        eprintln!("{{ \"n\": {}, \"score\": {} }}", self.n, self.score);
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
    solver.ans();
    solver.result();
}
