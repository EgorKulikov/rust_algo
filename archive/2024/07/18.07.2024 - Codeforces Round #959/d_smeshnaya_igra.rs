//{"name":"D. Смешная игра","group":"Codeforces - Codeforces Round 959 при поддержке NEAR (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1994/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n2\n1 4\n4\n99 7 1 13\n5\n10 2 31 44 73\n5\n87 6 81 44 32\n5\n62 35 33 79 16\n5\n6 51 31 69 42\n5\n52 63 25 21 5\n12\n33 40 3 11 31 43 37 8 50 5 12 22\n","output":"YES\n2 1\nYES\n4 1\n2 1\n3 2\nYES\n5 1\n4 1\n3 1\n2 1\nYES\n4 1\n3 1\n2 1\n5 4\nYES\n3 1\n5 1\n2 1\n4 2\nYES\n4 1\n5 1\n2 1\n3 2\nYES\n2 1\n5 2\n3 1\n4 3\nYES\n9 1\n12 9\n11 1\n10 1\n6 1\n7 6\n2 1\n8 2\n5 2\n3 1\n4 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSmeshnayaIgra"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = Vec::new();
    let mut used = BitSet::new(n);
    let mut id = vec![0; n - 1];
    for i in (1..n).rev() {
        id[..i].fill(n);
        for j in 0..n {
            if used[j] {
                continue;
            }
            let r = a[j] % i;
            if id[r] != n {
                ans.push((id[r] + 1, j + 1));
                used.set(j);
                break;
            }
            id[r] = j;
        }
    }
    ans.reverse();
    out.print_line(true);
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
