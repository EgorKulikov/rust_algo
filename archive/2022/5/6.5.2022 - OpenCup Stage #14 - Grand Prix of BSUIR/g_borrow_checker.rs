//{"name":"G. Borrow checker","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/G/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nfn input()\nfn clone(&a)\nfn sum(a, b)\n5\na := input()\nb := input()\nbr := &b\ns := sum(a, b)\nbc := clone(br)\n","output":"Error in line 5\n"},{"input":"2\nfn input()\nfn clone(&a)\n3\na := input()\na := &a\nb := clone(a)\n","output":"Error in line 3\n"},{"input":"2\nfn &input()\nfn sum(&a, &b)\n3\na := input()\nb := sum(a, a)\nc := sum(a, &b)\n","output":"Valid\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GBorrowChecker"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::out_line;
use std::collections::{HashMap, HashSet};
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    #[derive(Eq, PartialEq, Copy, Clone)]
    enum VarType {
        Value,
        Ref,
    }
    struct Function {
        return_value: VarType,
        arguments: Vec<VarType>,
    }
    enum Variable {
        Value(HashSet<String>),
        Reference(Option<String>),
    }
    enum Expression {
        Value(String),
        Reference(String),
        FunctionCall(String, Vec<Expression>),
    }

    impl Expression {
        fn parse(mut s: &[u8]) -> (Self, &[u8]) {
            for (i, &c) in s.iter().enumerate() {
                if c == b'(' {
                    let name = String::from_utf8(s[..i].to_vec()).unwrap();
                    let mut args = Vec::new();
                    if s[i + 1] == b')' {
                        return (Expression::FunctionCall(name, args), &s[i + 2..]);
                    }
                    s = &s[i + 1..];
                    loop {
                        let (exp, cs) = Self::parse(s);
                        args.push(exp);
                        s = cs;
                        if s[0] == b')' {
                            return (Expression::FunctionCall(name, args), &s[1..]);
                        } else {
                            s = &s[2..];
                        }
                    }
                }
                if c == b',' || c == b')' {
                    if s[0] == b'&' {
                        return (
                            Expression::Reference(String::from_utf8(s[1..i].to_vec()).unwrap()),
                            &s[i..],
                        );
                    } else {
                        return (
                            Expression::Value(String::from_utf8(s[..i].to_vec()).unwrap()),
                            &s[i..],
                        );
                    }
                }
            }
            if s[0] == b'&' {
                return (
                    Expression::Reference(String::from_utf8(s[1..].to_vec()).unwrap()),
                    &[],
                );
            } else {
                return (
                    Expression::Value(String::from_utf8(s.to_vec()).unwrap()),
                    &[],
                );
            }
        }
    }

    let n = input.read_usize();
    let mut functions = HashMap::new();

    for _ in 0..n {
        input.read_string();
        let signature = input
            .read_line()
            .replace("(", " ")
            .replace(")", " ")
            .replace(",", " ");
        let tokens = signature.split(" ").collect_vec();
        let mut name = tokens[0];
        let is_ref = name.starts_with('&');
        if is_ref {
            name = name.trim_start_matches('&');
        }
        let mut args = Vec::new();
        for t in tokens.into_iter().skip(1) {
            if t.is_empty() {
                continue;
            }
            args.push(if t.starts_with('&') {
                VarType::Ref
            } else {
                VarType::Value
            });
        }
        functions.insert(
            name.to_string(),
            Function {
                return_value: if is_ref { VarType::Ref } else { VarType::Value },
                arguments: args,
            },
        );
    }

    let m = input.read_usize();
    let mut variables: HashMap<String, Variable> = HashMap::new();

    for line_number in 1..=m {
        let new_variable = input.read_string();
        input.read_string();
        let expression = Expression::parse(input.read_line().as_bytes()).0;
        let mut check = RecursiveFunction::new(
            |f, e: &Expression| -> Option<(VarType, Vec<String>, Vec<String>)> {
                match e {
                    Expression::Value(name) => match variables.get(name) {
                        None => None,
                        Some(var) => match var {
                            Variable::Value(_) => {
                                Some((VarType::Value, vec![name.clone()], Vec::new()))
                            }
                            Variable::Reference(_) => {
                                Some((VarType::Ref, Vec::new(), vec![name.clone()]))
                            }
                        },
                    },
                    Expression::FunctionCall(fun_name, args) => match functions.get(fun_name) {
                        None => None,
                        Some(fun) => {
                            if fun.arguments.len() != args.len() {
                                None
                            } else {
                                let mut values = Vec::new();
                                let mut refs = Vec::new();
                                for (ff, e) in fun.arguments.iter().zip(args.iter()) {
                                    match f.call(e) {
                                        None => {
                                            return None;
                                        }
                                        Some((vt, mut c_values, mut c_refs)) => {
                                            if *ff != vt {
                                                return None;
                                            }
                                            if c_values.len() > values.len() {
                                                swap(&mut values, &mut c_values);
                                            }
                                            values.extend_from_slice(&c_values);
                                            if c_refs.len() > refs.len() {
                                                swap(&mut c_refs, &mut refs);
                                            }
                                            refs.extend_from_slice(&c_refs);
                                        }
                                    }
                                }
                                Some((fun.return_value, values, refs))
                            }
                        }
                    },
                    Expression::Reference(name) => match variables.get(name) {
                        None => None,
                        Some(var) => match var {
                            Variable::Value(_) => {
                                Some((VarType::Ref, Vec::new(), vec![name.clone()]))
                            }
                            Variable::Reference(_) => None,
                        },
                    },
                }
            },
        );
        let ok = match check.call(&expression) {
            None => false,
            Some((_, values, refs)) => {
                let mut good = true;
                for v in values {
                    match variables.get(&v) {
                        None => {
                            good = false;
                            break;
                        }
                        Some(var) => match var {
                            Variable::Value(var) => {
                                for vv in var.clone() {
                                    variables.remove(&vv);
                                }
                                variables.remove(&v);
                            }
                            Variable::Reference(_) => {
                                unreachable!();
                            }
                        },
                    }
                }
                for v in refs {
                    if !variables.contains_key(&v) {
                        good = false;
                        break;
                    }
                }
                good
            }
        };
        if !ok {
            out_line!("Error in line", line_number);
            return;
        }
        let mut removed = HashSet::new();
        if let Some(var) = variables.get(&new_variable) {
            match var {
                Variable::Value(vv) => {
                    removed = vv.clone();
                    for v in &removed {
                        variables.remove(v);
                    }
                }
                Variable::Reference(par) => {
                    if let Some(par) = par {
                        let par = par.clone();
                        match variables.get_mut(&par) {
                            None => {
                                unreachable!();
                            }
                            Some(par) => match par {
                                Variable::Value(ch) => {
                                    ch.remove(&new_variable);
                                }
                                Variable::Reference(_) => {
                                    unreachable!();
                                }
                            },
                        }
                    }
                }
            }
        }
        match expression {
            Expression::Value(v) => {
                if !removed.contains(&v) {
                    match variables.get(&v) {
                        None => {
                            variables.insert(new_variable, Variable::Value(HashSet::new()));
                        }
                        Some(var) => match var {
                            Variable::Value(_) => {
                                unreachable!();
                            }
                            Variable::Reference(r) => match r {
                                None => {
                                    variables.insert(new_variable, Variable::Reference(None));
                                }
                                Some(vv) => {
                                    let vv = vv.clone();
                                    variables.insert(
                                        new_variable.clone(),
                                        Variable::Reference(Some(vv.clone())),
                                    );
                                    match variables.get_mut(&vv).unwrap() {
                                        Variable::Value(v) => {
                                            v.insert(new_variable);
                                        }
                                        Variable::Reference(_) => {
                                            unreachable!();
                                        }
                                    }
                                }
                            },
                        },
                    }
                }
            }
            Expression::Reference(r) => {
                if r != new_variable {
                    variables.insert(new_variable.clone(), Variable::Reference(Some(r.clone())));
                    match variables.get_mut(&r).unwrap() {
                        Variable::Value(v) => {
                            v.insert(new_variable);
                        }
                        Variable::Reference(_) => {
                            unreachable!();
                        }
                    }
                } else {
                    variables.remove(&r);
                }
            }
            Expression::FunctionCall(fun, _) => match functions.get(&fun).unwrap().return_value {
                VarType::Value => {
                    variables.insert(new_variable, Variable::Value(HashSet::new()));
                }
                VarType::Ref => {
                    variables.insert(new_variable, Variable::Reference(None));
                }
            },
        }
    }
    out_line!("Valid");
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
