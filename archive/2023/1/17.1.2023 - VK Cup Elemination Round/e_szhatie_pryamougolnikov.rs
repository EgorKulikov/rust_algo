//{"name":"E. Сжатие прямоугольников","group":"Codeforces - VK Cup 2022 - Отборочный раунд (Engine)","url":"https://codeforces.com/contest/1781/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"8\n5\n1 2 2 4\n2 4 2 8\n1 4 2 7\n1 2 1 2\n1 9 1 10\n2\n1 1 1 10\n1 5 1 15\n2\n1 1 1 10\n1 1 1 10\n5\n1 3 1 7\n1 3 1 8\n1 1 1 4\n1 2 1 7\n1 10 1 11\n2\n1 1 2 10\n1 5 1 8\n2\n1 5 2 10\n1 2 1 7\n2\n1 5 2 10\n2 2 2 15\n5\n2 6 2 7\n1 4 2 5\n1 5 1 9\n1 7 2 10\n1 2 1 6\n","output":"15\n1 2 2 4\n2 5 2 8\n1 5 1 7\n0 0 0 0\n1 9 1 10\n15\n1 1 1 10\n1 11 1 15\n10\n1 1 1 10\n0 0 0 0\n10\n0 0 0 0\n1 8 1 8\n1 1 1 4\n1 5 1 7\n1 10 1 11\n20\n1 1 2 10\n0 0 0 0\n15\n1 5 2 10\n1 2 1 4\n20\n1 5 1 10\n2 2 2 15\n16\n2 6 2 6\n2 4 2 5\n0 0 0 0\n1 7 2 10\n1 2 1 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESzhatiePryamougolnikov"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let rect = input.read_vec::<(usize, usize, usize, usize)>(n);

    let mut both = 0;
    let mut both_id = n;
    let mut up = 0;
    let mut up_id = n;
    let mut down = 0;
    let mut down_id = n;
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| rect[i].1);
    let mut ans = vec![(0usize, 0, 0usize, 0); n];
    for i in order {
        let x = rect[i].1;
        if both < x {
            if both != 0 {
                ans[both_id].3 = both;
                if up != 0 {
                    ans[up_id].1 = both + 1;
                }
                if down != 0 {
                    ans[down_id].1 = both + 1;
                }
            }
            both = 0;
        }
        if up < x {
            if up != 0 {
                ans[up_id].3 = up;
            }
            up = 0;
        }
        if down < x {
            if down != 0 {
                ans[down_id].3 = down;
            }
            down = 0;
        }
        if rect[i].0 == 1 {
            if rect[i].2 == 2 {
                if both != 0 && rect[i].3 > both
                    || both == 0 && rect[i].3 >= up && rect[i].3 >= down
                {
                    if both != 0 {
                        ans[both_id].3 = x - 1;
                        if up <= rect[i].3 {
                            up = 0;
                        }
                        if down <= rect[i].3 {
                            down = 0;
                        }
                    } else {
                        if up <= rect[i].3 {
                            if up != 0 {
                                ans[up_id].3 = x - 1;
                            }
                            up = 0;
                        }
                        if down <= rect[i].3 {
                            if down != 0 {
                                ans[down_id].3 = x - 1;
                            }
                            down = 0;
                        }
                    }
                    both = rect[i].3;
                    both_id = i;
                    ans[i].0 = 1;
                    ans[i].2 = 2;
                    ans[i].1 = x;
                } else if both == 0 && rect[i].3 > up {
                    if up != 0 {
                        ans[up_id].3 = x - 1;
                    }
                    up = rect[i].3;
                    up_id = i;
                    ans[i].0 = 1;
                    ans[i].2 = 1;
                    ans[i].1 = x;
                } else if both == 0 && rect[i].3 > down {
                    if down != 0 {
                        ans[down_id].3 = x - 1;
                    }
                    down = rect[i].3;
                    down_id = i;
                    ans[i].0 = 2;
                    ans[i].2 = 2;
                    ans[i].1 = x;
                }
            } else {
                if rect[i].3 > both && rect[i].3 > up {
                    if both == 0 && up != 0 {
                        ans[up_id].3 = x - 1;
                    }
                    up = rect[i].3;
                    up_id = i;
                    ans[i].0 = 1;
                    ans[i].2 = 1;
                    if both == 0 {
                        ans[i].1 = x;
                    }
                }
            }
        } else {
            if rect[i].3 > both && rect[i].3 > down {
                if both == 0 && down != 0 {
                    ans[down_id].3 = x - 1;
                }
                down = rect[i].3;
                down_id = i;
                ans[i].0 = 2;
                ans[i].2 = 2;
                if both == 0 {
                    ans[i].1 = x;
                }
            }
        }
    }
    if both != 0 {
        ans[both_id].3 = both;
        if up != 0 {
            ans[up_id].1 = both + 1;
        }
        if down != 0 {
            ans[down_id].1 = both + 1;
        }
    }
    if up != 0 {
        ans[up_id].3 = up;
    }
    if down != 0 {
        ans[down_id].3 = down;
    }
    for (a, b, c, d) in &mut ans {
        if *b == 0 || *d == 0 || *b > *d {
            *a = 0;
            *b = 0;
            *c = 0;
            *d = 0;
        }
    }
    let mut s = 0;
    for &(a, b, c, d) in &ans {
        if a != 0 {
            s += (d - b + 1) * (c - a + 1);
        }
    }
    out_line!(s);
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
