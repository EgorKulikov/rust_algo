//{"name":"day22","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day22"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::scan;
use std::collections::HashSet;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut bricks = Vec::new();
    while !input.is_empty() {
        scan!(input, "@,@,@~@,@,@", x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2: usize);
        bricks.push((x1, y1, z1, x2, y2, z2));
    }
    bricks.sort_by_key(|&(_, _, z1, _, _, z2)| z1.min(z2));
    let mut occupied = HashSet::new();
    for (x1, y1, z1, x2, y2, z2) in &mut bricks {
        if *x1 > *x2 {
            swap(x1, x2);
        }
        if *y1 > *y2 {
            swap(y1, y2);
        }
        if *z1 > *z2 {
            swap(z1, z2);
        }
        while *z1 > 1 {
            let mut settled = false;
            'outside: for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    for z in *z1 - 1..*z2 {
                        if occupied.contains(&(x, y, z)) {
                            settled = true;
                            break 'outside;
                        }
                    }
                }
            }
            if settled {
                break;
            }
            *z1 -= 1;
            *z2 -= 1;
        }
        for x in *x1..=*x2 {
            for y in *y1..=*y2 {
                for z in *z1..=*z2 {
                    occupied.insert((x, y, z));
                }
            }
        }
    }

    {
        // part 1
        let mut ans = 0;
        for (i, &(x1, y1, z1, x2, y2, z2)) in bricks.enumerate() {
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        occupied.remove(&(x, y, z));
                    }
                }
            }
            let mut good = true;
            for &(x1, y1, z1, x2, y2, _) in bricks.iter().skip(i + 1) {
                if z1 == 1 {
                    continue;
                }
                let mut can_fall = true;
                'outer: for x in x1..=x2 {
                    for y in y1..=y2 {
                        if occupied.contains(&(x, y, z1 - 1)) {
                            can_fall = false;
                            break 'outer;
                        }
                    }
                }
                if can_fall {
                    good = false;
                    break;
                }
            }
            if good {
                ans += 1;
            }
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        occupied.insert((x, y, z));
                    }
                }
            }
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut ans = 0;
        for (i, &(x1, y1, z1, x2, y2, z2)) in bricks.enumerate() {
            let mut occupied = occupied.clone();
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        occupied.remove(&(x, y, z));
                    }
                }
            }
            for &(x1, y1, z1, x2, y2, z2) in bricks.iter().skip(i + 1) {
                if z1 == 1 {
                    continue;
                }
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        for z in z1..=z2 {
                            occupied.remove(&(x, y, z));
                        }
                    }
                }
                let mut new_z1 = z1;
                let mut new_z2 = z2;
                while new_z1 > 1 {
                    let mut settled = false;
                    'outside: for x in x1..=x2 {
                        for y in y1..=y2 {
                            for z in new_z1 - 1..new_z2 {
                                if occupied.contains(&(x, y, z)) {
                                    settled = true;
                                    break 'outside;
                                }
                            }
                        }
                    }
                    if settled {
                        break;
                    }
                    new_z1 -= 1;
                    new_z2 -= 1;
                }
                if z1 != new_z1 {
                    ans += 1;
                }
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        for z in new_z1..=new_z2 {
                            occupied.insert((x, y, z));
                        }
                    }
                }
            }
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
