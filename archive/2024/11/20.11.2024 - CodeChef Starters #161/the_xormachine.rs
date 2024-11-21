//{"name":"The XOR Machine","group":"CodeChef - START161A","url":"https://www.codechef.com/START161A/problems/FREQXOR","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 4 10 5\n7 10 17 3\n2999 4289 589238 428\n2378 53892 684997 4819\n1238 4859 918372 3579\n","output":"2\n3\n1368\n132\n256\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TheXORMachine"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_iterator::iterate_with_base;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_size();
    let l = input.read_size();
    let r = input.read_size();
    let m = input.read_size();

    let mut base = 0;
    let mut segs = Vec::new();
    for (_, len, prefix) in iterate_with_base(l, r, 2) {
        let from = (((x >> len) << len) ^ prefix) % m;
        let len = 1 << len;
        base += len / m;
        let len = len % m;
        if from + len <= m {
            segs.push((from, from + len));
        } else {
            segs.push((from, m));
            segs.push((0, from + len - m));
        }
    }
    let (mut a, mut b) = segs.detuple();
    a.sort();
    b.sort();
    let mut cur = 0;
    let mut ans = base;
    let mut j = 0;
    for i in a {
        while j < b.len() && b[j] <= i {
            cur -= 1;
            j += 1;
        }
        cur += 1;
        ans.maxim(base + cur);
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
