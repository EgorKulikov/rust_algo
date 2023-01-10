//{"name":"day19","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day19"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::{out_line, scan};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    // const TIME: usize = 24;
    const TIME: usize = 32;
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    struct State {
        time: usize,
        ore: usize,
        clay: usize,
        obsidian: usize,
        ore_robot: usize,
        clay_robot: usize,
        obsidian_robot: usize,
    }

    // let mut ans = 0;
    let mut ans = 1;

    while !input.is_exhausted() {
        scan!(
            input,
            "Blueprint @: Each ore robot costs @ ore. Each clay robot costs @ ore. Each obsidian robot costs @ ore and @ clay. Each geode robot costs @ ore and @ obsidian.",
            id: usize,
            ore_cost: usize,
            clay_cost: usize,
            obsidian_ore_cost: usize,
            obsidian_clay_cost: usize,
            geode_ore_cost: usize,
            geode_obsidian_cost: usize,
        );
        if id > 3 {
            input.skip_whitespace();
            continue;
        }
        let mut res: HashMap<_, usize> = HashMap::new();
        let mut rec = RecursiveFunction::new(|f, state: State| -> usize {
            match res.get(&state).copied() {
                Some(x) => x,
                None => {
                    res.insert(state, ans);
                    let mut ans = 0;
                    let time_until_ore = if state.ore >= ore_cost {
                        0
                    } else {
                        (ore_cost - state.ore + state.ore_robot - 1) / state.ore_robot
                    };
                    if state.time + time_until_ore + ore_cost + 8 <= TIME {
                        ans.maxim(f.call(State {
                            time: state.time + time_until_ore + 1,
                            ore: state.ore + state.ore_robot * (time_until_ore + 1) - ore_cost,
                            clay: state.clay + state.clay_robot * (time_until_ore + 1),
                            obsidian: state.obsidian + state.obsidian_robot * (time_until_ore + 1),
                            ore_robot: state.ore_robot + 1,
                            clay_robot: state.clay_robot,
                            obsidian_robot: state.obsidian_robot,
                        }));
                    }
                    let time_until_clay = if state.ore >= clay_cost {
                        0
                    } else {
                        (clay_cost - state.ore + state.ore_robot - 1) / state.ore_robot
                    };
                    if state.time + time_until_clay + 6 <= TIME {
                        ans.maxim(f.call(State {
                            time: state.time + time_until_clay + 1,
                            ore: state.ore + state.ore_robot * (time_until_clay + 1) - clay_cost,
                            clay: state.clay + state.clay_robot * (time_until_clay + 1),
                            obsidian: state.obsidian + state.obsidian_robot * (time_until_clay + 1),
                            ore_robot: state.ore_robot,
                            clay_robot: state.clay_robot + 1,
                            obsidian_robot: state.obsidian_robot,
                        }));
                    }
                    if state.clay_robot != 0 {
                        let time_until_obsidian = if state.ore >= obsidian_ore_cost {
                            0
                        } else {
                            (obsidian_ore_cost - state.ore + state.ore_robot - 1) / state.ore_robot
                        }
                        .max(if state.clay >= obsidian_clay_cost {
                            0
                        } else {
                            (obsidian_clay_cost - state.clay + state.clay_robot - 1)
                                / state.clay_robot
                        });
                        if state.time + time_until_obsidian + 4 <= TIME {
                            ans.maxim(f.call(State {
                                time: state.time + (time_until_obsidian + 1),
                                ore: state.ore + state.ore_robot * (time_until_obsidian + 1)
                                    - obsidian_ore_cost,
                                clay: state.clay + state.clay_robot * (time_until_obsidian + 1)
                                    - obsidian_clay_cost,
                                obsidian: state.obsidian
                                    + state.obsidian_robot * (time_until_obsidian + 1),
                                ore_robot: state.ore_robot,
                                clay_robot: state.clay_robot,
                                obsidian_robot: state.obsidian_robot + 1,
                            }));
                        }
                    }
                    if state.obsidian_robot != 0 {
                        let time_until_geode = if state.ore >= geode_ore_cost {
                            0
                        } else {
                            (geode_ore_cost - state.ore + state.ore_robot - 1) / state.ore_robot
                        }
                        .max(if state.obsidian >= geode_obsidian_cost {
                            0
                        } else {
                            (geode_obsidian_cost - state.obsidian + state.obsidian_robot - 1)
                                / state.obsidian_robot
                        });
                        if state.time + time_until_geode + 2 <= TIME {
                            ans.maxim(
                                f.call(State {
                                    time: state.time + (time_until_geode + 1),
                                    ore: state.ore + state.ore_robot * (time_until_geode + 1)
                                        - geode_ore_cost,
                                    clay: state.clay + state.clay_robot * (time_until_geode + 1),
                                    obsidian: state.obsidian
                                        + state.obsidian_robot * (time_until_geode + 1)
                                        - geode_obsidian_cost,
                                    ore_robot: state.ore_robot,
                                    clay_robot: state.clay_robot,
                                    obsidian_robot: state.obsidian_robot,
                                }) + TIME
                                    - state.time
                                    - time_until_geode
                                    - 1,
                            );
                        }
                    }
                    res.insert(state, ans);
                    ans
                }
            }
        });
        let result = rec.call(State {
            time: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
        });
        eprintln!("{}: {}", id, result);
        // ans += result * id;
        ans *= result;
        input.skip_whitespace();
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
