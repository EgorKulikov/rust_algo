//{"name":"A good subset","group":"HackerEarth - January Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/january-circuits-022/algorithm/good-subset-88fda603/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 6\n1 4 3\n5 400\n3 1 4 1 5\n6 20\n10 4 3 10 25 2\n","output":"1\n5\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGoodSubset"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut cur = BitSet::new(k);
    cur.set(0, true);
    let mut front = Vec::with_capacity(n);
    for &a in &a {
        front.push(cur.clone());
        for j in (a..k).rev() {
            if cur[j - a] {
                cur.set(j, true);
            }
        }
    }
    cur.fill(false);
    cur.set(0, true);
    let mut ans = n;
    for (a, front) in a.into_iter().zip(front.into_iter()).rev() {
        let mut from = k;
        let mut to = k;
        let mut between = 0;
        for i in front.iter() {
            let n_to = k - i;
            let n_from = if n_to >= a { n_to - a } else { 0 };
            for j in n_from..from {
                if cur[j] {
                    between += 1;
                }
            }
            for j in n_to..to {
                if cur[j] {
                    between -= 1;
                }
            }
            if between > 0 {
                ans -= 1;
                break;
            }
            from = n_from;
            if from == 0 {
                break;
            }
            to = n_to;
        }
        for j in (a..k).rev() {
            if cur[j - a] {
                cur.set(j, true);
            }
        }
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
