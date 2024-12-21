//{"name":"T472521 「ALFR Round 3」D 核裂变","group":"Luogu","url":"https://www.luogu.com.cn/problem/T472521?contestId=210959","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3 1\n1\n1 2\n1 3\n1 1\n","output":"6 4 2\n"},{"input":"3 1000000000000000000 1\n1\n1 2\n1 3\n1 1\n","output":"151723985 433897441 433897439\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T472521ALFRRound3D"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();
    let v = input.read_size_vec(m).dec();
    let x = input.read_vec::<Vec<usize>>(n).dec();

    type Mod = ModIntF;
    let mut activated = v.copy_iter().collect::<FxHashSet<_>>();
    let mut delta = Vec::gen(n, |i, _| {
        if activated.contains(&i) {
            x[i].len() + 1
        } else {
            0
        }
    });
    let mut ans = vec![Mod::zero(); n];
    let mut last_updated = vec![0; n];
    let mut new = v.clone();
    for i in 0..k {
        let mut next = Vec::new();
        for j in new.copy_iter() {
            for k in x[j].copy_iter() {
                if !activated.contains(&k) {
                    last_updated[k] = i + 1;
                    delta[k] = x[k].len() + 1;
                    next.push(k);
                    activated.insert(k);
                } else {
                    ans[k] += Mod::from_index(i + 1 - last_updated[k]) * Mod::from_index(delta[k]);
                    delta[k] += 1;
                    last_updated[k] = i + 1;
                }
            }
        }
        if next.is_empty() {
            break;
        }
        new = next;
    }
    for i in 0..n {
        ans[i] += Mod::from_index(k - last_updated[i]) * Mod::from_index(delta[i]);
    }
    out.print_line(ans);
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
