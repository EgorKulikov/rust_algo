//{"name":"Array Concatanation","group":"CodeChef - START155A","url":"https://www.codechef.com/START155A/problems/ARRCONCAT","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 2\n1 5\n1005 2005\n","output":"2\n2\n74974562\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ArrayConcatanation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_size();
    let b = input.read_size();

    let n = a + b;
    let plus = n / 4;
    let minus = n / 2 - n / 4;
    let zeroes = n - plus - minus;
    type Mod = ModInt7;
    let mut ans = Mod::zero();
    let c = Combinations::<Mod>::new(n + 1);
    for i in 0..=plus.min(b / 2) {
        ans += c.c(plus, i) * c.c(minus, i) * c.c(zeroes, b - 2 * i);
    }
    out.print_line(ans);
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut cur = Vec::new();
        let mut len = 0;
        for i in 1..=100 {
            cur.push(0);
            for j in 0..i {
                len += 1;
                if len % 2 == 0 {
                    cur[j] += 1;
                } else {
                    cur[j] -= 1;
                }
            }
            if i % 2 == 0 {
                println!("{} {:?}", i, cur);
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
