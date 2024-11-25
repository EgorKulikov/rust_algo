//{"name":"Spoiled Milk","group":"TLX - TLX Regular Open Contest #41","url":"https://tlx.toki.id/contests/troc-41/problems/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6 3 4\n2 2 2 0 1 0\n","output":"YA\n1 1 0 0 1 1\n"},{"input":"6 4 4\n0 0 0 0 0 0\n","output":"YA\n0 0 0 0 0 0\n"},{"input":"5 4 3\n0 3 1 2 1\n","output":"TIDAK\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SpoiledMilk"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let t = input.read_size();
    let c = input.read_size_vec(n);

    let g = gcd(n, t);
    let mut ans = vec![0; n];
    let mut free = vec![0; g];
    for i in 0..g {
        let mut cur = i;
        let mut min = 0;
        let mut max = 0;
        let mut cc = 0;
        for _ in 0..n / g {
            let next = (cur + 1) % n;
            if c[cur] == c[next] {
            } else if (c[cur] + 1) % k == c[next] {
                cc += 1;
            } else if (c[next] + 1) % k == c[cur] {
                cc -= 1;
            } else {
                out.print_line("TIDAK");
                return;
            }
            min.minim(cc);
            max.maxim(cc);
            cur = (cur + t) % n;
        }
        if max - min > 1 || cc != 0 {
            out.print_line("TIDAK");
            return;
        }
        if max == min {
            free[i] = 1;
        }
        cc = -min;
        for _ in 0..n / g {
            ans[cur] = cc as usize;
            let next = (cur + 1) % n;
            if c[cur] == c[next] {
            } else if (c[cur] + 1) % k == c[next] {
                cc += 1;
            } else if (c[next] + 1) % k == c[cur] {
                cc -= 1;
            }
            cur = (cur + t) % n;
        }
    }
    let times = t / g;
    let mut sum = ans.copy_take(t).sum::<usize>() % k;
    let f = free.copy_sum();
    for mut delta in 0..=f {
        if sum == c[0] {
            for i in 0..g {
                if delta == 0 {
                    break;
                }
                if free[i] == 1 {
                    for j in (i..n).step_by(g) {
                        ans[j] += 1;
                    }
                    delta -= 1;
                }
            }
            out.print_line("YA");
            out.print_line(ans);
            return;
        }
        sum += times;
        sum %= k;
    }
    out.print_line("TIDAK");
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
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
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
