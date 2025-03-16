//{"name":"T520542 数组的划分","group":"Luogu","url":"https://www.luogu.com.cn/problem/T520542?contestId=183510","interactive":false,"timeLimit":3000,"tests":[{"input":"5 3 7 0\n3 1 2 3\n4 3 2 2 2\n3 3 2 2\n2 3 3 2 1\n3 1 5\n1 3\n3 1 5\n2 2 4 1 3 2\n3 1 5\n1 3\n3 1 5\n","output":"3\n4\n5\n4\n"},{"input":"10 5 20 0\n3 1 2 3\n5 3 3 1 1 3\n10 1 2 1 1 2 3 2 1 1 3\n2 1 1\n2 1 3\n1 3 2 3 3 1 3 3 2 3\n1 4\n2 7 7 3\n3 3 9\n1 4\n1 2\n2 5 5 2\n1 2\n2 7 7 2\n1 1\n3 5 8\n2 4 4 1\n3 3 8\n1 1\n1 3\n2 6 6 1\n2 1 1 1\n2 4 4 2\n1 7\n3 1 5\n3 1 9\n","output":"4\n2\n3\n4\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::hash::{CompositeHash, HashBase, SimpleHash, StringHash};
use algo_lib::value_ref;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let _id = input.read_size();
    let s = input.read_vec::<Vec<i64>>(m);
    let mut t = input.read_long_vec(n);

    HashBase::init();
    let mut present = FxHashSet::default();
    for i in 0..m {
        let hash = SimpleHash::new(&s[i]);
        for j in s[i].indices() {
            for k in j..s[i].len() {
                present.insert(hash.hash(j..=k));
            }
        }
    }
    value_ref!(Present: FxHashSet<u64> = present);
    struct Part {
        start: usize,
        blocks: BTreeSet<usize>,
        hash: SimpleHash,
        ans: Vec<usize>,
        to: Vec<usize>,
        one_step: Vec<usize>,
    }

    impl Default for Part {
        fn default() -> Self {
            Part {
                start: 0,
                blocks: BTreeSet::new(),
                hash: SimpleHash::new(&[0i64]),
                ans: vec![],
                to: vec![],
                one_step: vec![],
            }
        }
    }

    impl Part {
        fn new(start: usize, t: &[i64], next_block: Option<&Self>) -> Self {
            let hash = SimpleHash::new(t);
            let mut part = Part {
                start,
                blocks: BTreeSet::new(),
                hash,
                ans: vec![0; t.len()],
                to: vec![0; t.len()],
                one_step: vec![0; t.len()],
            };
            part.rebuild(next_block);
            part
        }

        fn rebuild(&mut self, next_block: Option<&Self>) {
            let hash = |mut l: usize, mut r: usize| {
                l -= self.start;
                r -= self.start;
                if let Some(next) = next_block {
                    let ch = CompositeHash::new(&self.hash, &next.hash);
                    ch.hash(l..r)
                } else {
                    self.hash.hash(l..r)
                }
            };
            let mut end = if let Some(next) = next_block {
                self.start
                    + self.ans.len()
                    + next
                        .blocks
                        .first()
                        .map_or(next.ans.len(), |&x| x - next.start)
            } else {
                self.start + self.ans.len()
            };
            for i in self.ans.indices().rev() {
                if self.blocks.contains(&(self.start + i + 1)) {
                    end = self.start + i + 1;
                }
                while !Present::with(|x| x.contains(&hash(i + self.start, end))) {
                    end -= 1;
                }
                self.one_step[i] = end;
                self.ans[i] = if end >= self.start + self.ans.len() {
                    1
                } else {
                    1 + self.ans[end - self.start]
                };
                self.to[i] = if end >= self.start + self.ans.len() {
                    end
                } else {
                    self.to[end - self.start]
                };
            }
        }

        fn replace(&mut self, t: &[i64], next_block: Option<&Self>) {
            self.hash = SimpleHash::new(t);
            self.rebuild(next_block);
        }
    }

    const BUBEN: usize = 50;
    let mut parts = Vec::with_gen_back(n.upper_div(BUBEN), |i, p| {
        Part::new(
            i * BUBEN,
            &t[i * BUBEN..n.min((i + 1) * BUBEN)],
            p.get(i + 1),
        )
    });

    for _ in 0..q {
        let command = input.read_int();
        match command {
            1 => {
                let x = input.read_size();
                if !parts[x / BUBEN].blocks.insert(x) {
                    parts[x / BUBEN].blocks.remove(&x);
                }
                let (head, tail) = parts.split_at_mut(x / BUBEN + 1);
                head[x / BUBEN].rebuild(tail.get(0));
                if x >= BUBEN {
                    let (head, tail) = parts.split_at_mut(x / BUBEN);
                    head[x / BUBEN - 1].rebuild(tail.get(0));
                }
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let a = input.read_long_vec(r - l);
                t[l..r].copy_from_slice(&a);
                for i in ((l / BUBEN).saturating_sub(1)..=(r - 1) / BUBEN).rev() {
                    let (head, tail) = parts.split_at_mut(i + 1);
                    head[i].replace(&t[i * BUBEN..n.min((i + 1) * BUBEN)], tail.get(0));
                }
            }
            3 => {
                let l = input.read_size() - 1;
                let r = input.read_size();

                let mut pos = l;
                let mut ans = 0;
                while pos / BUBEN < r / BUBEN {
                    let id = pos / BUBEN;
                    ans += parts[id].ans[pos - id * BUBEN];
                    pos = parts[id].to[pos - id * BUBEN];
                }
                let id = pos / BUBEN;
                while pos < r {
                    ans += 1;
                    pos = parts[id].one_step[pos - id * BUBEN];
                }
                out.print_line(ans);
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
