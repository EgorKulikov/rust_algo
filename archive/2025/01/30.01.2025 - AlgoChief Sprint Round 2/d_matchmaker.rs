//{"name":"D. Matchmaker","group":"Codeforces - AlgoChief Sprint Round 2","url":"https://codeforces.com/gym/105687/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 4\nabcdef\n1 6\n2 2\n3 6\n1 4\n6 3\naaabbb\n1 3\n4 6\n1 6\n","output":"3\n0\n2\n2\n0\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    let qq: Vec<[i32; 26]> = Vec::with_gen_prefix(n + 1, |i, q| {
        if i == 0 {
            [0; 26]
        } else {
            let mut v: [i32; 26] = q[i - 1];
            v[s[i - 1] as usize - 'a' as usize] += 1;
            v
        }
    });

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();

        let mut ans = 0;
        for i in 0..26 {
            ans += (qq[r][i] - qq[l][i]) % 2;
        }
        out.print_line(ans / 2);
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
//END MAIN
