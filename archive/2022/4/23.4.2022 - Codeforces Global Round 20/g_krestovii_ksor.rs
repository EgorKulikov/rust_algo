//{"name":"G. Крестовый ксор","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n?10\n1??\n010\n","output":"1\n"},{"input":"2 3\n000\n001\n","output":"0\n"},{"input":"1 1\n?\n","output":"2\n"},{"input":"6 9\n1101011?0\n001101?00\n101000110\n001011010\n0101?01??\n00?1000?0\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GKrestoviiKsor"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let mut r = input.read_usize();
    let mut c = input.read_usize();
    let mut b = input.read_table::<char>(r, c);

    type Mod = ModIntF;

    if r % 2 == 0 && c % 2 == 0 {
        let q_marks = b.iter().filter(|&&it| it == '?').count();
        out_line!(Mod::new(2).power(q_marks));
        return;
    } else if r % 2 == 0 || c % 2 == 0 {
        if c % 2 == 0 {
            b = b.transpose();
            swap(&mut r, &mut c);
        }
        let mut ans = Mod::zero();
        for v in 0..2 {
            let mut q = 0;
            let mut good = true;
            for i in 0..r {
                let ones = b.row(i).filter(|&&it| it == '1').count();
                let qm = b.row(i).filter(|&&it| it == '?').count();
                if qm > 0 {
                    q += qm - 1;
                } else if ones % 2 != v {
                    good = false;
                    break;
                }
            }
            if good {
                ans += Mod::new(2).power(q);
            }
        }
        out_line!(ans);
        return;
    }
    let mut ans = Mod::zero();
    for v in 0..2 {
        let mut free = Vec::new();
        let mut dsu = DSU::new(r + c);
        let mut q = 0;
        for i in 0..r {
            for j in 0..c {
                if b[(i, j)] == '?' {
                    if dsu.join(i, r + j) {
                        free.push((i, j));
                    } else {
                        q += 1;
                    }
                }
            }
        }
        let m = free.len();
        let mut a = vec![vec![0u32; m / 32 + 1]; r + c];
        for (id, (i, j)) in free.into_iter().enumerate() {
            a[i][id >> 5] |= 1 << (id & 31);
            a[j + r][id >> 5] |= 1 << (id & 31);
        }
        for i in 0..r {
            let q = b.row(i).filter(|&&it| it == '1').count() % 2;
            a[i][m >> 5] |= (v ^ q).into_u32() << (m & 31);
        }
        for i in 0..c {
            let q = b.column(i).filter(|&&it| it == '1').count() % 2;
            a[i + r][m >> 5] |= (v ^ q).into_u32() << (m & 31);
        }
        let mut st = 0;
        let mut bad = false;
        for i in 0..=m {
            let mut at = None;
            for j in st..r + c {
                if a[j][i >> 5].is_set(i & 31) {
                    at = Some(j);
                    break;
                }
            }
            if let Some(at) = at {
                if i == m {
                    bad = true;
                    break;
                }
                a.swap(st, at);
                for j in st + 1..r + c {
                    if a[j][i >> 5].is_set(i & 31) {
                        for k in (i >> 5)..=(m >> 5) {
                            a[j][k] ^= a[st][k];
                        }
                    }
                }
                st += 1;
            } else {
                assert_eq!(i, m);
            }
        }
        if !bad {
            ans += Mod::new(2).power(q);
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
