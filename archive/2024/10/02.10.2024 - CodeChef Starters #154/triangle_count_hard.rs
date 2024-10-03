//{"name":"Triangle Count (Hard)","group":"CodeChef - START154A","url":"https://www.codechef.com/START154A/problems/TRICOUNT2","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n5 2 4 12\n6\n3 1 6 5 50 17\n3\n100 2 69\n","output":"3 7 15\n1 6 9 20 53\n3 137\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TriangleCountHard"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Default, Debug)]
    struct Segments {
        segments: BTreeSet<(usize, usize)>,
        len: usize,
    }

    impl Segments {
        fn add(&mut self, mut l: usize, mut r: usize) {
            if let Some(&(a, b)) = self.segments.floor(&(l, usize::MAX)) {
                if b >= l {
                    self.segments.remove(&(a, b));
                    self.len -= b - a + 1;
                    l.minim(a);
                    r.maxim(b);
                }
            }
            while let Some(&(a, b)) = self.segments.ceil(&(l, l)) {
                if a > r {
                    break;
                }
                self.segments.remove(&(a, b));
                self.len -= b - a + 1;
                r.maxim(b);
            }
            self.segments.insert((l, r));
            self.len += r - l + 1;
        }
    }

    let mut segments = Segments::default();
    let mut set = BTreeSet::new();
    let mut ans = Vec::with_capacity(n);
    for x in a {
        if let Some(&y) = set.floor(&x) {
            segments.add(x - y + 1, x + y - 1);
        }
        if let Some(&y) = set.ceil(&x) {
            segments.add(y - x + 1, x + y - 1);
        }
        // eprintln!("{x} {:?}", segments);
        set.insert(x);
        ans.push(segments.len);
    }
    out.print_line_iter(ans.into_iter().skip(1));
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
