//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use std::cmp::min;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();

    let mut segs = Vec::new();
    let mut i = 2;
    let mut to = n;
    loop {
        let from = n / i + 1;
        if from > to {
            for i in (1..=to).rev() {
                segs.push((i, i));
            }
            break;
        }
        segs.push((from, to));
        to = from - 1;
        i += 1;
    }
    segs.reverse();
    type Mod = ModIntF;
    let mut val = vec![Mod::zero(); segs.len()];
    for (pos, &(from, _)) in segs.iter().enumerate().rev() {
        let mut i = 2;
        let mut res = Mod::one();
        let mut c_pos = pos;
        while i <= 20210926 {
            if i * from > n {
                break;
            }
            let n_pos = segs[c_pos + 1..].lower_bound(&(i * from, n + 1)) + c_pos;
            let ni = (segs[n_pos].1 / from).min(20210926);
            res += Mod::from_index(ni - i + 1) * val[n_pos];
            i = ni + 1;
            c_pos = n_pos;
        }
        val[pos] = res;
    }
    out_line!(val[0]);
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
    // stress_test::stress_test(run, tester::check);
}
//END MAIN
