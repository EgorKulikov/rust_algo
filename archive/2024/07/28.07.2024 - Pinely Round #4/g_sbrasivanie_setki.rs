//{"name":"G. Сбрасывание сетки","group":"Codeforces - Pinely Round 4 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1991/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n4 5 3 6\nHVVHHV\n","output":"1 1\n2 1\n1 1\n2 3\n3 3\n2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSbrasivanieSetki"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    if n == k {
        if m == k {
            let ans = vec![(1, 1); q];
            out.print_per_line(&ans);
            return;
        }
        let mut next = 0;
        let mut ans = Vec::new();
        for c in s {
            if c == b'H' {
                ans.push((next + 1, 1));
                next += 1;
                next %= n;
            } else {
                if m == k + 1 {
                    next = 0;
                }
                ans.push((1, k + 1));
            }
        }
        out.print_per_line(&ans);
        return;
    }
    if m == k {
        let mut next = 0;
        let mut ans = Vec::new();
        for c in s {
            if c == b'V' {
                ans.push((1, next + 1));
                next += 1;
                next %= m;
            } else {
                if n == k + 1 {
                    next = 0;
                }
                ans.push((k + 1, 1));
            }
        }
        out.print_per_line(&ans);
        return;
    }

    enum Annex {
        Empty(BitSet),
        Full(BitSet),
    }
    enum Core {
        Empty,
        Vertical(BitSet),
        Horizontal(BitSet),
    }
    let mut vertical_annex = Annex::Empty(BitSet::new(m - k));
    let mut horizontal_annex = Annex::Empty(BitSet::new(n - k));
    let mut core = Core::Empty;
    let mut ans = Vec::with_capacity(q);
    for c in s {
        if c == b'H' {
            if let Annex::Empty(bs) = &mut horizontal_annex {
                let mut found = false;
                for i in 0..n - k {
                    if !bs[i] {
                        ans.push((i + k + 1, 1));
                        bs.set(i);
                        found = true;
                        break;
                    }
                }
                assert!(found);
            } else if let Annex::Full(bs) = &mut vertical_annex {
                assert!(matches!(core, Core::Empty));
                let mut found = false;
                for i in 0..k {
                    if !bs[i] {
                        ans.push((i + 1, 1));
                        bs.set(i);
                        found = true;
                        break;
                    }
                }
                assert!(found);
            } else {
                assert!(!matches!(core, Core::Vertical(_)));
                if let Core::Empty = core {
                    core = Core::Horizontal(BitSet::new(k));
                }
                if let Core::Horizontal(bs) = &mut core {
                    let mut found = false;
                    for i in 0..k {
                        if !bs[i] {
                            ans.push((i + 1, 1));
                            bs.set(i);
                            found = true;
                            break;
                        }
                    }
                    assert!(found);
                }
            }
        } else {
            if let Annex::Empty(bs) = &mut vertical_annex {
                let mut found = false;
                for i in 0..m - k {
                    if !bs[i] {
                        ans.push((1, i + k + 1));
                        bs.set(i);
                        found = true;
                        break;
                    }
                }
                assert!(found);
            } else if let Annex::Full(bs) = &mut horizontal_annex {
                assert!(matches!(core, Core::Empty));
                let mut found = false;
                for i in 0..k {
                    if !bs[i] {
                        ans.push((1, i + 1));
                        bs.set(i);
                        found = true;
                        break;
                    }
                }
                assert!(found);
            } else {
                assert!(!matches!(core, Core::Horizontal(_)));
                if let Core::Empty = core {
                    core = Core::Vertical(BitSet::new(k));
                }
                if let Core::Vertical(bs) = &mut core {
                    let mut found = false;
                    for i in 0..k {
                        if !bs[i] {
                            ans.push((1, i + 1));
                            bs.set(i);
                            found = true;
                            break;
                        }
                    }
                    assert!(found);
                }
            }
        }
        if let Annex::Empty(bs) = &vertical_annex {
            if bs.count_ones() == m - k {
                if let Core::Horizontal(bs2) = &core {
                    vertical_annex = Annex::Full(bs2.clone());
                    core = Core::Empty;
                } else {
                    vertical_annex = Annex::Full(BitSet::new(k));
                    // assert!(matches!(core, Core::Empty));
                }
            }
        }
        if let Annex::Empty(bs) = &horizontal_annex {
            if bs.count_ones() == n - k {
                if let Core::Vertical(bs2) = &core {
                    horizontal_annex = Annex::Full(bs2.clone());
                    core = Core::Empty;
                } else {
                    horizontal_annex = Annex::Full(BitSet::new(k));
                    // assert!(matches!(core, Core::Empty));
                }
            }
        }
        if let Core::Horizontal(bs) = &core {
            if bs.count_ones() == k {
                if let Annex::Full(bs2) = &horizontal_annex {
                    if bs2.count_ones() == 0 {
                        core = Core::Empty;
                    } else {
                        // let mut nbs = BitSet::new(k);
                        // for i in 0..k {
                        //     if !bs2[i] {
                        //         nbs.set(i);
                        //     }
                        // }
                        core = Core::Vertical(bs2.clone());
                    }
                    horizontal_annex = Annex::Empty(BitSet::new(n - k));
                } else {
                    unreachable!();
                }
            }
        }
        if let Core::Vertical(bs) = &core {
            if bs.count_ones() == k {
                if let Annex::Full(bs2) = &vertical_annex {
                    if bs2.count_ones() == 0 {
                        core = Core::Empty;
                    } else {
                        // let mut nbs = BitSet::new(k);
                        // for i in 0..k {
                        //     if !bs2[i] {
                        //         nbs.set(i);
                        //     }
                        // }
                        core = Core::Horizontal(bs2.clone());
                    }
                    vertical_annex = Annex::Empty(BitSet::new(m - k));
                } else {
                    unreachable!();
                }
            }
        }
        if let Annex::Full(bs) = &vertical_annex {
            if bs.count_ones() == k {
                vertical_annex = Annex::Empty(BitSet::new(m - k));
            }
        }
        if let Annex::Full(bs) = &horizontal_annex {
            if bs.count_ones() == k {
                horizontal_annex = Annex::Empty(BitSet::new(n - k));
            }
        }
    }
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
