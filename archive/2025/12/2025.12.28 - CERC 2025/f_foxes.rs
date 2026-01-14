//{"name":"F. Foxes","group":"Universal Cup - CERC 2025","url":"https://contest.ucup.ac/contest/2814/problem/16005","interactive":false,"timeLimit":1000,"tests":[{"input":"5 8\n3 4 1 7 2\n>\n! 2\n>\n>\n>\n! 10\n<\n! 11\n","output":"2\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut m = input.read_size_vec(n);

    struct State {
        n: usize,
        l_val: Vec<Vec<usize>>,
        l: Vec<usize>,
        r_val: Vec<Vec<usize>>,
        r: Vec<Reverse<usize>>,
        pairs: BTreeSet<(usize, usize, usize)>,
        ltr: Vec<Option<usize>>,
        rtl: Vec<Option<usize>>,
        pos: Vec<usize>,
        at: usize,
    }

    impl State {
        fn new(n: usize) -> Self {
            let mut pairs = BTreeSet::new();
            pairs.insert((0, 0, 0));
            Self {
                n,
                l_val: vec![vec![]],
                l: vec![0],
                pairs,
                ltr: vec![Some(0)],
                rtl: vec![Some(0)],
                pos: vec![n + 1; n],
                r_val: vec![vec![]],
                r: vec![Reverse(1_000_001)],
                at: n - 1,
            }
        }

        fn invalidate_right(&mut self, p: usize) {
            if let Some(l_pos) = self.rtl[p] {
                self.pairs.remove(&(l_pos + p, l_pos, p));
                self.rtl[p] = None;
                self.ltr[l_pos] = None;
                self.try_add_left(l_pos);
            }
        }

        fn invalidate_left(&mut self, p: usize) {
            if let Some(r_pos) = self.ltr[p] {
                self.pairs.remove(&(p + r_pos, p, r_pos));
                self.ltr[p] = None;
                self.rtl[r_pos] = None;
                self.try_add_right(r_pos);
            }
        }

        fn try_add_left(&mut self, p: usize) {
            let r_pos = self.r.lower_bound(&Reverse(self.l[p])) - 1;
            if let Some(cur_r) = self.ltr[p] {
                if cur_r == r_pos {
                    return;
                }
                self.pairs.remove(&(p + cur_r, p, cur_r));
                self.rtl[cur_r] = None;
                self.ltr[p] = None;
            }
            let r_pos = self.r.lower_bound(&Reverse(self.l[p])) - 1;
            if let Some(l_pos) = self.rtl[r_pos] {
                if l_pos >= p {
                    return;
                }
                self.ltr[l_pos] = None;
                self.pairs.remove(&(l_pos + r_pos, l_pos, r_pos));
            }
            self.rtl[r_pos] = Some(p);
            self.ltr[p] = Some(r_pos);
            self.pairs.insert((p + r_pos, p, r_pos));
        }

        fn try_add_right(&mut self, p: usize) {
            let l_pos = self.l.lower_bound(&self.r[p].0) - 1;
            if let Some(cur_l) = self.rtl[p] {
                if cur_l == l_pos {
                    return;
                }
                self.pairs.remove(&(cur_l + p, cur_l, p));
                self.ltr[cur_l] = None;
                self.rtl[p] = None;
            }
            if let Some(r_pos) = self.ltr[l_pos] {
                if r_pos >= p {
                    return;
                }
                self.rtl[r_pos] = None;
                self.pairs.remove(&(l_pos + r_pos, l_pos, r_pos));
            }
            self.ltr[l_pos] = Some(p);
            self.rtl[p] = Some(l_pos);
            self.pairs.insert((l_pos + p, l_pos, p));
        }

        fn remove_left(&mut self) {
            let p = self.pos[self.at - 1];
            self.l_val[p].pop();
            if self.l_val[p].is_empty() {
                self.l.pop();
                self.invalidate_left(p);
                self.ltr.pop();
                self.l_val.pop();
            } else {
                self.l[p] = self.l_val[p][Back(0)];
                self.invalidate_left(p);
                self.try_add_left(p);
            }
            self.pos[self.at - 1] = self.n + 1;
        }

        fn remove_right(&mut self) {
            let p = self.pos[self.at];
            self.r_val[p].pop();
            if self.r_val[p].is_empty() {
                self.r.pop();
                self.invalidate_right(p);
                self.rtl.pop();
                self.r_val.pop();
            } else {
                self.r[p] = Reverse(self.r_val[p][Back(0)]);
                self.invalidate_right(p);
                self.try_add_right(p);
            }
            self.pos[self.at] = self.n + 1;
        }

        fn add_left(&mut self, val: usize) {
            let p = self.l.lower_bound(&val);
            if p < self.l.len() {
                self.l[p] = val;
                self.invalidate_left(p);
            } else {
                self.l.push(val);
                self.ltr.push(None);
                self.l_val.push(Vec::new());
            }
            self.l_val[p].push(val);
            self.try_add_left(p);
            self.pos[self.at - 1] = p;
        }

        fn add_right(&mut self, val: usize) {
            let p = self.r.lower_bound(&Reverse(val));
            if p < self.r.len() {
                self.r[p] = Reverse(val);
                self.invalidate_right(p);
            } else {
                self.r.push(Reverse(val));
                self.rtl.push(None);
                self.r_val.push(Vec::new());
            }
            self.r_val[p].push(val);
            self.try_add_right(p);
            self.pos[self.at] = p;
        }

        fn move_left(&mut self) {
            self.at -= 1;
        }

        fn move_right(&mut self) {
            self.at += 1;
        }

        fn resut(&self) -> usize {
            self.pairs.last().unwrap().0
        }
    }

    let mut state = State::new(n);
    for i in (0..n).rev() {
        state.add_right(m[i]);
        if i != 0 {
            state.move_left();
        }
    }

    for _ in 0..q {
        let command = input.read_char();

        match command {
            b'<' => {
                state.remove_left();
                state.move_left();
                state.add_right(m[state.at]);
            }
            b'>' => {
                state.remove_right();
                state.move_right();
                state.add_left(m[state.at - 1]);
            }
            b'!' => {
                let val = input.read_size();
                state.remove_right();
                state.add_right(val);
                m[state.at] = val;
                out.print_line(state.resut());
            }
            _ => {
                unreachable!();
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
