//{"name":"D. Сортировка умножением","group":"Codeforces - Educational Codeforces Round 154 (Rated for Div. 2)","url":"https://codeforces.com/contest/1861/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n1 1 2 2 2\n6\n5 4 3 2 5 1\n3\n1 2 3\n","output":"3\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSortirovkaUmnozheniem"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = 0;
    for (i, j) in a.consecutive_iter() {
        if i >= j {
            ans += 1;
        }
    }
    let mut tail = ans;
    let mut head = 0;
    for (i, j) in a.consecutive_iter() {
        if i >= j {
            tail -= 1;
        }
        ans.minim(tail + head + 1);
        if i <= j {
            head += 1;
        }
    }
    ans.minim(head + 1);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
