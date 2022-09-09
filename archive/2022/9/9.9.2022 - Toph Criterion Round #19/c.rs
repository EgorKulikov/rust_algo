//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let h = input.read_usize_vec(n);

    let mut hc = h.clone();
    hc.sort();
    let mut vars = 0;
    let mut best = 0;
    for (i, j) in hc.into_iter().enumerate() {
        let cand = j * (n - i);
        if cand > best {
            best = cand;
            vars = j;
        }
    }
    let mut cur = Vec::with_capacity(n);
    let mut tail = Vec::with_capacity(n);
    let mut in_tail = false;
    for (id, &j) in h.iter().enumerate() {
        if j >= vars {
            in_tail = true;
            cur.push(id + 1);
        } else if in_tail {
            tail.push(id + 1);
        } else {
            cur.push(id + 1);
        }
    }
    cur.append(&mut tail);
    out_line!(cur);
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
