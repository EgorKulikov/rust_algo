//{"name":"day_24","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_24"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::{BTreeMap, HashMap};

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use algo_lib::{scan, str_scan};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut ins = BTreeMap::new();
    let mut rep = HashMap::new();
    loop {
        let s = input.read_line();
        if s.is_empty() {
            break;
        }
        str_scan!(s, "@: @", name: Str, val: i32);
        // rep.insert(name.clone(), name.clone());
        ins.insert(name, val);
    }
    let mut ops = Vec::new();
    while !input.is_exhausted() {
        scan!(input, "@ @ @ -> @", a: Str, op: Str, b: Str, c: Str);
        ops.push((a, op, b, c));
    }

    let mut sorted_ops = Vec::new();
    // part 1
    {
        let mut done = BitSet::new(ops.len());
        loop {
            let mut updated = false;
            for i in ops.indices() {
                if done[i] {
                    continue;
                }
                let (a, op, b, c) = &ops[i];
                if ins.contains_key(a) && ins.contains_key(b) {
                    let res = match op.as_slice() {
                        b"AND" => ins[a] & ins[b],
                        b"OR" => ins[a] | ins[b],
                        b"XOR" => ins[a] ^ ins[b],
                        _ => unreachable!(),
                    };
                    ins.insert(c.clone(), res);
                    done.set(i);
                    updated = true;
                    if a[0] == b'x' || a[0] == b'y' {
                        let str = Str::from(format!("{}{}", op[0] as char, Str::from(&a[1..])));
                        rep.insert(c.clone(), str.clone());
                        println!("Added {}", str);
                    } else if op.as_slice() == b"AND" {
                        let r1 = &rep[a];
                        let r2 = &rep[b];
                        let (r1, r2) = (r1.max(r2), r1.min(r2));
                        let id2 = Str::from(&r1[1..]).parse::<usize>();
                        let id1 = Str::from(&r2[1..]).parse::<usize>();
                        if r1[0] == b'X' && (r2[0] == b'A' || r2[0] == b'F') && id1 + 1 == id2 {
                            rep.insert(c.clone(), Str::from(format!("D{}", id1)));
                        } else {
                            rep.insert(c.clone(), Str::from(format!("D{}", id1)));
                            println!(
                                "Bad arg AND: a = {} b = {} a_rep = {} b_rep = {}",
                                a, b, rep[a], rep[b]
                            );
                        }
                    } else if op.as_slice() == b"XOR" {
                        let r1 = &rep[a];
                        let r2 = &rep[b];
                        let (r1, r2) = (r1.max(r2), r1.min(r2));
                        let id2 = Str::from(&r1[1..]).parse::<usize>();
                        let id1 = Str::from(&r2[1..]).parse::<usize>();
                        if r1[0] == b'X' && (r2[0] == b'A' || r2[0] == b'F') && id1 + 1 == id2 {
                            rep.insert(c.clone(), Str::from(format!("E{}", id1)));
                        } else {
                            rep.insert(c.clone(), Str::from(format!("E{}", id1)));
                            println!(
                                "Bad arg XOR: a = {} b = {} a_rep = {} b_rep = {}",
                                a, b, rep[a], rep[b]
                            );
                            for (k, v) in rep.iter() {
                                if v.as_slice() == b"X16" {
                                    println!("Should be {}", k);
                                }
                            }
                        }
                        if c.as_slice() != format!("z{:02}", id2).as_bytes() {
                            println!("Wrong output, should be z{}, is {}", id2, c);
                        }
                    } else if op.as_slice() == b"OR" {
                        let r1 = &rep[a];
                        let r2 = &rep[b];
                        let (r1, r2) = (r1.max(r2), r1.min(r2));
                        let id2 = Str::from(&r1[1..]).parse::<usize>();
                        let id1 = Str::from(&r2[1..]).parse::<usize>();
                        if r1[0] == b'D' && r2[0] == b'A' && id2 + 1 == id1 {
                            rep.insert(c.clone(), Str::from(format!("F{}", id1)));
                        } else {
                            rep.insert(c.clone(), Str::from(format!("F{}", id1)));
                            println!(
                                "Bad arg OR: a = {} b = {} a_rep = {} b_rep = {}",
                                a, b, rep[a], rep[b]
                            );
                        }
                    }
                    sorted_ops.push((a.clone(), op.clone(), b.clone(), c.clone()));
                }
            }
            if !updated {
                break;
            }
        }
        let mut ans = 0;
        let mut by = 1;
        for (k, v) in ins.iter() {
            if k.starts_with(b"z") {
                ans += (*v as i64) * by;
                by *= 2;
            }
        }
        out.print_line(ans);

        // let mut vals = rep.values().collect::<Vec<_>>();
        // vals.sort_by_key(|v| v.len());
        // for v in vals {
        //     println!("{}", v);
        // }
    }

    // part 2
    {}
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
