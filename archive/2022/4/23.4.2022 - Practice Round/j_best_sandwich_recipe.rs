//{"name":"J. Best Sandwich Recipe","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/J","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3 4\n2 3 1\n3 3 2\n2 3 1 4\n","output":"2\n"},{"input":"5 5 5\n3 1 4 1 5\n9 4 8 8 5\n9 8 6 9 6\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JBestSandwichRecipe"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n_b = input.read_usize();
    let n_c = input.read_usize();
    let n_s = input.read_usize();
    let mut b = input.read_int_vec(n_b);
    let mut c = input.read_int_vec(n_c);
    let mut s = input.read_int_vec(n_s);

    b.sort_unstable();
    c.sort_unstable();
    s.sort_unstable();
    let mut j = 0;
    let mut k = 0;
    let mut ans = 0;
    for b in b {
        while j < n_c && c[j] <= b {
            j += 1;
        }
        if j == n_c {
            break;
        }
        while k < n_s && s[k] <= c[j] {
            k += 1;
        }
        if k == n_s {
            break;
        }
        ans += 1;
        j += 1;
        k += 1;
    }
    out_line!(ans);
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
