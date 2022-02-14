//{"name":"G. Birthday","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n3\n","output":"-1\n3\n1 3\n2 2\n4 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GBirthday"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    if n == 2 {
        out_line!(-1);
        return;
    }
    let ans = solve_impl(n);
    out_line!(ans.len());
    output().print_per_line(&ans);
}

fn solve_impl(n: usize) -> Vec<(usize, usize)> {
    let mut target = 1;
    while target < n {
        target *= 2;
    }
    let mut qty = vec![0usize; target + 1];
    for i in 1..=n {
        qty[i] = 1;
    }
    let mut ans = Vec::new();
    let mut residue = 1;
    while residue < target {
        let mut cur_target = target;
        let mut v = Vec::new();
        for i in (residue..=n).step_by(2 * residue) {
            v.push(i);
        }
        v.reverse();
        for i in v {
            if (i & (i - 1)) == 0 {
                continue;
            }
            while cur_target > 2 * i {
                cur_target /= 2;
            }
            if qty[i] > 0 {
                assert!(qty[cur_target - i] >= qty[i]);
                let cur = qty[i];
                for _ in 0..cur {
                    ans.push((cur_target - i, i));
                }
                qty[cur_target - i] -= cur;
                qty[cur_target] += cur;
                qty[2 * i - cur_target] += cur;
                qty[i] = 0;
            }
        }
        residue *= 2;
    }
    let mut cur = 1;
    while cur < target {
        while qty[cur] > 0 {
            if qty[cur] >= 2 {
                ans.push((cur, cur));
                qty[2 * cur] += 1;
                qty[0] += 1;
                qty[cur] -= 2;
                continue;
            }
            if qty[0] == 0 {
                let mut n_cur = cur * 2;
                loop {
                    assert!(n_cur < target);
                    if qty[n_cur] > 1 {
                        ans.push((n_cur, n_cur));
                        qty[2 * n_cur] += 1;
                        qty[0] += 1;
                        qty[n_cur] -= 2;
                        break;
                    }
                    n_cur *= 2;
                }
            }
            ans.push((0, cur));
            ans.push((cur, cur));
            qty[cur] -= 1;
            qty[2 * cur] += 1;
        }
        cur *= 2;
    }
    while qty[0] > 0 {
        ans.push((0, target));
        qty[0] -= 1;
        qty[target] += 1;
    }
    ans
}

#[test]
fn test() {
    for i in 3..=50000 {
        eprintln!("n = {}", i);
        solve_impl(i);
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
