//{"name":"P11234 [CSP-S 2024] 擂台游戏（官方数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11234?contestId=209925","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5\n0 0 0 0 0\n5 4 1 2 3\n1001\n10\n1\n4\n2 1 0 0\n1 2 1 0\n0 2 3 1\n2 2 0 1\n","output":"5\n19\n7\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11234CSPS2024"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::splits::Split;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);
    let c = input.read_size_vec(m).dec();
    let k = (n - 1).ilog2() as usize + 1;
    let g = input.read_str_vec(k);
    let t = input.read_size();

    #[derive(Clone, PartialEq, Eq)]
    enum Winner {
        Exact((usize, usize)),
        Random((usize, usize)),
        Mixed {
            exact: Vec<(usize, usize)>,
            random_segs: (usize, usize),
        },
    }

    /*struct Winner {
        exact: Vec<(usize, usize)>,
        random_segs: Option<(usize, usize)>,
    }*/
    impl Winner {
        fn join_random(random: &mut (usize, usize), l: usize, r: usize) {
            if random.0 == r + 1 {
                random.0 = l;
            } else if l == random.1 + 1 {
                random.1 = r;
            } else {
                unreachable!();
            }
        }

        fn play(&mut self, player: &Winner, other: &Winner, round: usize) {
            match player {
                Winner::Exact((_, val)) => {
                    if *val >= round {
                        *self = player.clone();
                    } else {
                        *self = other.clone();
                    }
                    return;
                }
                Winner::Random((..)) => {
                    *self = player.clone();
                }
                Winner::Mixed {
                    exact: exact1,
                    random_segs: random_segs1,
                } => {
                    let mut exact = Vec::new();
                    for &(id, val) in exact1 {
                        if val >= round {
                            exact.push((id, val));
                        }
                    }
                    if exact.is_empty() {
                        *self = Winner::Random(*random_segs1);
                    } else {
                        *self = Winner::Mixed {
                            exact,
                            random_segs: *random_segs1,
                        };
                    }
                }
            }
            match other {
                Winner::Exact((id, val)) => match self {
                    Winner::Exact(_) => unreachable!(),
                    Winner::Random(random_segs) => {
                        *self = Winner::Mixed {
                            exact: vec![(*id, *val)],
                            random_segs: *random_segs,
                        };
                    }
                    Winner::Mixed { exact, .. } => exact.push((*id, *val)),
                },
                Winner::Random((l, r)) => match self {
                    Winner::Exact(_) => unreachable!(),
                    Winner::Random(random) => {
                        Self::join_random(random, *l, *r);
                    }
                    Winner::Mixed { random_segs, .. } => {
                        Self::join_random(random_segs, *l, *r);
                    }
                },
                Winner::Mixed {
                    exact: exact1,
                    random_segs: random_segs1,
                } => match self {
                    Winner::Random(random) => {
                        Self::join_random(random, random_segs1.0, random_segs1.1);
                        *self = Winner::Mixed {
                            exact: exact1.clone(),
                            random_segs: *random,
                        };
                    }
                    _ => unreachable!(),
                },
            }
        }

        fn value(&self) -> usize {
            match self {
                Winner::Exact((id, _)) => *id,
                Winner::Random((l, r)) => (l + r) * (r - l + 1) / 2,
                Winner::Mixed { exact, random_segs } => {
                    let mut res = 0;
                    for &(id, _) in exact {
                        res += id;
                    }
                    res +=
                        (random_segs.0 + random_segs.1) * (random_segs.1 - random_segs.0 + 1) / 2;
                    res
                }
            }
        }
    }
    let mut winners = vec![Vec::new(); k + 1];
    for i in 0..=k {
        winners[i] = vec![Winner::Exact((0, 0)); 1 << (k - i)];
    }
    let mut ans = vec![0; m];
    let mut id = vec![Vec::new(); n];
    for i in 0..m {
        id[c[i]].push(i);
    }
    for _ in 0..t {
        let x = input.read_size_vec(4);
        for i in winners[0].indices() {
            winners[0][i] = Winner::Random((i + 1, i + 1));
        }
        for j in 0..k {
            for i in 0..1 << (k - j - 1) {
                let (next, cur) = winners.two_mut(j + 1, j);
                next[i].play(&cur[2 * i], &cur[2 * i + 1], j + 1);
            }
        }
        for i in 0..n {
            winners[0][i] = Winner::Exact((i + 1, a[i] ^ x[(i + 1) & 3]));
            let mut at = i;
            let level = if i == 0 { 0 } else { i.ilog2() as usize + 1 };
            for j in 0..level {
                let other = at ^ 1;
                let (next, cur) = winners.two_mut(j + 1, j);
                let mut res = Winner::Exact((0, 0));
                if g[j][at >> 1] == b'0' {
                    res.play(&cur[at.min(other)], &cur[at.max(other)], j + 1);
                } else {
                    res.play(&cur[at.max(other)], &cur[at.min(other)], j + 1);
                }
                // max.maxim(res.exact.len());
                if res == next[at >> 1] {
                    break;
                }
                swap(&mut next[at >> 1], &mut res);
                at >>= 1;
            }
            let cur_ans = winners[level][0].value();
            for &j in &id[i] {
                ans[j] = cur_ans;
            }
        }
        let mut res = 0;
        for i in 0..m {
            res ^= (i + 1) * ans[i];
        }
        out.print_line(res);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
