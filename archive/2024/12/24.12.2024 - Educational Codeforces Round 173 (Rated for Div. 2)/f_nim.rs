//{"name":"F. Nim","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/F","interactive":false,"timeLimit":6000,"tests":[{"input":"9 5\n0 1 2 1 3 4 5 6 0\n1 5\n2 5\n3 5\n4 5\n1 9\n","output":"4 1\n2 1\n0 1\n-1\n8 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNim"}}}

use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let qq = input.read_size();
    let a = input.read_size_vec(n);

    let q = Vec::gen(51, |i, _| {
        Vec::gen(n + 1, |j, q| {
            if j == 0 {
                0
            } else {
                q[j - 1] + if a[j - 1] == i { 1 } else { 0 }
            }
        })
    });

    type Mod = ModIntF;

    for _ in 0..qq {
        let l = input.read_size() - 1;
        let r = input.read_size();

        let mut mem = Memoization2d::new(52, 64, |mem, pos, xor| -> (usize, Mod) {
            if pos == 51 {
                if xor == 0 {
                    (0, Mod::one())
                } else {
                    (52, Mod::zero())
                }
            } else if q[pos][l] == q[pos][r] {
                mem.call(pos + 1, xor)
            } else {
                let (min, ways) = mem.call(pos + 1, xor);
                let (mut min2, mut ways2) = mem.call(pos + 1, xor ^ pos);
                min2 += 1;
                ways2 *= Mod::from_index(q[pos][r] - q[pos][l]);
                match min.cmp(&min2) {
                    std::cmp::Ordering::Less => (min, ways),
                    std::cmp::Ordering::Equal => (min, ways + ways2),
                    std::cmp::Ordering::Greater => (min2, ways2),
                }
            }
        });
        let mut min = 52;
        let mut ways = Mod::zero();
        for i in 0..=50 {
            if q[i][l] != q[i][r] {
                let (mut min2, mut ways2) = mem.call(i + 1, i);
                min2 += 1;
                ways2 *= Mod::from_index(q[i][r] - q[i][l]);
                match min.cmp(&min2) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => ways += ways2,
                    std::cmp::Ordering::Greater => {
                        min = min2;
                        ways = ways2;
                    }
                }
                if q[i][r] - q[i][l] >= 2 {
                    let min2 = 2;
                    let ways2 = Mod::from_index(q[i][r] - q[i][l] - 1)
                        * Mod::from_index(q[i][r] - q[i][l])
                        / Mod::new(2);
                    match min.cmp(&min2) {
                        std::cmp::Ordering::Less => (),
                        std::cmp::Ordering::Equal => ways += ways2,
                        std::cmp::Ordering::Greater => {
                            min = min2;
                            ways = ways2;
                        }
                    }
                }
            }
        }
        if min == 52 {
            out.print_line(-1);
        } else {
            out.print_line((r - l - min, ways));
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
