//{"name":"A - Non-Crossing Railcar Rearrangement","group":"AtCoder - JR WEST and ALGO ARTIS Programming Contest  (AtCoder Heuristic Contest 064)","url":"https://atcoder.jp/contests/ahc064/tasks/ahc064_a","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::io::output::BoolOutput;

type PreCalc = ();

const R: usize = 10;
type Move = (u8, usize, usize, usize);

fn apply_turn(moves: &[Move], main: &mut [Vec<usize>], siding: &mut [Vec<usize>]) {
    // Each main and each siding is used at most once per turn (the contest's
    // non-crossing constraint), so sequential simulation gives the same
    // result as simultaneous application.
    for &(mt, i, j, k) in moves {
        if mt == 0 {
            let start = main[i].len() - k;
            let chunk: Vec<usize> = main[i].drain(start..).collect();
            for (idx, &c) in chunk.iter().enumerate() {
                siding[j].insert(idx, c);
            }
        } else {
            let chunk: Vec<usize> = siding[j].drain(0..k).collect();
            main[i].extend(chunk);
        }
    }
}

fn commit(moves: Vec<Move>, main: &mut [Vec<usize>], siding: &mut [Vec<usize>], turns: &mut Vec<Vec<Move>>) {
    if !moves.is_empty() {
        apply_turn(&moves, main, siding);
        turns.push(moves);
    }
}

fn locate(siding: &[Vec<usize>], car: usize) -> Option<(usize, usize)> {
    for s in 0..R {
        if let Some(p) = siding[s].iter().position(|&c| c == car) {
            return Some((s, p));
        }
    }
    None
}

/// Longest strictly increasing subsequence — returns indices.
fn lis(seq: &[usize]) -> Vec<usize> {
    let n = seq.len();
    if n == 0 {
        return vec![];
    }
    let mut len = vec![1usize; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];
    for i in 0..n {
        for j in 0..i {
            if seq[j] < seq[i] && len[j] + 1 > len[i] {
                len[i] = len[j] + 1;
                prev[i] = Some(j);
            }
        }
    }
    let mut best = 0;
    for i in 1..n {
        if len[i] > len[best] {
            best = i;
        }
    }
    let mut result = vec![];
    let mut idx = Some(best);
    while let Some(i) = idx {
        result.push(i);
        idx = prev[i];
    }
    result.reverse();
    result
}

const MAIN_CAPACITY: usize = 15;

struct Plan {
    chosen_targets: Vec<usize>,
    step1: Vec<Move>,
    step2_batches: Vec<Vec<Move>>,
    step3: Vec<Move>,
    /// (scratch_main, K) — extra cars placed on each scratch main when the
    /// obstruction chunk's leading prefix happens to match that main's next
    /// target sequence. Step 3 already accounts for this (its k is reduced).
    bonus_placements: Vec<(usize, usize)>,
}

