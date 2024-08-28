//{"name":"coderun_452","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_452"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_table(n, m);
    let b = input.read_size_table(n, m);

    let k = *a.iter().max().unwrap();
    if k == n * m {
        if a == b {
            out.print_line(0);
        } else {
            out.print_line(-1);
        }
        return;
    }
    let mut a_pos = vec![(0, 0); k];
    let mut b_pos = vec![(0, 0); k];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] != 0 {
                a_pos[a[i][j] - 1] = (i, j);
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            if b[i][j] != 0 {
                b_pos[b[i][j] - 1] = (i, j);
            }
        }
    }
    let mut dsu = DSU::new(n);
    for i in 0..k {
        dsu.join(a_pos[i].0, b_pos[i].0);
    }
    let mut has_zero = BitSet::new(n);
    for i in 0..n {
        for j in 0..m {
            if a[(i, j)] == 0 {
                has_zero.set(dsu.get(i));
            }
        }
    }
    let mut ans = k;
    for i in 0..n {
        if dsu.get(i) == i && !has_zero[i] {
            ans += 1;
        }
    }
    for i in 0..n {
        let mut row = Vec::new();
        for j in 0..m {
            if a[(i, j)] != 0 && b_pos[a[(i, j)] - 1].0 == i {
                row.push(b_pos[a[(i, j)] - 1].1);
            }
        }
        let mut x = Vec::new();
        for j in row {
            let pos = x.lower_bound(&j);
            if pos == x.len() {
                x.push(j);
            } else {
                x[pos] = j;
            }
        }
        ans -= x.len();
        if x.len() == m {
            ans -= 1;
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
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
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
