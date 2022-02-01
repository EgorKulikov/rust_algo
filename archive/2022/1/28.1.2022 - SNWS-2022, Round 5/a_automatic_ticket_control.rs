//{"name":"A. Automatic Ticket Control","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/A/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 9\n4\nsnws\n5\n","output":"snwy\nsnxd\nsnxm\nsnxv\nsnya\n"},{"input":"1083 600\n6\nzzzzyz\n10\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAutomaticTicketControl"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::string::string::Str;
use algo_lib::{dynamic_value, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read_int();
    let b = input.read_int();
    let _n = input.read_usize();
    let mut s: Str = input.read();
    let k = input.read_usize();

    dynamic_value!(ModVal, MV, i32);
    ModVal::set_val(b);
    type Mod = ModInt<i32, ModVal>;
    fn add_one(s: &mut Str, a: Mod) -> Option<Mod> {
        let mut pow = Mod::one();
        let mut res = Mod::zero();
        for c in s.iter_mut().rev() {
            if *c == b'z' {
                res -= pow * Mod::new(25);
                *c = b'a';
            } else {
                *c += 1;
                res += pow;
                return Some(res);
            }
            pow *= a;
        }
        None
    }
    let a = Mod::new(a);
    let mut hash = Mod::zero();
    for c in s.iter() {
        hash *= a;
        hash += Mod::new((c - b'a').into_i32());
    }
    let or_s = s.clone();
    let mut res = vec![Vec::new(); b.into_usize()];
    let mut at = 0;
    loop {
        res[hash.to_index()].push(at);
        if res[hash.to_index()].len() == k {
            break;
        }
        match add_one(&mut s, a) {
            None => {
                out_line!(-1);
                return;
            }
            Some(val) => {
                at += 1;
                hash += val;
            }
        }
    }
    s = or_s;
    let v = res[hash.to_index()].clone();
    at = 0;
    for i in v {
        while at < i {
            at += 1;
            add_one(&mut s, a);
        }
        out_line!(s);
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
