//{"name":"P11230 [CSP-J 2024] 接龙（民间数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11230?contestId=209924","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n3 3 7\n5 1 2 3 4 1\n3 1 2 5\n3 5 1 6\n1 2\n1 4\n2 4\n3 4\n6 6\n1 1\n7 7\n","output":"1\n0\n1\n0\n1\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11230CSPJ2024"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let s = input.read_vec::<Vec<usize>>(n).dec();

    let max = *s.iter().map(|v| v.iter().max().unwrap()).max().unwrap() + 1;

    #[derive(Copy, Clone)]
    enum Ans {
        None,
        Single(u32),
        Multiple,
    }

    impl Ans {
        #[inline]
        fn accumulate(&mut self, id: usize) {
            let id = id as u32;
            match self {
                Ans::None => *self = Ans::Single(id),
                Ans::Single(x) if *x != id => *self = Ans::Multiple,
                _ => {}
            }
        }

        #[inline]
        fn can(&self, id: usize) -> bool {
            match self {
                Ans::None => false,
                Ans::Single(x) => *x != id as u32,
                Ans::Multiple => true,
            }
        }
    }

    let mut ans = Arr2d::new(101, max, Ans::None);
    ans[(0, 0)] = Ans::Multiple;
    for i in 0..100 {
        for j in 0..n {
            let mut cur = 0;
            for l in s[j].indices() {
                if l < cur {
                    ans[(i + 1, s[j][l])].accumulate(j);
                }
                if ans[(i, s[j][l])].can(j) {
                    cur.maxim(l + k);
                }
            }
        }
    }

    for _ in 0..q {
        let r = input.read_size();
        let c = input.read_size() - 1;

        if c >= max {
            out.print_line(0);
            continue;
        }

        out.print_line(match ans[(r, c)] {
            Ans::None => 0,
            _ => 1,
        })
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
