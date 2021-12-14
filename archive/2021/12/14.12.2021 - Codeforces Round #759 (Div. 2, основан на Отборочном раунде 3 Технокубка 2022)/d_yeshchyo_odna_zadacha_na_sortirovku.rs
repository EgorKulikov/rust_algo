//{"name":"D. Ещё одна задача на сортировку","group":"Codeforces - Codeforces Round #759 (Div. 2, основан на Отборочном раунде 3 Технокубка 2022)","url":"https://codeforces.com/contest/1591/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n1\n2\n2 2\n2\n2 1\n3\n1 2 3\n3\n2 1 3\n3\n3 1 2\n4\n2 1 4 3\n","output":"YES\nYES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYeshchyoOdnaZadachaNaSortirovku"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<usize>(n).dec_by_one();

    let mut ft = FenwickTree::new(n);
    let mut inv = 0;
    for i in a {
        if ft.get(i, i + 1) != 0 {
            out_line!("YES");
            return;
        }
        inv += ft.get(i, n);
        ft.add(i, 1i64);
    }
    out_line!(if inv % 2 == 0 { "YES" } else { "NO" });
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
