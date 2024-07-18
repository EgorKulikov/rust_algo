//{"name":"C. София и утерянные операции","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3\n1 2 1\n1 3 2\n4\n1 3 1 2\n4\n1 2 3 5\n2 1 3 5\n2\n2 3\n5\n7 6 1 10 10\n3 6 1 11 11\n3\n4 3 11\n4\n3 1 7 8\n2 2 7 10\n5\n10 3 2 2 1\n5\n5 7 1 7 9\n4 10 1 2 9\n8\n1 1 9 8 7 2 10 4\n4\n1000000000 203 203 203\n203 1000000000 203 1000000000\n2\n203 1000000000\n1\n1\n1\n5\n1 3 4 5 1\n","output":"YES\nNO\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSofiyaIUteryannieOperatsii"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let m = input.read_size();
    let d = input.read_int_vec(m);

    if !b.contains(&d[m - 1]) {
        out.print_line(false);
        return;
    }
    let mut has = DefaultHashMap::<_, usize>::new();
    for i in d {
        has[i] += 1;
    }
    for i in 0..n {
        if a[i] != b[i] {
            if has[b[i]] == 0 {
                out.print_line(false);
                return;
            }
            has[b[i]] -= 1;
        }
    }
    out.print_line(true);
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
}
//END MAIN
