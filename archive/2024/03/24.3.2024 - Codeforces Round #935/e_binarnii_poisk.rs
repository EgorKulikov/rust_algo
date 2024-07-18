//{"name":"E. Бинарный поиск","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 3\n1 2 3 4 5 6\n6 5\n3 1 6 5 2 4\n5 1\n3 5 4 2 1\n6 3\n4 3 1 5 2 6\n3 2\n3 2 1\n","output":"0\n1\n3 4\n2\n2 4\n1 5\n2\n4 5\n2 4\n1\n1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBinarniiPoisk"}}}

// use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_size() - 1;
    let p = input.read_size_vec(n).dec();

    // let mut touched = BitSet::new(n);
    let mut l = 0;
    let mut r = n;
    while l + 1 < r {
        let m = (l + r) / 2;
        // touched.set(m);
        if p[m] <= x {
            l = m;
        } else {
            r = m;
        }
    }
    if p[l] == x {
        out.print_line(0);
        return;
    }
    let x_pos = p.iter().position(|&v| v == x).unwrap();
    // if !touched[x_pos] || p[l] <= x {
    out.print_line(1);
    out.print_line((x_pos + 1, l + 1));
    // return;
    // }
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
