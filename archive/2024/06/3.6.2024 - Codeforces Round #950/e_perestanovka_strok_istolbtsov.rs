//{"name":"E. Перестановка строк и столбцов","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n1 1\n1\n1\n2 2\n1 2\n3 4\n4 3\n2 1\n2 2\n1 2\n3 4\n4 3\n1 2\n3 4\n1 5 9 6\n12 10 4 8\n7 11 3 2\n1 5 9 6\n12 10 4 8\n7 11 3 2\n3 3\n1 5 9\n6 4 2\n3 8 7\n9 5 1\n2 4 6\n7 8 3\n2 3\n1 2 6\n5 4 3\n6 1 2\n3 4 5\n1 5\n5 1 2 3 4\n4 2 5 1 3\n","output":"YES\nYES\nNO\nYES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPerestanovkaStrokIStolbtsov"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_size_table(n, m);
    let mut b = input.read_size_table(n, m);

    fn canonize(a: &mut Arr2d<usize>) {
        let n = a.d1();
        let m = a.d2();
        let mut r = n;
        let mut c = m;
        'outer: for i in 0..n {
            for j in 0..m {
                if a[(i, j)] == 1 {
                    r = i;
                    c = j;
                    break 'outer;
                }
            }
        }
        assert_ne!(r, n);
        a.swap_rows(0, r);
        let mut map = DefaultHashMap::new();
        for i in 0..n {
            for j in 0..m {
                map[(a[(i, c)], a[(0, j)])] = a[(i, j)];
            }
        }
        let top_row = a.row(0).copied().collect_vec().sorted();
        let left_col = a.column(c).copied().collect_vec().sorted();
        for i in 0..n {
            for j in 0..m {
                a[(i, j)] = map[(left_col[i], top_row[j])];
            }
        }
    }
    canonize(&mut a);
    canonize(&mut b);
    out.print_line(a == b);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
