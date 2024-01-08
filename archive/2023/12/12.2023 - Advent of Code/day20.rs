//{"name":"day20","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day20"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::io::eol_string::EolString;
use algo_lib::io::input::Input;
use algo_lib::io::output::{err, Output};
use algo_lib::numbers::gcd::lcm;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::str_scan;
use algo_lib::string::str::{Str, StrReader};
use std::collections::{HashMap, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let lines = input.read_lines();

    let mut mods = Vec::new();
    for line in lines {
        let (tp, inp) = match line[0] {
            b'%' => (1, Str::from(&line[1..])),
            b'&' => (2, Str::from(&line[1..])),
            _ => (0, line),
        };
        str_scan!(inp, "@ -> @", from: Str<'static>, to: EolString);
        mods.push((
            tp,
            from,
            to.split(b", ")
                .into_iter()
                .map(|s| s.into_owned())
                .collect_vec(),
        ));
    }

    {
        // part 1
        let mut state = Vec::new();
        let mut incoming = Vec::new();
        let mut id = HashMap::new();
        for m in &mods {
            id.insert(m.1.clone(), state.len());
            incoming.push(Vec::new());
            if m.0 == 2 {
                for i in mods.indices() {
                    for o in &mods[i].2 {
                        if o == &m.1 {
                            incoming[state.len()].push(i);
                            break;
                        }
                    }
                }
            }
            state.push(0u64);
        }
        for m in &mods {
            for to in &m.2 {
                if !id.contains_key(to) {
                    id.insert(to.clone(), mods.len());
                }
            }
        }
        let mut low_sent = 0i64;
        let mut high_sent = 0i64;
        for _ in 0..1000 {
            let mut queue = VecDeque::new();
            queue.push_back((id[&Str::from(b"broadcaster")], false, 0));
            while let Some((m, is_high, from)) = queue.pop_front() {
                if is_high {
                    high_sent += 1;
                } else {
                    low_sent += 1;
                }
                if m == mods.len() {
                    continue;
                }
                match mods[m].0 {
                    0 => {
                        for to in &mods[m].2 {
                            queue.push_back((id[to], is_high, m));
                        }
                    }
                    1 => {
                        if !is_high {
                            let send_high = state[m] == 0;
                            state[m] ^= 1;
                            for to in &mods[m].2 {
                                queue.push_back((id[to], send_high, m));
                            }
                        }
                    }
                    2 => {
                        let pos = incoming[m].iter().find_eq(&from).unwrap();
                        if is_high {
                            state[m].set_bit(pos);
                        } else {
                            state[m].unset_bit(pos);
                        }
                        let send_high = state[m] != u64::all_bits(incoming[m].len());
                        for to in &mods[m].2 {
                            queue.push_back((id[to], send_high, m));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        out.print_line(low_sent * high_sent);
    }

    {
        // part 2
        mods.push((0, Str::from(b"rx"), Vec::new()));
        let mut state = Vec::new();
        let mut incoming = Vec::new();
        let mut id = HashMap::new();
        for m in &mods {
            id.insert(m.1.clone(), state.len());
            incoming.push(Vec::new());
            if m.0 != 1 {
                for i in mods.indices() {
                    for o in &mods[i].2 {
                        if o == &m.1 {
                            incoming[state.len()].push(i);
                            break;
                        }
                    }
                }
            }
            state.push(0u64);
        }
        for m in &mods {
            for to in &m.2 {
                if !id.contains_key(to) {
                    id.insert(to.clone(), mods.len());
                }
            }
        }
        let mut wh = vec![None; mods.len()];
        for i in 1i64..10000 {
            let mut queue = VecDeque::new();
            queue.push_back((id[&Str::from(b"broadcaster")], false, 0));
            let mut found = false;
            while let Some((m, is_high, from)) = queue.pop_front() {
                match mods[m].0 {
                    0 => {
                        for to in &mods[m].2 {
                            queue.push_back((id[to], is_high, m));
                        }
                    }
                    1 => {
                        if !is_high {
                            let send_high = state[m] == 0;
                            state[m] ^= 1;
                            for to in &mods[m].2 {
                                queue.push_back((id[to], send_high, m));
                            }
                        }
                    }
                    2 => {
                        let pos = incoming[m].iter().find_eq(&from).unwrap();
                        if is_high {
                            state[m].set_bit(pos);
                        } else {
                            state[m].unset_bit(pos);
                        }
                        let send_high = state[m] != u64::all_bits(incoming[m].len());
                        if send_high {
                            if wh[m].is_none() {
                                wh[m] = Some(i);
                            }
                        }
                        for to in &mods[m].2 {
                            queue.push_back((id[to], send_high, m));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        let in1 = incoming.rev()[0][0];
        let mut ans = 1;
        for &i in &incoming[in1] {
            ans = lcm(ans, wh[i].unwrap());
        }
        out.print_line(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
