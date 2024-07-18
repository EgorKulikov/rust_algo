//{"name":"D - Diversity of Scores","group":"AtCoder - AtCoder Beginner Contest 343","url":"https://atcoder.jp/contests/abc343/tasks/abc343_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n1 10\n3 20\n2 10\n2 10\n","output":"2\n3\n2\n2\n"},{"input":"1 3\n1 3\n1 4\n1 3\n","output":"1\n1\n1\n"},{"input":"10 10\n7 2620\n9 2620\n8 3375\n1 3375\n6 1395\n5 1395\n6 2923\n10 3375\n9 5929\n5 1225\n","output":"2\n2\n3\n3\n4\n4\n5\n5\n6\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDiversityOfScores"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let t = input.read_size();
    let changes = input.read_size_pair_vec(t);

    let mut ans = Vec::with_capacity(t);
    let mut score = vec![0; n];
    let mut by_score = DefaultHashMap::new();
    by_score[0] = n;
    for (a, b) in changes {
        let old = score[a - 1];
        let new = old + b;
        score[a - 1] = new;
        by_score[old] -= 1;
        by_score[new] += 1;
        if by_score[old] == 0 {
            by_score.remove(&old);
        }
        ans.push(by_score.len());
    }
    out.print_per_line(&ans);
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
            for i in 1..=t {
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
    //    tester::stress_test();
}
//END MAIN
