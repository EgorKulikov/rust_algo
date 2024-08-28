//{"name":"coderun_429","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_429"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let w = input.read_long();
    let h = input.read_long();
    let f = input.read_vec::<(i64, i64, i64)>(n);

    let mut d0 = Vec::new();
    let mut d1 = Vec::with_capacity(n);
    for i in 0..n {
        d1.push(f[i].1 - f[i].2);
    }
    d0.push(d1);
    let mut d2 = Vec::with_capacity(n);
    for i in 0..n {
        d2.push(w - (f[i].0 + f[i].2));
    }
    d0.push(d2);
    let mut d3 = Vec::with_capacity(n);
    for i in 0..n {
        d3.push(h - (f[i].1 + f[i].2));
    }
    d0.push(d3);
    let mut d4 = Vec::with_capacity(n);
    for i in 0..n {
        d4.push(f[i].0 - f[i].2);
    }
    d0.push(d4);
    let dist = Arr2d::generate(n, n, |i, j| {
        let (x1, y1, r1) = f[i];
        let (x2, y2, r2) = f[j];
        (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt() as i64 - r1 - r2
    });
    let mut edges = Vec::new();
    for k in 0..n {
        for l in 0..k {
            edges.push((dist[(k, l)], k, l));
        }
        for l in 0..4 {
            edges.push((d0[l][k], k, n + l));
        }
    }
    edges.sort();
    let mut d = Arr2d::new(4, 4, i64::MAX);
    let mut dsu = DSU::new(n + 4);
    for (dd, i, j) in edges {
        dsu.join(i, j);
        for k in 0..4 {
            for l in 0..k {
                if d[(k, l)] == i64::MAX && dsu.get(n + k) == dsu.get(n + l) {
                    d[(k, l)] = dd;
                    d[(l, k)] = dd;
                }
            }
        }
    }
    let dd = Arr2d::generate(4, 4, |i, j| {
        if i == j {
            return i64::MAX;
        }
        let mut res = i64::MAX;
        res.minim(d[(i, (i + 3) % 4)]);
        res.minim(d[(j, (j + 3) % 4)]);
        if (i + 1) % 4 == j {
            res.minim(d[(i, (i + 2) % 4)]);
        }
        if (j + 1) % 4 == i {
            res.minim(d[(j, (j + 2) % 4)]);
        }
        if (i + 2) % 4 == j {
            res.minim(d[(0, 2)]);
            res.minim(d[(1, 3)]);
        }
        res
    });

    for _ in 0..m {
        let r = input.read_long();
        let s = input.read_size() - 1;
        for i in 0..4 {
            if dd[(s, i)] >= 2 * r {
                out.print(i + 1);
            }
        }
        out.print_line(());
    }
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
