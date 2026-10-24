//! CCC 42 classic: Nanofabricator. Usage: nano <level> <input> <output>

mod board;
mod level1;
mod level2;
mod level3;
mod level4;
mod level5;
mod level6;
mod level6b;
mod level6c;
mod level6d;
mod level6e;
mod level6f;
mod level6g;
mod level6h;
mod level6i;
mod level6j;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let level: usize = args[1].parse().unwrap();
    let input = std::fs::read_to_string(&args[2]).unwrap();
    let output = match level {
        1 => level1::solve(&input),
        2 => level2::solve(&input),
        3 => level3::solve(&input),
        4 => level4::solve(&input),
        5 => level5::solve(&input),
        6 => level6::solve(&input),
        7 => level6::trace(&input, &std::fs::read_to_string(&args[3]).unwrap()),
        _ => panic!("level {} not implemented", level),
    };
    std::fs::write(&args[if level == 7 { 4 } else { 3 }], output).unwrap();
}
