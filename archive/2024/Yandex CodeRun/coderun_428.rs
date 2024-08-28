//{"name":"coderun_428","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_428"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::hash::{HashBase, SimpleHash, StringHash};
use algo_lib::string::str::StrReader;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_str();
    let s = input.read_str();

    HashBase::init();
    let mut v = vec![0; s.len()];
    let mut need = 0;
    let mut good = HashSet::new();
    for c in 33..=126 {
        if s.iter().count_eq(&c) == 0 {
            continue;
        }
        need += 1;
        for i in s.indices() {
            if s[i] == c {
                v[i] = 1;
            } else {
                v[i] = 0;
            }
        }
        good.insert(SimpleHash::new(&v).hash(..));
    }
    let mut matches = vec![0; t.len()];
    v = vec![0; t.len()];
    for c in 33..=126 {
        if t.iter().count_eq(&c) == 0 {
            continue;
        }
        for i in t.indices() {
            if t[i] == c {
                v[i] = 1;
            } else {
                v[i] = 0;
            }
        }
        let hash = SimpleHash::new(&v);
        for i in 0..=t.len() - s.len() {
            if good.contains(&hash.hash(i..i + s.len())) {
                matches[i] += 1;
            }
        }
    }
    let mut ans = Vec::new();
    for i in matches.indices() {
        if matches[i] == need {
            ans.push(i + 1);
        }
    }
    out.print_line(ans.len());
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
