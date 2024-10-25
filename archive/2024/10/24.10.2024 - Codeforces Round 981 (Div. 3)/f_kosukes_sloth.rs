//{"name":"F. Kosuke's Sloth","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2\n100 1\n1000000000000 1377\n","output":"9\n100\n999244007\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKosukesSloth"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut ids = Vec::new();
    let mut f1 = 1 % k;
    if f1 == 0 {
        ids.push(1);
    }
    let mut f2 = 1 % k;
    let mut cur = 2;
    while ids.len() < 2 {
        if f2 == 0 {
            ids.push(cur);
        }
        (f1, f2) = (f2, (f1 + f2) % k);
        cur += 1;
    }
    type Mod = ModInt7;
    out.print_line(Mod::new(ids[0]) + Mod::new(ids[1] - ids[0]) * Mod::new_from_wide(n as i64 - 1));
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        for i in 2..=1377 {
            print!("{} - ", i);
            let mut ids = Vec::new();
            let mut f1 = 1 % i;
            let mut f2 = 1 % i;
            let mut cur = 2;
            while ids.len() < 10 {
                if f2 == 0 {
                    ids.push(cur);
                }
                (f1, f2) = (f2, (f1 + f2) % i);
                cur += 1;
            }
            for j in 2..10 {
                if ids[j] - ids[j - 1] != ids[j - 1] - ids[j - 2] {
                    panic!();
                }
            }
            println!("{:?}", ids);
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
