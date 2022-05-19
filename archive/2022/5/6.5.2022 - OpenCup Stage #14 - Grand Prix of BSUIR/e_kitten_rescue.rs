//{"name":"E. Kitten rescue","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/E/","interactive":false,"timeLimit":3000,"tests":[{"input":"4 6\n..C...\n##....\n.#...K\n..@...\n","output":"Yes\nLLRRRRRDD\n"},{"input":"1 6\nK.@..C\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKittenRescue"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::arr4d::Arr4d;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::misc::dirs::D4;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let t = input.read_table::<char>(n, m);

    let mut cat = None;
    let mut dog = None;
    let mut kitten = None;
    for i in 0..n {
        for j in 0..m {
            match t[(i, j)] {
                'C' => cat = Some((i, j)),
                '@' => dog = Some((i, j)),
                'K' => kitten = Some((i, j)),
                _ => {}
            }
        }
    }
    let kitten = kitten.unwrap();
    let has_dog = dog.is_some();
    let mut res = Arr4d::new(n, m, n, m, None);
    let mut rec = RecursiveFunction2::new(|f, cat: (usize, usize), dog: (usize, usize)| -> bool {
        if res[(cat.0, cat.1, dog.0, dog.1)].is_some() {
            return false;
        }
        res[(cat.0, cat.1, dog.0, dog.1)] = Some(None);
        if cat == kitten {
            return true;
        }
        if has_dog && cat == dog {
            return false;
        }
        for n_cat in D4::iter(cat.0, cat.1, n, m) {
            if t[n_cat] == '#' {
                continue;
            }
            let mut n_dog = dog;
            if has_dog {
                for _ in 0..2 {
                    if n_cat.1 > n_dog.1 && t[(n_dog.0, n_dog.1 + 1)] != '#' {
                        n_dog.1 += 1;
                        continue;
                    }
                    if n_cat.1 < n_dog.1 && t[(n_dog.0, n_dog.1 - 1)] != '#' {
                        n_dog.1 -= 1;
                        continue;
                    }
                    if n_cat.0 > n_dog.0 && t[(n_dog.0 + 1, n_dog.1)] != '#' {
                        n_dog.0 += 1;
                        continue;
                    }
                    if n_cat.0 < n_dog.0 && t[(n_dog.0 - 1, n_dog.1)] != '#' {
                        n_dog.0 -= 1;
                        continue;
                    }
                }
            }
            if f.call(n_cat, n_dog) {
                res[(cat.0, cat.1, dog.0, dog.1)] = Some(Some((
                    n_cat.0.into_i8(),
                    n_cat.1.into_i8(),
                    n_dog.0.into_i8(),
                    n_dog.1.into_i8(),
                )));
                return true;
            }
        }
        false
    });
    let mut cat = cat.unwrap();
    let mut dog = match dog {
        None => (0, 0),
        Some(dog) => dog,
    };
    output().bool_output = BoolOutput::YesNo;
    out_line!(rec.call(cat, dog));
    loop {
        if let Some(Some((cat_r, cat_c, dog_r, dog_c))) = res[(cat.0, cat.1, dog.0, dog.1)] {
            let n_cat = (cat_r.into_usize(), cat_c.into_usize());
            let n_dog = (dog_r.into_usize(), dog_c.into_usize());
            if cat.0 < n_cat.0 {
                out!('D');
            } else if cat.0 > n_cat.0 {
                out!('U');
            } else if cat.1 < n_cat.1 {
                out!('R');
            } else {
                out!('L');
            }
            cat = n_cat;
            dog = n_dog;
        } else {
            break;
        }
    }
    out_line!();
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
