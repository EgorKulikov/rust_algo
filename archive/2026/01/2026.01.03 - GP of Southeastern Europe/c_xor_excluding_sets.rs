//{"name":"C. XOR-Excluding Sets","group":"Universal Cup - GP of Southeastern Europe","url":"https://contest.ucup.ac/contest/2828/problem/16117","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n2\n3\n4\n5\n","output":"1\n2\n5\n11\n22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_utils::powers;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    type Mod = ModInt7;
    let pw = powers(Mod::new(2), n + 1);
    let mut basis = Vec::new();
    let mut special: Vec<(i64, Vec<i64>)> = Vec::new();
    fn normalize(basis: &mut Vec<i64>) {
        for i in basis.indices() {
            for j in i + 1..basis.len() {
                if basis[j] > basis[i] {
                    basis.swap(i, j);
                }
            }
            for j in i + 1..basis.len() {
                let cand = basis[j] ^ basis[i];
                basis[j].minim(cand);
            }
        }
        assert_ne!(basis[Back(0)], 0);
    }
    fn check(basis: &[i64], val: i64) -> bool {
        let mut cur = val;
        for b in basis.iter().copied() {
            cur.minim(b ^ cur);
        }
        cur != 0
    }
    for i in 0..n {
        let mut n_special = Vec::new();
        let added = if check(&basis, a[i]) {
            n_special.push((a[i], basis.clone()));
            basis.push(a[i]);
            normalize(&mut basis);
            true
        } else {
            false
        };
        for (val, mut basis) in special {
            if added {
                basis.push(a[i]);
                normalize(&mut basis);
                n_special.push((val, basis));
            } else {
                if !check(&basis, a[i]) {
                    n_special.push((val, basis));
                    continue;
                }
            }
        }
        let mut ans = pw[i + 1];
        if basis.len() == i + 1 {
            ans -= i + 1;
        } else {
            ans -= pw[i + 1 - basis.len()] * n_special.len();
            ans -= pw[i - basis.len()] * (i + 1 - n_special.len());
        }
        special = n_special;
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
