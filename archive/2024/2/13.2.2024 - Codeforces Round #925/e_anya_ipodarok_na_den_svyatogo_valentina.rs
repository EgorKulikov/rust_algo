//{"name":"E. Аня и подарок на День святого Валентина","group":"Codeforces - Codeforces Round 925 (Div. 3)","url":"https://codeforces.com/contest/1931/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n2 2\n14 2\n3 5\n9 56 1\n4 10\n1 2007 800 1580\n4 5\n5000 123 30 4\n10 10\n6 4 6 2 3 1 10 9 10 7\n1 1\n6\n1 1\n10\n8 9\n1 2 9 10 10 2 10 2\n4 5\n10 10 10 10\n","output":"Sasha\nAnna\nAnna\nSasha\nSasha\nAnna\nAnna\nAnna\nSasha\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAnyaIPodarokNaDenSvyatogoValentina"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    let mut base = 0;
    let mut z = Vec::with_capacity(n);
    for mut i in a {
        let mut cur_z = 0;
        while i % 10 == 0 {
            base += 1;
            cur_z += 1;
            i /= 10;
        }
        while i > 0 {
            base += 1;
            i /= 10;
        }
        z.push(cur_z);
    }
    z.sort();
    z.reverse();
    for i in z.into_iter().step_by(2) {
        base -= i;
    }
    if base >= m + 1 {
        out.print_line("Sasha");
    } else {
        out.print_line("Anna");
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
