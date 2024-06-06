//{"name":"Minimum Increment Queries","group":"HackerRank - Bihari Coding Contest","url":"https://www.hackerrank.com/contests/bihari-coding-contest/challenges/minimum-increment-queries","interactive":false,"timeLimit":4000,"tests":[{"input":"7\n3 4 1 2 7 5 6\n5\n6\n10\n3\n30\n100000\n","output":"4\n5\n3\n8\n14289\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinimumIncrementQueries"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).sorted();

    let mut cur = a[0];
    let mut total = 0;
    let mut deltas = Vec::with_capacity(n - 1);
    for i in 1..n {
        total += (a[i] - cur) * i;
        deltas.push(total);
        cur = a[i];
    }
    let q = input.read_size();
    for _ in 0..q {
        let k = input.read_size();
        let pos = deltas.lower_bound(&k);
        let rem = if pos == 0 { k } else { k - deltas[pos - 1] };
        out.print_line(a[pos] + rem / (pos + 1));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
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
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
