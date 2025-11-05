//{"name":"C. Intercepting Butterflies","group":"Codeforces - Testing Round 20 (Unrated, Communication Problems)","url":"https://codeforces.com/contest/2168/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"first\n4\n1\n20\n50\n32768\n","output":"0\n\n3\n13 4 9\n4\n1 7 4 2\n10\n14 17 1 6 2 19 20 8 7 18\n"},{"input":"second\n4\n4\n4 5 9 13\n9\n1 2 6 7 8 14 17 18 19\n0\n\n5\n1 2 3 4 7\n","output":"20\n32768\n1\n50\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut rem = (0..1 << 20).collect::<BTreeSet<_>>();
    let mut x_to_s = vec![0; 1 << 15];
    let mut s_to_x = vec![0; 1 << 20];
    for x in usize::iter_all(15) {
        let s = rem.pop_first().unwrap();
        x_to_s[x] = s;
        s_to_x[s] = x;
        for i in 0..20 {
            let cs = s.flipped_bit(i);
            rem.remove(&cs);
            s_to_x[cs] = x;
            for j in 0..i {
                rem.remove(&cs.flipped_bit(j));
            }
        }
    }
    let mode = input.read_str();
    if mode.as_slice() == b"first" {
        let t = input.read_int();
        for _ in 0..t {
            let x = input.read_size() - 1;
            let s = x_to_s[x];
            let mut ans = Vec::new();
            for i in 0..20 {
                if s.is_set(i) {
                    ans.push(i + 1);
                }
            }
            out.print_line(ans.len());
            out.print_line(ans);
        }
    } else {
        let t = input.read_int();
        for _ in 0..t {
            let len = input.read_size();
            let s_list = input.read_size_vec(len).dec();
            let mut s = 0;
            for i in s_list {
                s.set_bit(i);
            }
            out.print_line(s_to_x[s] + 1);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::RunTwice;

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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
