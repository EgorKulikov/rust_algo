//{"name":"J. Social Media","group":"Universal Cup - The 3rd Universal Cup. Stage 16: Nanjing","url":"https://contest.ucup.ac/contest/1828/problem/9573","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4 12 7\n5 7 3 6\n3 6\n2 2\n1 4\n2 4\n1 3\n7 6\n4 1\n5 4\n1 1\n1 1\n2 1\n3 7\n2 7 6\n2 4\n1 2\n3 2\n2 5\n5 4\n2 6\n4 6\n2 6\n1 1 2\n1\n1 2\n2 1 2\n1 2\n1 2\n2 1 100\n24 11\n11 24\n","output":"9\n5\n1\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JSocialMedia"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let f = input.read_size_vec(n).dec();
    let edges = input.read_size_pair_vec(m).dec();

    let is_friend = BitSet::new(k).do_with(|is_friend| {
        for &i in &f {
            is_friend.set(i);
        }
    });
    let mut add = vec![0; k];
    let mut base = 0;
    let mut qty = DefaultHashMap::<_, usize>::new();
    for &(u, v) in &edges {
        if is_friend[u] && is_friend[v] {
            base += 1;
        } else if is_friend[u] || u == v {
            add[v] += 1;
        } else if is_friend[v] {
            add[u] += 1;
        }
        qty[(u, v)] += 1;
        qty[(v, u)] += 1;
    }
    if n == k {
        out.print_line(base);
        return;
    }
    let mut a_copy = add.clone();
    a_copy.sort();
    let mut ans = base + a_copy[Back(0)] + a_copy[Back(1)];
    for &(u, v) in &edges {
        if !is_friend[u] && !is_friend[v] && u != v {
            ans.maxim(base + add[u] + add[v] + qty[(u, v)]);
        }
    }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