/// Compute (but do not apply) the move-lists for one 3-step iteration on
/// `targets_to_attempt`. Drops candidates that share a source siding or whose
/// obstruction won't fit on any remaining scratch main.
fn plan_iteration(
    targets_to_attempt: &[usize],
    scratch: &[usize],
    next_id: &[usize],
    main: &[Vec<usize>],
    siding: &[Vec<usize>],
) -> Plan {
    let mut chosen: Vec<(usize, usize, usize, usize)> = Vec::new(); // (target, car_id, s, p)
    let mut sidings_used = std::collections::HashSet::new();
    for &t in targets_to_attempt {
        if next_id[t] >= R {
            continue;
        }
        let car_id = t * R + next_id[t];
        let Some((s, p)) = locate(siding, car_id) else {
            continue; // car not on a siding (e.g., still in a deferred obstruction chunk)
        };
        if sidings_used.insert(s) {
            chosen.push((t, car_id, s, p));
        }
    }

    let mut obstructed: Vec<(usize, usize, usize)> = chosen
        .iter()
        .filter(|c| c.3 > 0)
        .map(|c| (c.0, c.2, c.3))
        .collect();
    obstructed.sort_by_key(|c| c.1);

    // Greedy non-crossing scratch assignment with capacity check.
    let mut scratch_iter = scratch.iter().copied().peekable();
    let mut scratch_for: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();
    let mut step1: Vec<Move> = Vec::new();
    let mut failed: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for &(t, s, p) in &obstructed {
        let mut assigned = None;
        while let Some(&sm) = scratch_iter.peek() {
            scratch_iter.next();
            if main[sm].len() + p <= MAIN_CAPACITY {
                assigned = Some(sm);
                break;
            }
        }
        match assigned {
            Some(sm) => {
                scratch_for.insert(t, sm);
                step1.push((1, sm, s, p));
            }
            None => {
                failed.insert(t);
            }
        }
    }
    chosen.retain(|c| !failed.contains(&c.0));
    obstructed.retain(|c| !failed.contains(&c.0));

    // Step 2 batches: LIS of source sidings (sorted by target ascending).
    let mut step2_batches: Vec<Vec<Move>> = Vec::new();
    let mut step2_remaining: Vec<(usize, usize)> = chosen.iter().map(|c| (c.0, c.2)).collect();
    step2_remaining.sort_by_key(|&(t, _)| t);
    while !step2_remaining.is_empty() {
        let s_seq: Vec<usize> = step2_remaining.iter().map(|&(_, s)| s).collect();
        let take_set: std::collections::HashSet<usize> = lis(&s_seq)
            .into_iter()
            .map(|i| step2_remaining[i].0)
            .collect();
        let mut batch: Vec<Move> = Vec::new();
        let mut new_rem: Vec<(usize, usize)> = Vec::new();
        for &(t, s) in &step2_remaining {
            if take_set.contains(&t) {
                batch.push((1, t, s, 1));
            } else {
                new_rem.push((t, s));
            }
        }
        if !batch.is_empty() {
            step2_batches.push(batch);
        }
        step2_remaining = new_rem;
    }

    // For each obstructed target, the chunk [siding[s][0..p]] is appended
    // onto scratch main M in step 1. If a leading prefix of the chunk happens
    // to match M's next-target sequence (cars 10M+next_id[M], 10M+next_id[M]+1,
    // ...), those cars are correctly placed and don't need to be returned.
    // We track each main's bonus count and pretend next_id[M] grew by K.
    // To keep computations consistent across multiple obstructions hitting
    // distinct scratches (which they always do), we use the original next_id.
    let mut step3: Vec<Move> = Vec::new();
    let mut bonus_placements: Vec<(usize, usize)> = Vec::new();
    for &(t, s, p) in &obstructed {
        let m = *scratch_for.get(&t).unwrap();
        let mut keep = 0usize;
        let base_id = m * R + next_id[m];
        while keep < p && siding[s][keep] == base_id + keep {
            keep += 1;
        }
        let return_count = p - keep;
        if return_count > 0 {
            step3.push((0u8, m, s, return_count));
        }
        if keep > 0 {
            bonus_placements.push((m, keep));
        }
    }

    // Weave opportunistic placements: a "ready" car is at the front of a free
    // siding (siding not used by any chosen target's source) and is the next
    // car for some main M that is neither in the chosen target set nor used
    // as a scratch this iteration. We try to slip its placement (1, M, s, 1)
    // into step 1, any step-2 batch, or step 3, taking the first that
    // accepts it without breaking non-crossing.
    let target_set: std::collections::HashSet<usize> = chosen.iter().map(|c| c.0).collect();
    let used_scratches: std::collections::HashSet<usize> = scratch_for.values().copied().collect();
    let used_sidings: std::collections::HashSet<usize> = chosen.iter().map(|c| c.2).collect();
    let mut blocked_mains = target_set.clone();
    blocked_mains.extend(used_scratches);

    for s in 0..R {
        if used_sidings.contains(&s) {
            continue;
        }
        if siding[s].is_empty() {
            continue;
        }
        let car = siding[s][0];
        let m = car / R;
        if blocked_mains.contains(&m) {
            continue;
        }
        if next_id[m] != car % R {
            continue;
        }
        let new_move: Move = (1u8, m, s, 1);
        let mut placed = false;
        if try_add_to_turn(&mut step1, new_move) {
            placed = true;
        }
        if !placed {
            for batch in step2_batches.iter_mut() {
                if try_add_to_turn(batch, new_move) {
                    placed = true;
                    break;
                }
            }
        }
        if !placed && try_add_to_turn(&mut step3, new_move) {
            placed = true;
        }
        if placed {
            bonus_placements.push((m, 1));
        }
    }

    Plan {
        chosen_targets: chosen.iter().map(|c| c.0).collect(),
        step1,
        step2_batches,
        step3,
        bonus_placements,
    }
}

