use algo_lib::collections::arr3d::Arr3d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::{Bounds, IncDec};
use algo_lib::io::input::{Input, Readable};
use algo_lib::{compress, zip};

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

    let mut t = Vec::new();
    let mut x_from: Vec<i64> = Vec::new();
    let mut x_to: Vec<i64> = Vec::new();
    let mut y_from: Vec<i64> = Vec::new();
    let mut y_to: Vec<i64> = Vec::new();
    let mut z_from: Vec<i64> = Vec::new();
    let mut z_to: Vec<i64> = Vec::new();

    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        t.push(inp.read::<String>() == "on");
        let coords: String = inp.read();
        let coords = coords.split(',').collect_vec();
        assert_eq!(coords.len(), 3);
        let x = coords[0].split('=').collect_vec()[1]
            .split("..")
            .collect_vec();
        assert_eq!(x.len(), 2);
        x_from.push(x[0].parse().unwrap());
        x_to.push(x[1].parse().unwrap());
        let y = coords[1].split('=').collect_vec()[1]
            .split("..")
            .collect_vec();
        assert_eq!(y.len(), 2);
        y_from.push(y[0].parse().unwrap());
        y_to.push(y[1].parse().unwrap());
        let z = coords[2].split('=').collect_vec()[1]
            .split("..")
            .collect_vec();
        assert_eq!(z.len(), 2);
        z_from.push(z[0].parse().unwrap());
        z_to.push(z[1].parse().unwrap());
    }
    let x_to = x_to.inc_by_one();
    let y_to = y_to.inc_by_one();
    let z_to = z_to.inc_by_one();
    let (x, (x_from, x_to, x_bounds)) = compress!(x_from, x_to, vec![-50, 51]);
    let (y, (y_from, y_to, y_bounds)) = compress!(y_from, y_to, vec![-50, 51]);
    let (z, (z_from, z_to, z_bounds)) = compress!(z_from, z_to, vec![-50, 51]);

    let mut state = Arr3d::new(x.len(), y.len(), z.len(), false);
    for (t, x_from, x_to, y_from, y_to, z_from, z_to) in zip!(
        t.into_iter(),
        x_from.into_iter(),
        x_to.into_iter(),
        y_from.into_iter(),
        y_to.into_iter(),
        z_from.into_iter(),
        z_to.into_iter()
    ) {
        for x in x_from..x_to {
            for y in y_from..y_to {
                for z in z_from..z_to {
                    state[(x, y, z)] = t;
                }
            }
        }
    }

    let mut ans = 0i64;
    for i in x_bounds[0]..x_bounds[1] {
        for j in y_bounds[0]..y_bounds[1] {
            for k in z_bounds[0]..z_bounds[1] {
                if state[(i, j, k)] {
                    ans += (x[i + 1] - x[i]) * (y[j + 1] - y[j]) * (z[k + 1] - z[k]);
                }
            }
        }
    }
    println!("{}", ans);
}
