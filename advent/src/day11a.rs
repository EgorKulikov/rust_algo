use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, InputIterator, Readable};
use algo_lib::misc::dirs::D8;

pub trait CommaList {
    fn read_list<T: Readable>(&mut self) -> Vec<T>;
}

impl CommaList for Input<'_> {
    fn read_list<T: Readable>(&mut self) -> Vec<T> {
        let mut s: String = self.read();
        s = s.replace(",", " ");
        let mut b = s.as_bytes();
        let input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let inp = Input::new(&mut sin);

    let mut iter: InputIterator<char> = inp.into_iter();
    let mut map = Arr2d::generate(10, 10, |_, _| (iter.next().unwrap() as u8) - b'0');
    let mut ans = 0;

    let mut flashed = Arr2d::new(10, 10, false);
    for _ in 0..100 {
        map.iter_mut().for_each(|i| *i += 1);
        flashed.fill(false);
        loop {
            let mut flash = false;
            for i in 0..10 {
                for j in 0..10 {
                    if !flashed[(i, j)] && map[(i, j)] >= 10 {
                        flashed[(i, j)] = true;
                        flash = true;
                        for next in D8::iter(i, j, 10, 10) {
                            map[next] += 1;
                        }
                    }
                }
            }
            if !flash {
                break;
            }
        }
        map.iter_mut().for_each(|i| {
            if *i >= 10 {
                ans += 1;
                *i = 0;
            }
        })
    }

    println!("{}", ans);
}
