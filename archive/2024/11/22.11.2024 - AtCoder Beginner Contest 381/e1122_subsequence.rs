//{"name":"E - 11/22 Subsequence","group":"AtCoder - AtCoder Beginner Contest 381","url":"https://atcoder.jp/contests/abc381/tasks/abc381_e","interactive":false,"timeLimit":3000,"tests":[{"input":"12 5\n111/212/1122\n1 7\n9 12\n3 6\n4 10\n1 12\n","output":"5\n0\n3\n1\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1122Subsequence"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    let slashes = s
        .iter()
        .enumerate()
        .filter(|&(_, c)| c == b'/')
        .map(|(i, _)| i)
        .collect_vec();

    let mut ones = vec![0];
    let mut twos = vec![0];
    for i in 0..n {
        let add_one = if s[i] == b'1' { 1 } else { 0 };
        let add_two = if s[i] == b'2' { 1 } else { 0 };
        ones.push(ones[Back(0)] + add_one);
        twos.push(twos[Back(0)] + add_two);
    }

    for _ in 0..q {
        // eprintln!("q");
        let l = input.read_size() - 1;
        let r = input.read_size() - 1;

        let from = slashes.lower_bound(&l);
        let to = slashes.upper_bound(&r);
        if from == to {
            out.print_line(0);
            continue;
        }
        let mut left = from;
        let mut right = to;
        while right > left {
            let mid = (left + right) / 2;
            let p = slashes[mid];
            // eprintln!("{} {} {}", p, ones[p] - ones[l], twos[r + 1] - twos[p]);
            if twos[r + 1] - twos[p] > ones[p] - ones[l] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let mut ans = 0;
        if left > from {
            let p = slashes[left - 1];
            let v = (ones[p] - ones[l]).min(twos[r + 1] - twos[p]);
            // eprintln!("{} {}", p, v);
            ans = ans.max(2 * v + 1);
        }
        if left < to {
            let p = slashes[left];
            let v = (ones[p] - ones[l]).min(twos[r + 1] - twos[p]);
            // eprintln!("{} {}", p, v);
            ans = ans.max(2 * v + 1);
        }
        out.print_line(ans);
    }
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
