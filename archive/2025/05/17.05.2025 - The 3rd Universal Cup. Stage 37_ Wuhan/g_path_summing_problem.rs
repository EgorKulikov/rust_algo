//{"name":"G. Path Summing Problem","group":"Universal Cup - The 3rd Universal Cup. Stage 37: Wuhan","url":"https://contest.ucup.ac/contest/2025/problem/10742","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 3\n5 2 1\n1 5 5\n1 1\n1\n2 3\n3 3 3\n3 3 3\n","output":"7\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::number_ext::Square;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_table(n, m);

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + m - 1);
    let mut pos = DefaultHashMap::new(Vec::new());
    for i in 0..n {
        for j in 0..m {
            pos[a[(i, j)]].push((i, j));
        }
    }
    let mut ans = Mod::zero();
    let mut ways = Arr2d::new(n, m, Mod::zero());
    for (num, v) in pos {
        if v.len().square() > n * m {
            ans += c.c(n + m - 2, n - 1);
            if a[(0, 0)] == num {
                continue;
            }
            ways.fill(Mod::zero());
            ways[(0, 0)] = Mod::one();
            for j in 1..m {
                if a[(0, j)] == num {
                    break;
                }
                ways[(0, j)] = Mod::one();
            }
            for i in 1..n {
                if a[(i, 0)] != num {
                    ways[(i, 0)] = ways[(i - 1, 0)];
                }
                for j in 1..m {
                    if a[(i, j)] != num {
                        ways[(i, j)] = ways[(i - 1, j)] + ways[(i, j - 1)];
                    }
                }
            }
            ans -= ways[(n - 1, m - 1)];
        } else {
            let mut qty = vec![Mod::zero(); v.len()];
            for i in v.indices() {
                let (row, col) = v[i];
                qty[i] = c.c(row + col, row);
                for j in 0..i {
                    let (row2, col2) = v[j];
                    if row2 <= row && col2 <= col {
                        let s = qty[j] * c.c(row - row2 + col - col2, row - row2);
                        qty[i] -= s;
                    }
                }
                ans += qty[i] * c.c(n + m - row - col - 2, n - row - 1);
            }
        }
    }
    out.print_line(ans);
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
