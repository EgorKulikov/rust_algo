//{"name":"E. Деревянная игра","group":"Codeforces - Codeforces Round 959 при поддержке NEAR (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1994/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1\n1\n2\n4\n1 2 2\n6\n1 1 3 1 3\n1\n10\n1 2 2 1 1 5 7 6 4\n","output":"1\n7\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDerevyannayaIgra"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let k = input.read_size();
    let mut sizes = Vec::with_capacity(k);
    for _ in 0..k {
        let n = input.read_size();
        input.read_size_vec(n - 1);
        sizes.push(n);
    }
    sizes.sort();
    sizes.reverse();
    let mut ans = 0;
    for i in sizes {
        if (ans & i) == 0 {
            ans += i;
            continue;
        }
        for j in (0..20).rev() {
            if ans.is_set(j) && i.is_set(j) {
                ans |= i;
                ans |= usize::all_bits(j);
                break;
            }
        }
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
}
//END MAIN
