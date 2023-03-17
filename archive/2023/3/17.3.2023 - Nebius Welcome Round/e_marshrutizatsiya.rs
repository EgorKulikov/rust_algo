//{"name":"E. Маршрутизация","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6 7\n1 2\n2 3\n3 1\n4 5\n5 6\n4 6\n2 5\n","output":"Yes\n2 5 2 5 2 5\n"},{"input":"3 2\n1 2\n2 3\n","output":"Yes\n2 1 2\n"},{"input":"4 4\n1 3\n2 3\n4 1\n4 2\n","output":"Yes\n3 3 1 1\n"},{"input":"6 5\n1 2\n2 3\n3 4\n4 5\n5 6\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMarshrutizatsiya"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    set_bool_output(BoolOutput::YesNo);

    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec_by_one();

    let mut g = vec![0usize; n];
    for (u, v) in edges {
        g[u].set_bit(v);
        g[v].set_bit(u);
    }
    let mut reachable = Arr2d::new(1 << n, n, n);
    for j in 0..n {
        reachable.fill(n);
        reachable[(1 << j, j)] = j;
        for i in 1..(1 << n) {
            for k in 0..n {
                if reachable[(i, k)] == n {
                    continue;
                }
                let mut good = g[j].is_set(k);
                for l in 0..n {
                    if i.is_set(l) {
                        continue;
                    }
                    if (g[l] & i) == 0 {
                        good = false;
                    }
                    if g[k].is_set(l) {
                        reachable[(i.with_bit(l), l)] = k;
                    }
                }
                if good {
                    let mut ans = vec![0; n];
                    let mut mask = i;
                    let mut v = k;
                    while v != j {
                        ans[v] = reachable[(mask, v)] + 1;
                        mask.unset_bit(v);
                        v = ans[v] - 1;
                    }
                    ans[j] = k + 1;
                    for l in 0..n {
                        if ans[l] == 0 {
                            for m in 0..n {
                                if g[l].is_set(m) && i.is_set(m) {
                                    ans[l] = m + 1;
                                    break;
                                }
                            }
                        }
                        assert_ne!(ans[l], 0);
                    }
                    out_line!(true);
                    out_line!(ans);
                    return;
                }
            }
        }
    }
    out_line!(false);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
