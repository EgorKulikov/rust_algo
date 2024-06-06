//{"name":"D. Thala For A Reason","group":"Codeforces - Coding Challenge Alpha V - by AlgoRave","url":"https://codeforces.com/gym/105005/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n8\nbbbbbbbb\n14\ncbbccccbccccbb\n","output":"2\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DThalaForAReason"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut next = Arr2d::new(n + 1, 26, n);
    for i in (0..n).rev() {
        for j in 0..26 {
            next[(i, j)] = next[(i + 1, j)];
        }
        next[(i, s[i] as usize - 'a' as usize)] = i;
    }
    let mut qty = vec![0; 26];
    let mut ans = 0;
    let mut left = 0;
    for i in 0..n {
        let c = s[i] as usize - 'a' as usize;
        qty[c] += 1;
        if qty[c] > 7 {
            for j in left..=next[(left, c)] {
                let d = s[j] as usize - 'a' as usize;
                qty[d] -= 1;
            }
            left = next[(left, c)] + 1;
        }
        let mut to = left;
        for j in 0..26 {
            if qty[j] == 7 {
                to.maxim(next[(left, j)] + 1);
            }
        }
        ans += to - left;
    }
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
    let test_type = TestType::MultiNumber;
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
