#![allow(dead_code)]

use algo_lib::io::input::Input as AlgoInput;
use algo_lib::io::output::Output as AlgoOutput;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub const R: usize = 10;
pub const TMAX: usize = 4000;
pub const TOTAL_CARS: usize = 100;
pub const CARS_PER_LINE: usize = 10;
pub const MAIN_LINE_CAPACITY: usize = 15;
pub const SIDING_LINE_CAPACITY: usize = 20;

#[derive(Clone, Debug)]
pub struct Input {
    pub main_lines: Vec<Vec<usize>>,
}

impl Input {
    pub fn write(&self, out: &mut AlgoOutput) {
        out.print_line(R);
        for line in &self.main_lines {
            out.print_line_iter(line.iter().copied());
        }
        out.flush();
    }

    pub fn read(input: &mut AlgoInput) -> Self {
        let r: usize = input.read();
        assert_eq!(r, R);
        let mut main_lines = Vec::with_capacity(R);
        for _ in 0..R {
            let line: Vec<usize> = input.read_vec(CARS_PER_LINE);
            main_lines.push(line);
        }
        Self { main_lines }
    }
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut all_cars: Vec<usize> = (0..TOTAL_CARS).collect();
    all_cars.shuffle(&mut rng);
    let mut main_lines = Vec::with_capacity(R);
    for r in 0..R {
        let start = r * CARS_PER_LINE;
        main_lines.push(all_cars[start..start + CARS_PER_LINE].to_vec());
    }
    Input { main_lines }
}

// (move_type, i, j, k): 0 = main→siding, 1 = siding→main
pub type Move = (u8, usize, usize, usize);

pub fn parse_output(input: &Input, out: &mut AlgoInput) -> Result<Vec<Vec<Move>>, String> {
    let _ = input;
    let t = out
        .peek()
        .ok_or_else(|| "Empty output".to_string())
        .and_then(|_| Ok(out.read_size()))?;
    if t > TMAX {
        return Err(format!("Too many turns: {} > {}", t, TMAX));
    }
    let mut turns = Vec::with_capacity(t);
    for turn_idx in 0..t {
        let k = out.read_size();
        if k == 0 || k > R {
            return Err(format!(
                "Invalid move count for turn {}: {} (must satisfy 0 < K <= {})",
                turn_idx, k, R
            ));
        }
        let mut moves = Vec::with_capacity(k);
        for _ in 0..k {
            let mt: usize = out.read();
            let i: usize = out.read();
            let j: usize = out.read();
            let cnt: usize = out.read();
            if mt > 1 {
                return Err(format!("Invalid move_type: {}", mt));
            }
            if i >= R || j >= R {
                return Err(format!("Index out of range: i={}, j={}", i, j));
            }
            if cnt == 0 {
                return Err("Number of cars to move is 0".to_string());
            }
            moves.push((mt as u8, i, j, cnt));
        }
        turns.push(moves);
    }
    Ok(turns)
}

fn check_crossing(moves: &[Move]) -> Result<(), String> {
    use std::collections::HashSet;
    let mut used_main = HashSet::new();
    let mut used_siding = HashSet::new();
    for &(_, i, j, _) in moves {
        if !used_main.insert(i) {
            return Err(format!("Main line {} is used multiple times", i));
        }
        if !used_siding.insert(j) {
            return Err(format!("Siding line {} is used multiple times", j));
        }
    }
    let mut sorted: Vec<Move> = moves.to_vec();
    sorted.sort_by_key(|&(_, i, _, _)| i);
    for w in sorted.windows(2) {
        let (_, _, j1, _) = w[0];
        let (_, _, j2, _) = w[1];
        if j2 <= j1 {
            return Err(format!(
                "Crossing conflict: j={} → j={} is not strictly increasing",
                j1, j2
            ));
        }
    }
    Ok(())
}

fn apply_moves(
    main_lines: &mut [Vec<usize>],
    siding_lines: &mut [Vec<usize>],
    moves: &[Move],
    turn: usize,
) -> Result<(), String> {
    use std::collections::HashMap;
    let mut removed_from_main: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut removed_from_siding: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(mt, i, j, k) in moves {
        if mt == 0 {
            if main_lines[i].len() < k {
                return Err(format!(
                    "Turn {}: main line {} does not have {} cars (current: {} cars)",
                    turn,
                    i,
                    k,
                    main_lines[i].len()
                ));
            }
            let start = main_lines[i].len() - k;
            removed_from_main.insert(i, main_lines[i].drain(start..).collect());
        } else {
            if siding_lines[j].len() < k {
                return Err(format!(
                    "Turn {}: siding {} does not have {} cars (current: {} cars)",
                    turn,
                    j,
                    k,
                    siding_lines[j].len()
                ));
            }
            removed_from_siding.insert(j, siding_lines[j].drain(0..k).collect());
        }
    }
    for &(mt, i, j, _) in moves {
        if mt == 0 {
            if let Some(cars) = removed_from_main.get(&i) {
                for (idx, &car) in cars.iter().enumerate() {
                    siding_lines[j].insert(idx, car);
                }
            }
        } else if let Some(cars) = removed_from_siding.get(&j) {
            main_lines[i].extend(cars);
        }
    }
    Ok(())
}

fn check_goal(main_lines: &[Vec<usize>]) -> bool {
    for r in 0..R {
        if main_lines[r].len() != CARS_PER_LINE {
            return false;
        }
        for i in 0..CARS_PER_LINE {
            if main_lines[r][i] != 10 * r + i {
                return false;
            }
        }
    }
    true
}

/// Returns (score, error_message, turns_to_goal_if_reached).
pub fn compute_score(input: &Input, turns: &[Vec<Move>]) -> (i64, String, Option<usize>) {
    let mut main_lines: Vec<Vec<usize>> = input.main_lines.clone();
    let mut siding_lines: Vec<Vec<usize>> = vec![vec![]; R];

    for (turn_idx, moves) in turns.iter().enumerate() {
        let turn = turn_idx + 1;
        if let Err(e) = check_crossing(moves) {
            return (0, format!("Turn {}: {}", turn, e), None);
        }
        if let Err(e) = apply_moves(&mut main_lines, &mut siding_lines, moves, turn) {
            return (0, e, None);
        }
        for i in 0..R {
            if main_lines[i].len() > MAIN_LINE_CAPACITY {
                return (
                    0,
                    format!(
                        "Turn {}: main line {} exceeds capacity ({} > {})",
                        turn,
                        i,
                        main_lines[i].len(),
                        MAIN_LINE_CAPACITY
                    ),
                    None,
                );
            }
            if siding_lines[i].len() > SIDING_LINE_CAPACITY {
                return (
                    0,
                    format!(
                        "Turn {}: siding line {} exceeds capacity ({} > {})",
                        turn,
                        i,
                        siding_lines[i].len(),
                        SIDING_LINE_CAPACITY
                    ),
                    None,
                );
            }
        }
    }

    if check_goal(&main_lines) {
        return ((5000 - turns.len() as i64), String::new(), Some(turns.len()));
    }

    let mut score = 0i64;
    for r in 0..R {
        for (c, &car_id) in main_lines[r].iter().enumerate() {
            if car_id / 10 == r {
                score += 1;
                if car_id % 10 == c {
                    score += 9;
                }
            }
        }
    }
    (score, String::new(), None)
}
