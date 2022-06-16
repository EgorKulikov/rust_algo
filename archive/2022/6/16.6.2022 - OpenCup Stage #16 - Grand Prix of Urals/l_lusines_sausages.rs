//{"name":"L. Lusine’s Sausages","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/L/","interactive":false,"timeLimit":1000,"tests":[{"input":"3 100\n70 50 20\n30 50 80\n","output":"Outstanding\n"},{"input":"3 150\n70 50 20\n30 50 80\n","output":"Good Quality\n"},{"input":"3 201\n70 50 20\n30 50 80\n","output":"Low Quality\n"},{"input":"3 301\n70 50 20\n30 50 80\n","output":"Reject\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LLusinesSausages"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_usize_vec(n);
    let b = input.read_usize_vec(n);

    if a.iter().sum::<usize>() + b.iter().sum::<usize>() < k {
        out_line!("Reject");
        return;
    }
    for (w1, w2, verdict) in [(3, 7, "Outstanding"), (1, 1, "Good Quality")] {
        let mut free_weight = 0;
        let mut free_quality = 0;
        let mut bad = Vec::new();
        for (&a, &b) in a.iter().zip(b.iter()) {
            if w1 * a >= w2 * b {
                free_weight += a + b;
                free_quality += w1 * a - w2 * b;
            } else {
                bad.push((w2 * b - w1 * a, a + b));
            }
        }
        let mut best = vec![free_weight; free_quality + 1];
        for (q, w) in bad {
            if q > free_quality {
                continue;
            }
            for i in (0..=free_quality - q).rev() {
                let cand = best[i] + w;
                best[i + q].maxim(cand);
            }
        }
        if best[free_quality] >= k {
            out_line!(verdict);
            return;
        }
    }
    out_line!("Low Quality");
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