fn is_valid_turn(moves: &[Move]) -> bool {
    let mut mains = std::collections::HashSet::new();
    let mut sidings = std::collections::HashSet::new();
    for &(_, i, j, _) in moves {
        if !mains.insert(i) || !sidings.insert(j) {
            return false;
        }
    }
    let mut sorted: Vec<Move> = moves.to_vec();
    sorted.sort_by_key(|&(_, i, _, _)| i);
    for w in sorted.windows(2) {
        if w[1].2 <= w[0].2 {
            return false;
        }
    }
    true
}

fn try_add_to_turn(turn: &mut Vec<Move>, new_move: Move) -> bool {
    let mut test = turn.clone();
    test.push(new_move);
    if is_valid_turn(&test) {
        turn.push(new_move);
        true
    } else {
        false
    }
}



/// Choose up to 5 targets for the next step. Constraint: each target's next
/// car must sit on a distinct source siding. Tiebreaker: minimize sum of
/// `next_id[t]` across selected.
fn select_targets(siding: &[Vec<usize>], next_id: &[usize]) -> Vec<usize> {
    enumerate_top_selections(siding, next_id, 1)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// Enumerate up to `x` best target selections at this state. Primary
/// criterion: minimum number of step-2 batches. Tiebreaker: minimum sum of
/// `next_id[t]` across selected. All selections are max-cardinality.
fn enumerate_top_selections(
    siding: &[Vec<usize>],
    next_id: &[usize],
    x: usize,
) -> Vec<Vec<usize>> {
    use std::collections::HashMap;
    let mut best_per_siding: HashMap<usize, (usize, usize)> = HashMap::new();
    for m in 0..R {
        if next_id[m] >= R {
            continue;
        }
        let car_id = m * R + next_id[m];
        let Some((s, _)) = locate(siding, car_id) else {
            continue;
        };
        let count = next_id[m];
        let entry = best_per_siding.entry(s).or_insert((m, count));
        if count < entry.1 {
            *entry = (m, count);
        }
    }
    let candidates: Vec<(usize, usize, usize)> = best_per_siding
        .iter()
        .map(|(&s, &(m, c))| (m, s, c))
        .collect();
    let d = candidates.len();
    if d == 0 {
        return vec![];
    }
    let k = d.min(5);
    let mut subsets: Vec<(Vec<usize>, usize, usize)> = Vec::new();
    for mask in 1u32..(1u32 << d) {
        if mask.count_ones() as usize != k {
            continue;
        }
        let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(k); // (target, siding)
        let mut sum = 0usize;
        for i in 0..d {
            if mask & (1 << i) != 0 {
                pairs.push((candidates[i].0, candidates[i].1));
                sum += candidates[i].2;
            }
        }
        pairs.sort_by_key(|&(t, _)| t);
        let s_seq: Vec<usize> = pairs.iter().map(|&(_, s)| s).collect();
        let batches = step2_batch_count(&s_seq);
        let targets: Vec<usize> = pairs.into_iter().map(|(t, _)| t).collect();
        subsets.push((targets, batches, sum));
    }
    subsets.sort_by_key(|s| (s.1, s.2));
    subsets.truncate(x);
    subsets.into_iter().map(|s| s.0).collect()
}

/// Minimum number of strictly-increasing pieces a sequence partitions into.
/// (Patience sorting; equals length of the longest strictly-decreasing
/// subsequence by Dilworth's theorem.)
fn step2_batch_count(seq: &[usize]) -> usize {
    let mut piles_top: Vec<usize> = Vec::new();
    for &s in seq {
        let mut placed = false;
        for top in piles_top.iter_mut() {
            if *top < s {
                *top = s;
                placed = true;
                break;
            }
        }
        if !placed {
            piles_top.push(s);
        }
    }
    piles_top.len()
}

#[derive(Clone)]
struct State {
    main: Vec<Vec<usize>>,
    siding: Vec<Vec<usize>>,
    next_id: Vec<usize>,
}

impl State {
    fn is_complete(&self) -> bool {
        self.next_id.iter().all(|&v| v >= R)
    }

    /// Find every siding whose front car is the next-needed car for some main
    /// (no obstruction handling required), and deliver as many as possible
    /// non-crossing in a single turn. Returns true if any delivery happened.
    fn try_direct_delivery(&mut self, turns: &mut Vec<Vec<Move>>) -> bool {
        let mut candidates: Vec<(usize, usize)> = Vec::new(); // (target_main, source_siding)
        for s in 0..R {
            if self.siding[s].is_empty() {
                continue;
            }
            let car = self.siding[s][0];
            let m = car / R;
            if self.next_id[m] != car % R {
                continue;
            }
            candidates.push((m, s));
        }
        if candidates.is_empty() {
            return false;
        }
        // Pick a max non-crossing subset: sort by main ascending, take longest
        // strictly-increasing sidings.
        candidates.sort_by_key(|&(m, _)| m);
        let s_seq: Vec<usize> = candidates.iter().map(|&(_, s)| s).collect();
        let pick: std::collections::HashSet<usize> =
            lis(&s_seq).into_iter().collect();
        let turn: Vec<Move> = candidates
            .iter()
            .enumerate()
            .filter_map(|(i, &(m, s))| if pick.contains(&i) { Some((1u8, m, s, 1)) } else { None })
            .collect();
        // Single placement: cheaper to let weave pick it up inside an
        // existing step-1/2/3 turn than to spend a whole turn here.
        if turn.len() < 2 {
            return false;
        }
        apply_turn(&turn, &mut self.main, &mut self.siding);
        for &(_, m, _, _) in &turn {
            self.next_id[m] += 1;
        }
        turns.push(turn);
        true
    }

    fn apply_step(&mut self, targets: &[usize], turns: &mut Vec<Vec<Move>>) {
        let target_set: std::collections::HashSet<usize> = targets.iter().copied().collect();
        let scratch: Vec<usize> = (0..R).filter(|m| !target_set.contains(m)).collect();
        let plan = plan_iteration(targets, &scratch, &self.next_id, &self.main, &self.siding);

        if !plan.step1.is_empty() {
            apply_turn(&plan.step1, &mut self.main, &mut self.siding);
            turns.push(plan.step1);
        }
        for batch in plan.step2_batches {
            apply_turn(&batch, &mut self.main, &mut self.siding);
            turns.push(batch);
        }
        if !plan.step3.is_empty() {
            apply_turn(&plan.step3, &mut self.main, &mut self.siding);
            turns.push(plan.step3);
        }

        for &t in &plan.chosen_targets {
            self.next_id[t] += 1;
        }
        for (m, k) in plan.bonus_placements {
            self.next_id[m] += k;
        }
    }

    fn run_to_end(&mut self, turns: &mut Vec<Vec<Move>>) {
        while !self.is_complete() {
            // Drain free deliveries first.
            while self.try_direct_delivery(turns) {}
            if self.is_complete() {
                break;
            }
            let targets = select_targets(&self.siding, &self.next_id);
            assert!(!targets.is_empty(), "stuck: no candidates");
            self.apply_step(&targets, turns);
        }
    }
}

/// Beam search over the per-step target-selection decision. At each step,
/// for each beam state, generate up to `beam_width` candidate selections;
/// for each, apply the step and run greedy to completion to score it.
/// Take the `beam_width` best post-step states (by completed turn-count)
/// as the next beam.
fn beam_search(initial: State, beam_width: usize) -> Vec<Vec<Move>> {
    // Baseline: pure greedy.
    let mut baseline_state = initial.clone();
    let mut best_path: Vec<Vec<Move>> = Vec::new();
    baseline_state.run_to_end(&mut best_path);

    let mut beam: Vec<(State, Vec<Vec<Move>>)> = vec![(initial, Vec::new())];

    while !beam.is_empty() {
        let mut next_candidates: Vec<(State, Vec<Vec<Move>>, usize)> = Vec::new();

        for (state, prefix) in &beam {
            // Direct delivery is deterministic — drain it once per state, before
            // branching on selection candidates.
            let mut delivered_state = state.clone();
            let mut delivered_prefix = prefix.clone();
            while delivered_state.try_direct_delivery(&mut delivered_prefix) {}

            if delivered_state.is_complete() {
                if delivered_prefix.len() < best_path.len() {
                    best_path = delivered_prefix;
                }
                continue;
            }

            let target_options = enumerate_top_selections(
                &delivered_state.siding,
                &delivered_state.next_id,
                beam_width,
            );
            for targets in target_options {
                let mut new_state = delivered_state.clone();
                let mut step_turns: Vec<Vec<Move>> = Vec::new();
                new_state.apply_step(&targets, &mut step_turns);

                let mut new_path = delivered_prefix.clone();
                new_path.extend(step_turns);

                if new_state.is_complete() {
                    if new_path.len() < best_path.len() {
                        best_path = new_path;
                    }
                    continue;
                }

                // Score this candidate by running greedy to completion.
                let mut completion_state = new_state.clone();
                let mut completion_turns: Vec<Vec<Move>> = Vec::new();
                completion_state.run_to_end(&mut completion_turns);
                let total = new_path.len() + completion_turns.len();

                if total < best_path.len() {
                    let mut full = new_path.clone();
                    full.extend(completion_turns);
                    best_path = full;
                }

                next_candidates.push((new_state, new_path, total));
            }
        }

        next_candidates.sort_by_key(|c| c.2);
        next_candidates.truncate(beam_width);
        beam = next_candidates.into_iter().map(|c| (c.0, c.1)).collect();
    }

    best_path
}

const BEAM_WIDTH: usize = 35;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let r: usize = input.read();
    assert_eq!(r, R);
    let mut main: Vec<Vec<usize>> = (0..R).map(|_| input.read_vec(R)).collect();
    let mut siding: Vec<Vec<usize>> = vec![Vec::new(); R];
    let mut turns: Vec<Vec<Move>> = Vec::new();

    // Prep: dump every main onto its same-indexed siding in a single turn.
    let prep: Vec<Move> = (0..R).map(|m| (0u8, m, m, R)).collect();
    commit(prep, &mut main, &mut siding, &mut turns);

    let initial = State {
        main,
        siding,
        next_id: vec![0; R],
    };
    let beam_turns = beam_search(initial, BEAM_WIDTH);
    turns.extend(beam_turns);

    out.print_line(turns.len());
    for turn in &turns {
        out.print_line(turn.len());
        for &(mt, i, j, k) in turn {
            out.print_line((mt as usize, i, j, k));
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}


#[cfg(feature = "local")]
mod scoring;
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
