//{"name":"No Palindromes","group":"CodeChef - START169A","url":"https://www.codechef.com/START169A/problems/P6169","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\nabc\n2\naa\n4\nbbaa\n","output":"0\n-1\n2\nba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::qty::StrQty;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let q = s.qty_lower();
    let num_odd = q.copy_filter(|x| *x % 2 == 1).count();
    if num_odd >= 2 {
        out.print_line(0);
        return;
    }
    if q[s[0] as usize - 'a' as usize] == n {
        out.print_line(-1);
        return;
    }
    let mut state = 0i32;
    for i in 0..26 {
        if q[i] % 2 == 1 {
            state.set_bit(i);
        }
    }
    let by_index = by_index(&s);
    let mut at = 0;
    let mut ans = Str::new();
    loop {
        let mut ended = false;
        for c in b'a'..=b'z' {
            let pos = by_index[c].lower_bound(&at);
            if pos == by_index[c].len() {
                continue;
            }
            let id = (c - b'a') as usize;
            if (state ^ (1 << id)).count_ones() > 1 {
                ans.push(c);
                ended = true;
                break;
            }
            let i = by_index[c][pos];
            state ^= 1 << id;
            let mut need = 2 - state.count_ones();
            for d in b'a'..=b'z' {
                let id2 = (d - b'a') as usize;
                if state.is_set(id2) {
                    continue;
                }
                let pos2 = by_index[d].lower_bound(&(i + 1));
                if pos2 != by_index[d].len() {
                    need -= 1;
                    if need == 0 {
                        break;
                    }
                }
            }
            if need == 0 {
                ans.push(c);
                at = i + 1;
                break;
            } else {
                state ^= 1 << id;
            }
        }
        if ended {
            break;
        }
    }
    out.print_line(ans.len());
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
