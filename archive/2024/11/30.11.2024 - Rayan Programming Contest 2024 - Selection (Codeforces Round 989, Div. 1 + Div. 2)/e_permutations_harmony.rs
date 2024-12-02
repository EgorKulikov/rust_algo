//{"name":"E. Permutations Harmony","group":"Codeforces - Rayan Programming Contest 2024 - Selection (Codeforces Round 989, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2034/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 3\n4 2\n5 1\n","output":"YES\n1 2 3\n2 3 1\n3 1 2\nYES\n1 2 3 4\n4 3 2 1\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPermutationsHarmony"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    if n == 1 {
        if k == 1 {
            out.print_line(true);
            out.print_line(1);
            return;
        }
        out.print_line(false);
        return;
    }

    if n % 2 == 0 && k % 2 == 1 {
        out.print_line(false);
        return;
    }

    let mut forbidden = FxHashSet::default();
    let mut ans = Vec::with_capacity(k);
    if k % 2 == 1 {
        if k < 3 {
            out.print_line(false);
            return;
        }
        let base = (1..=n).collect_vec();
        let mut o1 = vec![0; n];
        let mut o2 = vec![0; n];
        let mut next = 1;
        for i in (0..n).rev().step_by(2) {
            o1[i] = next;
            next += 1;
        }
        for i in (0..n - 1).rev().step_by(2) {
            o1[i] = next;
            next += 1;
        }
        next = 1;
        for i in (0..n - 1).rev().step_by(2) {
            o2[i] = next;
            next += 1;
        }
        for i in (0..n).rev().step_by(2) {
            o2[i] = next;
            next += 1;
        }
        ans.push(base.clone());
        ans.push(o1.clone());
        ans.push(o2.clone());
        forbidden.insert(o1);
        forbidden.insert(o2);
        forbidden.insert(base);
        // let mut other = vec![0; n];
        // for i in n - 1..2 * n - 1 {
        //     for j in 0..n {
        //         other[j] = (base[j] + i) % n + 1;
        //     }
        //     ans.push(other.clone());
        //     forbidden.insert(other.clone());
        // }
    }

    let mut base = (1..=n).collect_vec();
    let mut other = vec![0; n];
    while ans.len() < k {
        if base[0] > n / 2 && (n % 2 == 0 || base[0] > n / 2 + 1 || base[1] > n / 2) {
            out.print_line(false);
            return;
        }
        for i in 0..n {
            other[i] = n + 1 - base[i];
        }
        if forbidden.contains(&other) || forbidden.contains(&base) {
            base.next_permutation();
            continue;
        }
        ans.push(base.clone());
        ans.push(other.clone());
        base.next_permutation();
    }
    out.print_line(true);
    out.print_per_line(&ans);
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
