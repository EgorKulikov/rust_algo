//{"name":"D. QED's Favorite Permutation","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 3\n1 4 2 5 3\nRLRLL\n2\n4\n3\n8 5\n1 5 2 4 8 3 6 7\nRRLLRRRL\n4\n3\n5\n3\n4\n6 2\n1 2 3 4 5 6\nRLRLRL\n4\n5\n","output":"YES\nYES\nNO\nNO\nYES\nNO\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DQEDsFavoritePermutation"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let p = input.read_size_vec(n).dec();
    let mut s = input.read_str();

    let mut allowed = BitSet::new(n);
    let mut max = 0;
    for i in 0..n - 1 {
        max.maxim(p[i]);
        if i == max {
            allowed.set(i + 1);
        }
    }
    let mut bad = HashSet::new();
    for i in 1..n {
        if s[i - 1] == b'L' && s[i] == b'R' && !allowed[i] {
            bad.insert(i);
        }
    }
    for _ in 0..q {
        let pos = input.read_size() - 1;
        bad.remove(&pos);
        bad.remove(&(pos + 1));
        if s[pos] == b'L' {
            s[pos] = b'R';
        } else {
            s[pos] = b'L';
        }
        if s[pos - 1] == b'L' && s[pos] == b'R' && !allowed[pos] {
            bad.insert(pos);
        }
        if s[pos] == b'L' && s[pos + 1] == b'R' && !allowed[pos + 1] {
            bad.insert(pos + 1);
        }
        out.print_line(bad.is_empty());
    }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
