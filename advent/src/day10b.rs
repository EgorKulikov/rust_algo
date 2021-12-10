use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
use algo_lib::string::string::Str;
use std::collections::VecDeque;

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
    let mut inp = Input::new(&mut sin);

    let mut ans = Vec::new();

    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }

        let cur = inp.read::<Str>();
        let mut st = VecDeque::new();
        let mut good = true;
        for c in cur {
            match c {
                b']' => {
                    if st.back().unwrap() == &b'[' {
                        st.pop_back();
                    } else {
                        good = false;
                        break;
                    }
                }
                b')' => {
                    if st.back().unwrap() == &b'(' {
                        st.pop_back();
                    } else {
                        good = false;
                        break;
                    }
                }
                b'}' => {
                    if st.back().unwrap() == &b'{' {
                        st.pop_back();
                    } else {
                        good = false;
                        break;
                    }
                }
                b'>' => {
                    if st.back().unwrap() == &b'<' {
                        st.pop_back();
                    } else {
                        good = false;
                        break;
                    }
                }
                c => st.push_back(c),
            }
        }

        if good {
            let mut cur = 0u64;
            while let Some(c) = st.pop_back() {
                cur *= 5;
                cur += match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                }
            }
            ans.push(cur);
        }
    }

    ans.sort();

    println!("{}", ans[ans.len() / 2]);
}
