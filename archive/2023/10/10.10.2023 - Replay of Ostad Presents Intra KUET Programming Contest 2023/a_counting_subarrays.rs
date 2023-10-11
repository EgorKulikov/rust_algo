//{"name":"A. Counting Subarrays","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"6 3\n1 3\n2 3\n5 5\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACountingSubarrays"}}}

use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut segments = input.read_size_pair_vec(m).dec();

    let mut ends = MultiTreeSet::new();
    for &(_, r) in &segments {
        ends.insert(r);
    }
    segments.sort();
    let mut start = 0;
    let mut ans = 0;
    for (l, r) in segments {
        let x = ends.first().copied().unwrap_or(n - 1);
        ans += (l + 1 - start) * (2 * x - start - l) / 2;
        start = l + 1;
        ends.remove(&r);
    }
    ans += (n - start) * (n - start + 1) / 2;
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
