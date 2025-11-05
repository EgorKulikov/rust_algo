//{"name":"#include&lt;scoring&gt;","group":"Kattis","url":"https://open.kattis.com/problems/includescoring","interactive":false,"timeLimit":1000,"tests":[{"input":"41\n6 179 65 1\n6 305 86 1\n6 324 96 0\n6 390 112 1\n5 280 97 0\n4 79 45 1\n4 94 49 1\n4 126 55 1\n4 160 100 1\n4 173 76 0\n4 214 106 1\n4 221 110 1\n4 226 96 1\n4 384 103 0\n3 35 26 1\n3 90 56 0\n3 113 83 1\n3 137 106 0\n3 171 101 1\n3 184 104 0\n3 195 65 1\n2 14 11 1\n2 15 10 1\n2 17 11 1\n2 19 12 1\n2 20 13 1\n2 21 14 0\n2 29 20 1\n2 30 19 0\n2 36 27 1\n2 39 13 1\n2 52 23 1\n2 52 31 1\n2 69 23 1\n2 86 17 0\n2 113 55 1\n2 125 62 1\n2 328 119 0\n1 10 10 0\n1 15 15 1\n1 33 33 0\n","output":"101\n76\n60\n51\n45\n41\n37\n33\n30\n26\n25\n23\n21\n18\n17\n15\n15\n13\n13\n11\n11\n10\n9\n8\n7\n6\n4\n4\n2\n2\n1\n1\n1\n1\n0\n1\n1\n0\n0\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::LegacyTaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let spfo = input.read_vec::<(i64, i64, i64, usize)>(n);

    let mut score = vec![
        100, 75, 60, 50, 45, 40, 36, 32, 29, 26, 24, 22, 20, 18, 16, 15, 14, 13, 12, 11, 10, 9, 8,
        7, 6, 5, 4, 3, 2, 1,
    ];
    score.resize(n, 0);
    let (s, p, f, mut ans) = spfo.detuple();
    for i in 0..n {
        let mut less = 0;
        let mut same = 0;
        for j in 0..n {
            if s[j] > s[i] {
                less += 1;
            } else if s[j] == s[i] {
                if p[j] < p[i] {
                    less += 1;
                } else if p[j] == p[i] {
                    if f[j] < f[i] {
                        less += 1;
                    } else if f[j] == f[i] {
                        same += 1;
                    }
                }
            }
        }
        let mut sum = 0;
        for j in less..less + same {
            sum += score[j];
        }
        ans[i] += sum.upper_div(same);
    }
    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: LegacyTaskType = LegacyTaskType::Classic;

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
        LegacyTaskType::Classic => input.is_empty(),
        LegacyTaskType::Interactive => true,
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
