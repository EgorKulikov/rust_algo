//{"name":"A. Не пытайтесь посчитать","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/contest/1881/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n1 5\na\naaaaa\n5 5\neforc\nforce\n2 5\nab\nababa\n3 5\naba\nababa\n4 3\nbabb\nbbb\n5 1\naaaaa\na\n4 2\naabb\nba\n2 8\nbk\nkbkbkbkb\n12 2\nfjdgmujlcont\ntf\n2 2\naa\naa\n3 5\nabb\nbabba\n1 19\nm\nmmmmmmmmmmmmmmmmmmm\n","output":"3\n1\n2\n-1\n1\n0\n1\n3\n1\n0\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANePitaitesPoschitat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::composite_slicelike::SlicelikeZip;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::z_algorithm::ZAlgorithm;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let _n = input.read_size();
    let _m = input.read_size();
    let mut x = input.read_str();
    let s = input.read_str();

    let mut ans = 0;
    let mut last = false;
    loop {
        let c = s.as_slice().zip(x.as_slice());
        let z = c.z_algorithm();
        if z.into_iter().skip(s.len()).any(|x| x >= s.len()) {
            out.print_line(ans);
            return;
        }
        if last {
            out.print_line(-1);
            return;
        }
        if x.len() >= s.len() {
            last = true;
        }
        ans += 1;
        x += x.clone();
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
