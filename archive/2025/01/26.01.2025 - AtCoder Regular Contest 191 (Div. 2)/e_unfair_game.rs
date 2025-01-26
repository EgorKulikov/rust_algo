//{"name":"E - Unfair Game","group":"AtCoder - AtCoder Regular Contest 191 (Div. 2)","url":"https://atcoder.jp/contests/arc191/tasks/arc191_e","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1 1\n1 0\n1 1\n","output":"2\n"},{"input":"2 2 1\n1 2\n1 2\n","output":"3\n"},{"input":"5 8 3\n0 0\n0 0\n0 0\n0 0\n0 0\n","output":"0\n"},{"input":"7 2025 191\n1323 9953\n2763 3225\n2624 5938\n6718 2998\n3741 7040\n9837 1681\n8817 4471\n","output":"40\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_int();
    let y = input.read_int();
    let ab = input.read_size_pair_vec(n);

    type Mod = ModIntF;
    if x % 2 == 1 && y % 2 == 1 {
        let mut win_bags = 0;
        let mut lose_bags = 0;
        for i in 0..n {
            let (_, b) = ab[i];
            if b % 2 == 1 {
                win_bags += 1;
            } else {
                lose_bags += 1;
            }
        }
        let mut ans = Mod::zero();
        let c = Combinations::<Mod>::new(win_bags + 1);
        for i in 0..=win_bags {
            let we = i;
            let them = win_bags - i;
            if we > them {
                ans += c.c(win_bags, i);
            }
        }
        out.print_line(ans * Mod::new(2).power(lose_bags));
    } else if x % 2 == 0 && y % 2 == 0 {
        let mut win_bags = 0;
        let mut lose_bags = 0;
        for i in 0..n {
            let (a, b) = ab[i];
            if (b + a) % 2 == 1 {
                win_bags += 1;
            } else {
                lose_bags += 1;
            }
        }
        let mut ans = Mod::zero();
        let c = Combinations::<Mod>::new(win_bags + 1);
        for i in 0..=win_bags {
            let we = i;
            let them = win_bags - i;
            if we > them {
                ans += c.c(win_bags, i);
            }
        }
        out.print_line(ans * Mod::new(2).power(lose_bags));
    } else if x % 2 == 0 {
        let mut auto_win_bags = 0;
        let mut win_bags = 0;
        let mut lose_bags = 0;
        for i in 0..n {
            let (a, b) = ab[i];
            if a >= 2 {
                auto_win_bags += 1;
            } else if a == 1 {
                if b % 2 == 1 {
                    win_bags += 1;
                } else {
                    auto_win_bags += 1;
                }
            } else {
                if b % 2 == 1 {
                    win_bags += 1;
                } else {
                    lose_bags += 1;
                }
            }
        }
        let mut ans = Mod::zero();
        let c = Combinations::<Mod>::new(n + 1);
        let mut add = Mod::zero();
        let mut vals = Vec::with_capacity(auto_win_bags + 1);
        for i in 0..=auto_win_bags {
            vals.push(c.c(auto_win_bags, i));
        }
        let p = vals.partial_sums();
        for i in 0..=win_bags {
            let we = auto_win_bags + i;
            let them = win_bags - i;
            if we > them {
                ans += c.c(win_bags, i);
                if i <= win_bags - i {
                    add += c.c(win_bags, i)
                        * Mod::new(2).power(lose_bags)
                        * p[((win_bags - i) - i + 1).min(auto_win_bags)];
                }
            }
        }
        out.print_line(ans * Mod::new(2).power(lose_bags + auto_win_bags) - add);
    } else {
        let mut auto_lose_bags = 0;
        let mut win_bags = 0;
        let mut lose_bags = 0;
        for i in 0..n {
            let (a, b) = ab[i];
            if a >= 2 {
                auto_lose_bags += 1;
            } else if a == 1 {
                if b % 2 == 1 {
                    win_bags += 1;
                } else {
                    auto_lose_bags += 1;
                }
            } else {
                if b % 2 == 1 {
                    win_bags += 1;
                } else {
                    lose_bags += 1;
                }
            }
        }
        let mut ans = Mod::zero();
        let c = Combinations::<Mod>::new(n + 1);
        let mut add = Mod::zero();
        let mut vals = Vec::with_capacity(auto_lose_bags + 1);
        for i in 0..=auto_lose_bags {
            vals.push(c.c(auto_lose_bags, i));
        }
        let p = vals.partial_sums();
        for i in 0..=win_bags {
            let we = i;
            let them = win_bags - i + auto_lose_bags;
            if we > them {
                ans += c.c(win_bags, i);
            } else if i > win_bags - i {
                add += Mod::new(2).power(lose_bags)
                    * c.c(win_bags, i)
                    * p[(i - (win_bags - i)).min(auto_lose_bags)];
            }
        }
        out.print_line(ans * Mod::new(2).power(lose_bags + auto_lose_bags) + add);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
