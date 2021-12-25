#![feature(map_first_last)]

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::{IterExt, IterPartialEqExt};
use algo_lib::io::input::{Input, Readable};

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

    let mut id = 0;
    let mut cf = Arr2d::new(4, 15, 0i128);
    let mut vv = vec![0; 4];
    let is = [9, 9, 4, 2, 9, 7, 9, 5, 9, 9, 3, 9, 2, 9];

    let mut instr_id = 0;
    loop {
        // println!("After {}", instr_id);
        fn build(v: &[i128]) -> String {
            let mut res = String::new();
            res += format!("{}", v[0]).as_str();
            for i in 1..15 {
                if v[i] == 0 {
                    continue;
                }
                if v[i] > 0 {
                    res += " + ";
                } else {
                    res += " - ";
                }
                res += format!("{}a{}", v[i].abs(), i - 1).as_str();
            }
            res
        }
        // println!("w = {}", build(&cf[0]));
        // println!("x = {}", build(&cf[1]));
        // println!("y = {}", build(&cf[2]));
        // println!("z = {}", build(&cf[3]));
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let t: String = inp.read();
        let mut reg_id_or_num = || {
            inp.skip_whitespace();
            if inp.peek().unwrap().is_ascii_alphabetic() {
                let reg: char = inp.read();
                Ok((reg as u8 - b'w') as usize)
            } else {
                Err(inp.read::<i128>())
            }
        };
        match t.as_str() {
            "inp" => {
                let reg = reg_id_or_num().unwrap();
                cf[reg].fill(0);
                vv[reg] = is[id];
                id += 1;
                cf[(reg, id)] = 1;
            }
            "add" => {
                let reg = reg_id_or_num().unwrap();
                match reg_id_or_num() {
                    Ok(o_reg) => {
                        vv[reg] += vv[o_reg];
                        for i in 0..15 {
                            let to_add = cf[(o_reg, i)];
                            cf[(reg, i)] += to_add;
                        }
                    }
                    Err(val) => {
                        vv[reg] += val;
                        cf[(reg, 0)] += val;
                    }
                }
            }
            "mul" => {
                let reg = reg_id_or_num().unwrap();
                match reg_id_or_num() {
                    Ok(o_reg) => {
                        vv[reg] *= vv[o_reg];
                        for i in 0..15 {
                            let to_add = cf[(o_reg, 0)];
                            cf[(reg, i)] *= to_add;
                            if i > 0 && cf[(o_reg, i)] != 0 {
                                panic!("");
                            }
                        }
                    }
                    Err(val) => {
                        vv[reg] *= val;
                        for i in 0..15 {
                            cf[(reg, i)] *= val;
                        }
                    }
                }
            }
            "div" => {
                let reg = reg_id_or_num().unwrap();
                let val = reg_id_or_num().err().unwrap();
                vv[reg] /= val;
                if val != 1 {
                    let mut from = 0;
                    let mut to = 0;
                    for i in 0..15 {
                        if i > 0 && cf[(reg, i)] % val != 0 && cf[(reg, i)] % val != 1 {
                            panic!("");
                        }
                        if i > 0 && cf[(reg, i)] % val == 1 {
                            from += 1;
                            to += 9;
                        }
                        if i == 0 {
                            from += cf[(reg, i)] % val;
                            to += cf[(reg, i)] % val;
                        }
                        cf[(reg, i)] /= val;
                    }
                    if from / val != to / val {
                        panic!("");
                    }
                    cf[(reg, 0)] -= (from / val) * val;
                }
            }
            "mod" => {
                let reg = reg_id_or_num().unwrap();
                let val = reg_id_or_num().err().unwrap();
                vv[reg] %= val;
                let mut from = 0;
                let mut to = 0;
                for i in 0..15 {
                    if i > 0 && cf[(reg, i)] % val != 0 && cf[(reg, i)] != 1 {
                        panic!("");
                    }
                    if i > 0 && cf[(reg, i)] == 1 {
                        from += 1;
                        to += 9;
                    }
                    cf[(reg, i)] %= val;
                    if i == 0 {
                        from += cf[(reg, i)];
                        to += cf[(reg, i)];
                    }
                }
                if from / val != to / val {
                    panic!("");
                }
            }
            "eql" => {
                let mut from = 0;
                let mut to = 0;
                let reg = reg_id_or_num().unwrap();
                for i in 0..15 {
                    if i == 0 {
                        from += cf[(reg, i)];
                        to += cf[(reg, i)];
                    } else {
                        from += cf[(reg, i)];
                        to += 9 * cf[(reg, i)];
                    }
                }
                let mut delta = 0;
                let mut in_o_reg = 0;
                match reg_id_or_num() {
                    Ok(o_reg) => {
                        vv[reg] = if vv[reg] == vv[o_reg] { 1 } else { 0 };
                        delta = cf[(reg, 0)] - cf[(o_reg, 0)];
                        in_o_reg = cf[o_reg].iter().skip(1).find(&1).unwrap_or(0);
                        for i in 0..15 {
                            if i == 0 {
                                from -= cf[(o_reg, i)];
                                to -= cf[(o_reg, i)];
                            } else {
                                from -= 9 * cf[(o_reg, i)];
                                to -= cf[(o_reg, i)];
                            }
                        }
                    }
                    Err(val) => {
                        vv[reg] = if vv[reg] == val { 1 } else { 0 };
                        from -= val;
                        to -= val;
                    }
                }
                let in_reg = cf[reg].iter().skip(1).find(&1).unwrap_or(0);
                cf[reg].fill(0);
                if from == to {
                    cf[(reg, 0)] = if from == 0 { 1 } else { 0 };
                } else {
                    if from <= 0 && to >= 0 {
                        println!(
                            "Delta = {}, in reg = {}, in o_reg = {}",
                            delta, in_reg, in_o_reg
                        );
                        // if in_o_reg != 5 {
                        cf[(reg, 0)] = 1;
                        // }
                    }
                }
            }
            _ => unreachable!(),
        }
        for i in 0..4 {
            let mut value = 0;
            for j in 0..15 {
                if j == 0 {
                    value += cf[(i, j)];
                } else {
                    value += cf[(i, j)] * is[j - 1];
                }
            }
            assert_eq!(value, vv[i]);
        }
        instr_id += 1;
    }
    for i in cf[3].iter() {
        println!("{}", *i);
    }
    println!("vv = {}", vv[3]);
}
