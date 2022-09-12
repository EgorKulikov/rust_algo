//{"name":"F. Kirei and the Linear Function","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1003004\n4 1\n1 2 1\n179572007\n4 2\n2 7 3\n2 7 4\n111\n2 1\n2 2 6\n0000\n1 2\n1 4 0\n1 4 1\n484\n1 5\n2 2 0\n2 3 7\n1 2 5\n3 3 8\n2 2 6\n","output":"2 4\n1 5\n1 2\n-1 -1\n1 2\n-1 -1\n1 3\n1 3\n-1 -1\n-1 -1\n-1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKireiAndTheLinearFunction"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();
    let w = input.read_usize();

    let d = s
        .iter()
        .map(|c| (c - b'0').into_usize())
        .collect_vec()
        .as_slice()
        .partial_sums();

    let val = |l: usize, r: usize| -> usize { (d[r] - d[l]) % 9 };

    let mut first = vec![None; 9];
    let mut second = vec![None; 9];
    for i in 0..=s.len() - w {
        let v = val(i, i + w);
        if first[v].is_none() {
            first[v] = Some(i + 1);
        } else if second[v].is_none() {
            second[v] = Some(i + 1);
        }
    }

    let m = input.read_usize();
    for _ in 0..m {
        let l = input.read_usize() - 1;
        let r = input.read_usize();
        let v = input.read_usize();
        let mut ans = None;
        let cv = val(l, r);
        for i in 0..9 {
            for j in 0..9 {
                if (i * cv + j) % 9 == v {
                    if i == j {
                        if second[i].is_some() {
                            ans.minim((first[i].unwrap(), second[i].unwrap()));
                        }
                    } else {
                        if first[i].is_some() && first[j].is_some() {
                            ans.minim((first[i].unwrap(), first[j].unwrap()));
                        }
                    }
                }
            }
        }
        if let Some((l, r)) = ans {
            out_line!(l, r);
        } else {
            out_line!(-1, -1);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
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
}
//END MAIN
