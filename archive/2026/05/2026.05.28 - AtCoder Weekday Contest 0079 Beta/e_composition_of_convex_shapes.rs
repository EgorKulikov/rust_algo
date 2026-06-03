use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut pts = 0;
    let mut lines = DefaultHashMap::new(0);
    for _ in 0..n {
        let k = input.read_size();
        let p = input.read_long_pair_vec(k).sorted();
        if k == 1 {
            pts += 1;
        } else {
            let line = Point::new(p[0].0, p[0].1).line(Point::new(p[1].0, p[1].1));
            let mut is_line = true;
            for i in 2..k {
                if !line.contains(Point::new(p[i].0, p[i].1)) {
                    is_line = false;
                    break;
                }
            }
            if is_line {
                let dx = p[1].0 - p[0].0;
                let dy = p[1].1 - p[0].1;
                let g = gcd(dx, dy);
                lines[(dx / g, dy / g)] += 1;
            }
        }
    }

    type Mod = ModInt7;
    let mut ans = Mod::new(2).power(n) - Mod::new(2).power(pts);
    for c in lines.into_values() {
        ans -= Mod::new(2).power(c + pts) - Mod::new(2).power(pts);
    }
    out.print_line(ans);
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
