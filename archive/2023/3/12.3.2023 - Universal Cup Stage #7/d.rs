//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let c = input.read_int_vec(n);

    let mut same = vec![false; n + 1];
    let mut base = vec![0; n + 1];
    let mut q_same = vec![0; n + 1];
    let mut q_diif = vec![0; n + 1];
    let mut cur_d = 1;
    for i in 1..=n {
        if (i + 1).count_ones() == 1 {
            same[i] = true;
            base[i] = i;
            cur_d *= 2;
        } else if i % 2 == 0 {
            same[i] = !same[i - 1];
            base[i] = base[i - 1];
        } else {
            let prev = i - cur_d;
            base[i] = base[prev];
            if 2 * (prev + 1) == cur_d {
                same[i] = !same[prev];
            } else {
                same[i] = same[prev];
            }
        }
        q_same[base[i]] += if !same[i] ^ (c[i - 1] == 1) { 1 } else { 0 };
        q_diif[base[i]] += if !same[i] ^ (c[i - 1] == -1) { 1 } else { 0 };
    }
    let mut ans = 0;
    for i in 1..=n {
        ans += q_same[i].min(q_diif[i]);
    }
    out_line!(ans);
}

#[test]
fn test() {
    use algo_lib::numbers::num_traits::bit_ops::BitOps;
    let mut c_good = vec![0i128];
    for n in 1..=100 {
        println!("n = {}", n);
        let mut n_good = Vec::new();
        for &g in &c_good {
            for m in [g, g + (1 << (n - 1))] {
                let mut pre_last = vec![1];
                let mut last = vec![0, 1];
                let mut good = true;
                for i in 0..n {
                    let mut next = vec![0i32];
                    next.extend_from_slice(&last);
                    let sgn = if m.is_set(i) { 1 } else { -1 };
                    for j in 0..pre_last.len() {
                        next[j] -= sgn * pre_last[j];
                    }
                    for &j in &next {
                        if j.abs() > 1 {
                            good = false;
                            break;
                        }
                    }
                    if !good {
                        break;
                    }
                    pre_last = last;
                    last = next;
                }
                if good {
                    for i in 0..n {
                        print!("{}", if m.is_set(i) { '+' } else { '-' });
                    }
                    println!();
                    n_good.push(m);
                }
            }
        }
        c_good = n_good;
    }
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
