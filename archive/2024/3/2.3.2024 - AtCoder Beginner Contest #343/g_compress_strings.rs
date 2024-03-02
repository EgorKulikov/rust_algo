//{"name":"G - Compress Strings","group":"AtCoder - AtCoder Beginner Contest 343","url":"https://atcoder.jp/contests/abc343/tasks/abc343_g","interactive":false,"timeLimit":5000,"tests":[{"input":"3\nsnuke\nkensho\nuk\n","output":"9\n"},{"input":"3\nabc\nabc\narc\n","output":"6\n"},{"input":"6\ncmcmrcc\nrmrrrmr\nmrccm\nmmcr\nrmmrmrcc\nccmcrcmcm\n","output":"27\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GCompressStrings"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::hash::{HashBase, SimpleHash, StringHash};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut s = input.read_str_vec(n);

    s.sort_by_key(|x| x.len());
    HashBase::init();
    let mut ss = Vec::new();
    let mut hash = Vec::new();
    for s in &s {
        hash.push(SimpleHash::new(s))
    }
    for i in 0..n {
        let mut ok = true;
        for j in i + 1..n {
            for k in s[i].len()..=s[j].len() {
                if hash[i].hash(..) == hash[j].hash(k - s[i].len()..k) {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            ss.push((s[i].clone(), hash[i].clone()));
        }
    }
    let shift = Arr2d::generate(ss.len(), ss.len(), |i, j| {
        for k in (1..ss[i].0.len().min(ss[j].0.len())).rev() {
            if ss[i].1.hash(ss[i].0.len() - k..) == ss[j].1.hash(..k) {
                return ss[j].0.len() - k;
            }
        }
        ss[j].0.len()
    });
    let mut mem = Memoization2d::new(ss.len(), 1 << ss.len(), |mem, last, mask| {
        assert!(!mask.is_set(last));
        if mask == 0 {
            return ss[last].0.len();
        }
        let mut ans = None;
        for i in ss.indices() {
            if mask.is_set(i) {
                ans.minim(mem.call(i, mask.without_bit(i)) + shift[(i, last)]);
            }
        }
        ans.unwrap()
    });
    let mut ans = None;
    for i in ss.indices() {
        ans.minim(mem.call(i, usize::all_bits(ss.len()).without_bit(i)));
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
    let test_type = TestType::Single;
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
