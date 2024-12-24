//{"name":"Genetics","group":"Kattis","url":"https://open.kattis.com/problems/genetics2","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3 1\nACC\nCCA\nACA\nAAA\n","output":"3\n"},{"input":"4 4 3\nCATT\nCAAA\nATGA\nTCTA\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Genetics"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{random, Shuffle};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_unsigned();
    let data = Vec::gen(n, |_, _| {
        let s = input.read_str();
        let mut v = vec![0i64; m.div_ceil(32)];
        for i in 0..m {
            match s[i] {
                b'A' => v[i >> 5].set_bit(2 * (i & 31)),
                b'C' => v[i >> 5].set_bit(2 * (i & 31) + 1),
                b'G' => {
                    v[i >> 5].set_bit(2 * (i & 31));
                    v[i >> 5].set_bit(2 * (i & 31) + 1);
                }
                _ => {}
            }
        }
        v
    });

    let mult = Vec::gen(n, |_, _| random().gen_range(0..1000000000i64));
    let sum = mult.copy_sum();
    let dist = Arr2d::gen(m, 4, |i, j| {
        let mut res = 0;
        for a in 0..n {
            let val = data[a][i >> 5] >> (2 * (i & 31)) & 3;
            if val as usize != j {
                res += mult[a];
            }
        }
        res
    });
    let mut good = Vec::new();
    for i in 0..n {
        let mut cur = 0;
        for j in 0..m {
            cur += dist[j][data[i][j >> 5] as usize >> (2 * (j & 31)) & 3];
        }
        if cur == k as i64 * (sum - mult[i]) {
            good.push(i);
        }
    }

    let mut even = 0i64;
    for i in 0..32 {
        even.set_bit(2 * i);
    }
    let mut checked = FxHashSet::default();
    let mut order = good.clone();
    order.shuffle();
    for i in order {
        let mut next = Vec::new();
        for j in good {
            if j == i || checked.contains(&(i, j)) {
                next.push(j);
                continue;
            }
            let mut delta = 0;
            for k in data[i].indices() {
                let diff = data[i][k] ^ data[j][k];
                let d1 = diff & even;
                let d2 = (diff >> 1) & even;
                delta += (d1 | d2).count_ones();
            }
            if delta == k {
                next.push(j);
                checked.insert((j, i));
            }
        }
        good = next;
        if good.len() == 1 {
            break;
        }
    }
    out.print_line(good.inc());
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
