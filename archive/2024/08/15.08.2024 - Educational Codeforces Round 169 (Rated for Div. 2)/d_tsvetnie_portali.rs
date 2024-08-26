//{"name":"D. Цветные порталы","group":"Codeforces - Educational Codeforces Round 169 (Rated for Div. 2)","url":"https://codeforces.com/contest/2004/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 5\nBR BR GY GR\n1 2\n3 1\n4 4\n1 4\n4 2\n2 1\nBG RY\n1 2\n","output":"1\n4\n0\n3\n2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTsvetniePortali"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let types = input.read_str_vec(n);

    let mut by_type = DefaultHashMap::<_, Vec<_>>::new();
    for i in 0..n {
        by_type[&types[i]].push(i);
    }

    fn intersect(a: &Str, b: &Str) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if a[i] == b[j] {
                    return true;
                }
            }
        }
        false
    }

    for _ in 0..q {
        let x = input.read_size() - 1;
        let y = input.read_size() - 1;

        let (x, y) = (x.min(y), x.max(y));
        if intersect(&types[x], &types[y]) {
            out.print_line(y - x);
            continue;
        }
        let mut ans = None;
        for (k, v) in by_type.iter() {
            if intersect(k, &types[x]) && intersect(k, &types[y]) {
                let pos = v.lower_bound(&x);
                if pos > 0 {
                    ans.minim(x + y - 2 * v[pos - 1]);
                }
                if pos < v.len() && v[pos] < y {
                    ans.minim(y - x);
                }
                let pos = v.lower_bound(&y);
                if pos < v.len() {
                    ans.minim(2 * v[pos] - x - y);
                }
            }
        }
        out.print_line(ans);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
