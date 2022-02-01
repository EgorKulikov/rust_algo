//{"name":"C. Concentration For Two","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/C/","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CConcentrationForTwo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut cards = vec![None; 2 * n];
    let mut times = vec![0; n];
    let mut at_first = 0;
    let mut at_second = 0;
    let mut num_first = 0;
    let mut num_second = 0;
    let mut second_skipped = Vec::new();
    for _ in 0..2 * n - 1 {
        let id = input.read_usize() - 1;
        match cards[id] {
            Some(i) => {
                out_line!(i);
            }
            None => {
                if num_first != n {
                    while times[at_first] == 1 {
                        at_first += 1;
                    }
                    assert_eq!(times[at_first], 0);
                    out_line!(at_first + 1);
                    cards[id] = Some(at_first + 1);
                    times[at_first] = 1;
                    at_first += 1;
                    num_first += 1;
                } else {
                    let res = if second_skipped.is_empty() {
                        while times[at_second] == 2 {
                            at_second += 1;
                        }
                        at_second += 1;
                        at_second - 1
                    } else {
                        second_skipped.pop().unwrap()
                    };
                    assert_eq!(times[res], 1);
                    out_line!(res + 1);
                    cards[id] = Some(res + 1);
                    times[res] = 2;
                    num_second += 1;
                }
            }
        }
        let first_res = cards[id].unwrap() - 1;
        let id = input.read_usize() - 1;
        match cards[id] {
            None => {
                if num_first != n && num_first <= num_second + 1 {
                    while times[at_first] == 1 {
                        at_first += 1;
                    }
                    assert_eq!(times[at_first], 0);
                    out_line!(at_first + 1);
                    cards[id] = Some(at_first + 1);
                    times[at_first] = 1;
                    at_first += 1;
                    num_first += 1;
                } else {
                    let mut res = None;
                    if num_second == n - 1 {
                        if second_skipped.is_empty() {
                            while times[at_second] == 2 {
                                at_second += 1;
                            }
                            res = Some(at_second);
                        } else {
                            res = Some(second_skipped[0]);
                            second_skipped.swap_remove(0);
                        }
                    }
                    if res.is_none() {
                        for (i, &v) in second_skipped.iter().enumerate() {
                            if v != first_res {
                                second_skipped.swap_remove(i);
                                res = Some(i);
                                break;
                            }
                        }
                    }
                    if res.is_none() {
                        while times[at_second] == 2 {
                            at_second += 1;
                        }
                        if at_second != first_res {
                            res = Some(at_second);
                            at_second += 1;
                        }
                    }
                    if res.is_none() {
                        let mut i = at_second + 1;
                        while times[i] == 2 {
                            i += 1;
                        }
                        res = Some(i);
                    }
                    let res = res.unwrap();
                    assert_eq!(times[res], 1);
                    out_line!(res + 1);
                    cards[id] = Some(res + 1);
                    times[res] = 2;
                    num_second += 1;
                }
            }
            Some(i) => {
                out_line!(i);
            }
        }
    }
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
