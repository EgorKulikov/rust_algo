//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::{err, Output};
use algo_lib::misc::random::random;
use algo_lib::numbers::gauss::gauss;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    type Mod = ModInt7;

    let n = input.read_size();
    let a = input.read_table::<Mod>(n, n);
    let b = input.read_table::<Mod>(n, n);

    fn multiply<'a>(a: impl Iterator<Item = &'a Mod>, b: impl Iterator<Item = &'a Mod>) -> Mod {
        a.zip(b)
            .map(|(a, b)| *a * *b)
            .fold(Mod::zero(), |a, b| a + b)
    }

    let rand = random();
    let mut checked_rows = Vec::new();
    let mut cols = HashSet::new();
    const BUBEN: usize = 20;
    for _ in 0..BUBEN.min(n) {
        let mut c = Vec::with_capacity(n);
        for _ in 0..n {
            c.push(Mod::new(rand.next(1000) as i32));
        }
        let mut r = Vec::with_capacity(n);
        for i in 0..n {
            r.push(multiply(c.iter(), a.column(i)));
        }
        for i in 0..n {
            if multiply(r.iter(), b.column(i)) != c[i] {
                cols.insert(i);
            }
        }
        checked_rows.push((r, c));
    }
    let mut checked_cols = Vec::new();
    let mut rows = HashSet::new();
    for _ in 0..BUBEN.min(n) {
        let mut c = Vec::with_capacity(n);
        for _ in 0..n {
            c.push(Mod::new(rand.next(1000) as i32));
        }
        let mut r = Vec::with_capacity(n);
        for i in 0..n {
            r.push(multiply(c.iter(), a.row(i)));
        }
        for i in 0..n {
            if multiply(b.row(i), r.iter()) != c[i] {
                rows.insert(i);
            }
        }
        checked_cols.push((r, c));
    }

    let row_vec = rows.iter().copied().collect::<Vec<_>>().sorted();
    let col_vec = cols.iter().copied().collect::<Vec<_>>().sorted();
    // err().print_line(("Rows", rows.len()));
    // err().print_line(("Cols", cols.len()));
    let mut mat = Arr2d::new(
        checked_rows.len() * cols.len() + checked_cols.len() * rows.len(),
        rows.len() * cols.len() + 1,
        Mod::zero(),
    );
    let mut pos = 0;
    for (r, c) in &checked_rows {
        for (p, &j) in col_vec.iter().enumerate() {
            for k in 0..row_vec.len() {
                mat[(pos, k * col_vec.len() + p)] = r[row_vec[k]];
            }
            let mut target = c[j];
            for k in 0..n {
                if !rows.contains(&k) {
                    target -= r[k] * b[(k, j)];
                }
            }
            mat[(pos, rows.len() * col_vec.len())] = target;
            pos += 1;
        }
    }
    for (r, c) in &checked_cols {
        for (p, &j) in row_vec.iter().enumerate() {
            for k in 0..col_vec.len() {
                mat[(pos, p * col_vec.len() + k)] = r[col_vec[k]];
            }
            let mut target = c[j];
            for k in 0..n {
                if !cols.contains(&k) {
                    target -= b[(j, k)] * r[k];
                }
            }
            mat[(pos, rows.len() * col_vec.len())] = target;
            pos += 1;
        }
    }
    gauss(&mut mat);
    let mut ans = Vec::new();
    for i in 0..rows.len() * cols.len() {
        for j in 0..mat.d1() {
            if mat[(j, i)] != Mod::zero() {
                let res = mat[(j, rows.len() * cols.len())] / mat[(j, i)];
                if res != b[(row_vec[i / col_vec.len()], col_vec[i % col_vec.len()])] {
                    ans.push((
                        row_vec[i / col_vec.len()] + 1,
                        col_vec[i % col_vec.len()] + 1,
                        res,
                    ));
                }
                break;
            }
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
    stress_test::stress_test(run, tester::check);
}
//END MAIN
