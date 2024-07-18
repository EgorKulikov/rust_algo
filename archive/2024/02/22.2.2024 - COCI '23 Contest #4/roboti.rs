//{"name":"#5 - Roboti","group":"DMOJ - COCI '23 Contest 4","url":"https://dmoj.ca/problem/coci23c4p5","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2 2\n1 1 L\n2 2 R\n5\n1 1 2 2\n2 1 1 2\n1 1 1 2\n2 1 1 1\n2 2 2 1\n","output":"-1\n1\n0\n0\n0\n"},{"input":"3 3 4\n1 1 L\n1 3 L\n2 1 L\n3 3 L\n7\n1 1 3 3\n3 3 2 1\n3 1 2 2\n2 3 1 2\n2 3 3 1\n1 2 3 2\n3 3 2 2\n","output":"1\n2\n1\n1\n1\n0\n3\n"},{"input":"4 4 8\n1 1 R\n1 3 L\n2 2 R\n2 4 L\n3 1 L\n3 3 L\n4 2 L\n4 4 L\n7\n1 2 1 4\n2 2 3 4\n4 4 3 2\n4 1 4 4\n4 2 3 1\n1 2 2 3\n2 4 2 3\n","output":"2\n1\n1\n0\n-1\n5\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Roboti"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let special = input.read_vec::<(usize, usize, char)>(k).dec();

    let mut is_free_row = BitSet::new(n);
    let mut is_free_col = BitSet::new(m);
    is_free_row.fill(true);
    is_free_col.fill(true);
    let mut by_row = vec![Vec::new(); n];
    let mut by_col = vec![Vec::new(); m];

    for (r, c, d) in special {
        is_free_row.unset(r);
        is_free_col.unset(c);
        by_row[r].push((c, d));
        by_col[c].push((r, d));
    }

    for i in 0..n {
        by_row[i].sort_unstable();
    }

    for i in 0..m {
        by_col[i].sort_unstable();
    }

    let mut cycle = HashMap::new();
    let mut length = Vec::new();

    let mut cycle_id = 0;
    for i in 0..n {
        if is_free_row[i] {
            continue;
        }
        for ((f, _), (t, _)) in by_row[i].iter().copied().zip(
            by_row[i]
                .iter()
                .skip(1)
                .chain(by_row[i].iter().take(1))
                .copied(),
        ) {
            let mut key = (true, i, f, t, true);
            if !cycle.contains_key(&key) {
                let start_key = key;
                let mut cycle_step = 0;
                loop {
                    cycle.insert(key, (cycle_id, cycle_step));
                    let (is_hor, was_line, _, will_line, is_dir) = key;
                    if is_hor {
                        let pos = by_col[will_line].lower_bound(&(was_line, 'A'));
                        let len = by_col[will_line].len();
                        if is_dir ^ (by_col[will_line][pos].1 == 'L') {
                            key = (
                                false,
                                will_line,
                                was_line,
                                by_col[will_line][(pos + 1) % len].0,
                                true,
                            );
                        } else {
                            key = (
                                false,
                                will_line,
                                was_line,
                                by_col[will_line][(pos + len - 1) % len].0,
                                false,
                            );
                        }
                    } else {
                        let pos = by_row[will_line].lower_bound(&(was_line, 'A'));
                        let len = by_row[will_line].len();
                        if is_dir ^ (by_row[will_line][pos].1 == 'R') {
                            key = (
                                true,
                                will_line,
                                was_line,
                                by_row[will_line][(pos + 1) % len].0,
                                true,
                            );
                        } else {
                            key = (
                                true,
                                will_line,
                                was_line,
                                by_row[will_line][(pos + len - 1) % len].0,
                                false,
                            );
                        }
                    }
                    cycle_step += 1;
                    if key == start_key {
                        break;
                    }
                }
                length.push(cycle_step);
                cycle_id += 1;
            }
            key = (true, i, t, f, false);
            if !cycle.contains_key(&key) {
                let mut cycle_step = 0;
                let start_key = key;
                loop {
                    cycle.insert(key, (cycle_id, cycle_step));
                    let (is_hor, was_line, _, will_line, is_dir) = key;
                    if is_hor {
                        let pos = by_col[will_line].lower_bound(&(was_line, 'A'));
                        let len = by_col[will_line].len();
                        if is_dir ^ (by_col[will_line][pos].1 == 'L') {
                            key = (
                                false,
                                will_line,
                                was_line,
                                by_col[will_line][(pos + 1) % len].0,
                                true,
                            );
                        } else {
                            key = (
                                false,
                                will_line,
                                was_line,
                                by_col[will_line][(pos + len - 1) % len].0,
                                false,
                            );
                        }
                    } else {
                        let pos = by_row[will_line].lower_bound(&(was_line, 'A'));
                        let len = by_row[will_line].len();
                        if is_dir ^ (by_row[will_line][pos].1 == 'R') {
                            key = (
                                true,
                                will_line,
                                was_line,
                                by_row[will_line][(pos + 1) % len].0,
                                true,
                            );
                        } else {
                            key = (
                                true,
                                will_line,
                                was_line,
                                by_row[will_line][(pos + len - 1) % len].0,
                                false,
                            );
                        }
                    }
                    cycle_step += 1;
                    if key == start_key {
                        break;
                    }
                }
                length.push(cycle_step);
                cycle_id += 1;
            }
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let start_r = input.read_size() - 1;
        let start_c = input.read_size() - 1;
        let end_r = input.read_size() - 1;
        let end_c = input.read_size() - 1;

        if start_r == end_r && is_free_row[start_r] {
            out.print_line(0);
            continue;
        }

        if start_c == end_c && is_free_col[start_c] {
            out.print_line(0);
            continue;
        }

        let get_keys = |r: usize, c: usize| -> Vec<(usize, usize)> {
            let mut res = Vec::new();
            if !is_free_row[r] {
                let len = by_row[r].len();
                let pos = by_row[r].lower_bound(&(c, 'A')) % len;
                if by_row[r][pos].0 == c {
                    res.push((true, r, c, by_row[r][(pos + 1) % len].0, true));
                    res.push((true, r, by_row[r][(pos + 1) % len].0, c, false));
                    res.push((true, r, c, by_row[r][(pos + len - 1) % len].0, false));
                    res.push((true, r, by_row[r][(pos + len - 1) % len].0, c, true));
                } else {
                    res.push((
                        true,
                        r,
                        by_row[r][(pos + len - 1) % len].0,
                        by_row[r][pos].0,
                        true,
                    ));
                    res.push((
                        true,
                        r,
                        by_row[r][pos].0,
                        by_row[r][(pos + len - 1) % len].0,
                        false,
                    ));
                }
            }
            if !is_free_col[c] {
                let len = by_col[c].len();
                let pos = by_col[c].lower_bound(&(r, 'A')) % len;
                if by_col[c][pos].0 == r {
                    res.push((false, c, r, by_col[c][(pos + 1) % len].0, true));
                    res.push((false, c, by_col[c][(pos + 1) % len].0, r, false));
                    res.push((false, c, r, by_col[c][(pos + len - 1) % len].0, false));
                    res.push((false, c, by_col[c][(pos + len - 1) % len].0, r, true));
                } else {
                    res.push((
                        false,
                        c,
                        by_col[c][(pos + len - 1) % len].0,
                        by_col[c][pos].0,
                        true,
                    ));
                    res.push((
                        false,
                        c,
                        by_col[c][pos].0,
                        by_col[c][(pos + len - 1) % len].0,
                        false,
                    ));
                }
            }
            res.into_iter()
                .map(|key| cycle.get(&key).copied().unwrap())
                .collect()
        };

        let start_keys = get_keys(start_r, start_c);
        let end_keys = get_keys(end_r, end_c);

        let mut ans = None;
        for &(id, pos) in &start_keys {
            for &(end_id, end_pos) in &end_keys {
                if id == end_id {
                    if end_pos >= pos {
                        ans.minim(end_pos - pos);
                    } else {
                        ans.minim(length[id] - pos + end_pos);
                    }
                }
            }
        }
        out.print_line(ans);
    }
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
    //    tester::stress_test();
}
//END MAIN
