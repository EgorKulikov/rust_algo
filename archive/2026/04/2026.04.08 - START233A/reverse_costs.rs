//{"name":"Reverse Costs","group":"CodeChef - START233A","url":"https://www.codechef.com/START233A/problems/REVCOSTS","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 1 1\n3\n2 1 1\n4\n1 2 3 4\n5\n1 2 1 2 1\n","output":"4\n9\n59\n37\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::string::hash::{CompositeHash, SimpleHash, StringHash};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    let q = a.qty_bound(n);
    let mut total = Mod::new(1);
    let mut len = 0;
    for i in q {
        len += i;
        total *= c.c(len, i);
    }
    let mut ans = total * (n - 2);
    let hash = SimpleHash::new(&a);
    let rev_hash = SimpleHash::new(&a.reversed());
    let mut set = FxHashSet::default();
    set.insert(hash.hash(..));
    set.insert(rev_hash.hash(..));
    ans += set.len() + 1;
    for i in 1..n {
        let sh1 = hash.sub_hash(i..);
        let sh2 = hash.sub_hash(..i);
        let hash = CompositeHash::new(&sh1, &sh2);
        set.insert(hash.hash(..));
        let rh1 = rev_hash.sub_hash(i..);
        let rh2 = rev_hash.sub_hash(..i);
        let rev_hash = CompositeHash::new(&rh1, &rh2);
        set.insert(rev_hash.hash(..));
    }
    ans += set.len();
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
