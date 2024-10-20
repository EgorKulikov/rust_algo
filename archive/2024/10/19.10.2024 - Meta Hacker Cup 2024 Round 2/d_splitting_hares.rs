//{"name":"D: Splitting Hares","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-2/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n5\n1 2 7 -1 8\n2 2 1 1 1\n5\n4 1 2 -1 -1\n2 1 2 1 1\n5\n-1 -1 3 -1 5\n1 1 1 5 5\n6\n2 3 1 6 4 5\n1 1 1 2 2 2\n5\n1 -1 10 11 12\n1 1 1 2 2\n7\n7 2 -1 10 16 19 21\n1 1 2 2 2 3 3\n","output":"Case #1: Yes\n1 2 7 6 8\nCase #2: No\nCase #3: Yes\n2 1 3 4 5\nCase #4: No\nCase #5: Yes\n1 9 10 11 12\nCase #6: Yes\n7 2 13 10 16 19 21\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"splitting_hares_.*input[.]txt"},"output":{"type":"file","fileName":"splitting_hares_output.txt","pattern":null},"languages":{"java":{"taskClass":"DSplittingHares"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut w = input.read_int_vec(n);
    let c = input.read_size_vec(n).dec();
    drop(input);

    out.set_bool_output(BoolOutput::YesNo);
    let mut done = BitSet::new(n);
    let mut colors = Vec::new();
    let mut big_colors = Vec::new();
    for i in 0..n {
        if done[c[i]] {
            continue;
        }
        let mut fix = Vec::new();
        let mut unknowns = 0;
        let mut total = 0;
        for j in i..n {
            if c[i] == c[j] {
                if w[j] == -1 {
                    unknowns += 1;
                } else {
                    fix.push(w[j]);
                }
                total += 1;
            }
        }
        if total > 3 {
            out.print_line((format!("Case #{}:", test_case), false));
            return;
        }
        fix.sort();
        if unknowns != total {
            colors.push((fix, c[i], unknowns));
        } else {
            big_colors.push((c[i], unknowns));
        }
        done.set(c[i]);
    }
    colors.sort();
    for i in 1..colors.len() {
        if colors[i - 1].0.backward()[0] > colors[i].0[0] {
            out.print_line((format!("Case #{}:", test_case), false));
            return;
        }
    }
    let mut sure = Vec::new();
    let mut variants = Vec::<Vec<Vec<i32>>>::new();
    let mut sum_sure = 0;
    let mut d = Vec::new();
    let mut slv = |v: &[i32]| -> i32 {
        if d.len() < v.len() + 1 {
            d.resize(v.len() + 1, 0);
        }
        d[0] = 0;
        d[1] = i32::MAX / 2;
        for i in 2..=v.len() {
            d[i] = d[i - 2] + v[i - 1] - v[i - 2];
            if i >= 3 {
                let cand = d[i - 3] + v[i - 1] - v[i - 3];
                d[i].minim(cand);
            }
        }
        d[v.len()]
    };
    for i in colors.indices() {
        let (fix, _, unknowns) = colors[i].clone();
        let mut cur_variants = Vec::new();
        if unknowns == 0 {
            cur_variants.push(fix.clone());
        } else if fix.len() == 2 {
            if fix[0] + 1 == fix[1] {
                cur_variants.push(vec![fix[0] - 1, fix[0], fix[1]]);
                cur_variants.push(vec![fix[0], fix[1], fix[1] + 1]);
            } else {
                for i in fix[0] + 1..fix[1] {
                    cur_variants.push(vec![fix[0], i, fix[1]]);
                }
            }
        } else {
            if unknowns == 2 {
                cur_variants.push(vec![fix[0] - 2, fix[0] - 1, fix[0]]);
                cur_variants.push(vec![fix[0] - 1, fix[0], fix[0] + 1]);
                cur_variants.push(vec![fix[0], fix[0] + 1, fix[0] + 2]);
            } else {
                cur_variants.push(vec![fix[0] - 1, fix[0]]);
                cur_variants.push(vec![fix[0], fix[0] + 1]);
            }
        }
        if variants.is_empty() {
            for variant in &cur_variants {
                if variant[0] >= 1 {
                    variants.push(vec![variant.clone()]);
                }
            }
            continue;
        }
        let mut new_variants = Vec::new();
        for variant in &variants {
            for cur_variant in &cur_variants {
                if variant.backward()[0].backward()[0] >= cur_variant[0] {
                    continue;
                }
                let mut total = sum_sure + cur_variant.backward()[0] - cur_variant[0];
                for piece in variant {
                    total += piece.backward()[0] - piece[0];
                }
                let size = sure.len();
                for piece in variant {
                    for i in piece {
                        sure.push(*i);
                    }
                }
                for i in cur_variant {
                    sure.push(*i);
                }
                if total == slv(&sure) {
                    let mut var = variant.clone();
                    var.push(cur_variant.clone());
                    new_variants.push(var);
                }
                sure.resize(size, 0);
            }
        }
        if new_variants.is_empty() {
            out.print_line((format!("Case #{}:", test_case), false));
            return;
        }
        variants.clear();
        const SHIFT: usize = 3;
        if new_variants[0].len() <= SHIFT {
            swap(&mut variants, &mut new_variants);
            continue;
        }
        let mut best = None;
        for nv in &new_variants {
            let mut val = nv[0].clone();
            val.reverse();
            best.minim(val);
        }
        let mut best = best.unwrap();
        for i in best.iter().rev() {
            sure.push(*i);
        }
        best.reverse();
        sum_sure += best.backward()[0] - best[0];
        colors[i - SHIFT].0 = best.clone();
        colors[i - SHIFT].2 = 0;
        for mut nv in new_variants {
            if nv[0] == best {
                nv.remove(0);
                variants.push(nv);
            }
        }
    }
    if !variants.is_empty() {
        let v = variants[0].clone();
        for i in v.indices() {
            let len = colors.len();
            colors[len - v.len() + i].0 = v[i].clone();
            colors[len - v.len() + i].2 = 0;
        }
    }
    for i in colors.indices() {
        let (mut fix, color, _) = colors[i].clone();
        for j in 0..n {
            if c[j] == color && w[j] != -1 {
                for k in fix.indices() {
                    if w[j] == fix[k] {
                        fix.remove(k);
                        break;
                    }
                }
            }
        }
        for j in 0..n {
            if c[j] == color && w[j] == -1 {
                w[j] = fix.pop().unwrap();
            }
        }
    }
    let mut start = 1000;
    for (i, _) in big_colors {
        for j in 0..n {
            if c[j] == i {
                w[j] = start;
                start += 1;
            }
        }
        start += 30;
    }

    out.print_line((format!("Case #{}:", test_case), true));
    out.print_line(w);
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    output.flush();
    is_exhausted
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
