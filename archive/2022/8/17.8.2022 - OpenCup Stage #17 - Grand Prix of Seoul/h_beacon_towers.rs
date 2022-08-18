//{"name":"H. Beacon Towers","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/H/","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 4 2 5 3\n","output":"6\n"},{"input":"3\n3 2 1\n","output":"1\n"},{"input":"8\n6 3 1 7 2 5 4 8\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HBeaconTowers"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let h = input.read_usize_vec(n);

    let mut max_to_left = Vec::with_capacity(n + 1);
    max_to_left.push(0);
    let mut max = 0;
    for &i in &h {
        max.maxim(i);
        max_to_left.push(max);
    }
    let mut next_max = n;
    let mut from = vec![0; n];
    for i in (0..n).rev() {
        if max_to_left[i + 1] != max_to_left[i] {
            next_max = i;
        }
        from[i] = next_max + 1;
    }
    type Mod = ModInt7;
    let mut ft = FenwickTree::new(n + 1);
    ft.add(n, Mod::one());
    for i in (0..n).rev() {
        let cur = ft.get(from[i], n + 1);
        ft.add(i, cur);
    }
    out_line!(ft.get(0, 1));
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
