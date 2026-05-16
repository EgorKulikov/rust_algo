//{"name":"ucup_10_g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup_10_g"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let castles = input.read_size_pair_vec(n);
    let obstacles = input.read_size_pair_vec(m);

    let mut base = 0;
    #[derive(Copy, Clone)]
    enum ObjectType {
        Castle,
        Obstacle,
    }
    #[derive(Copy, Clone)]
    struct Object {
        id: usize,
        x: usize,
        y: usize,
        t: ObjectType,
    }

    let mut objects = castles
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| Object {
            id: i,
            x,
            y,
            t: ObjectType::Castle,
        })
        .chain(obstacles.iter().enumerate().map(|(i, &(x, y))| Object {
            id: i,
            x,
            y,
            t: ObjectType::Obstacle,
        }))
        .collect::<Vec<_>>();
    objects.sort_by_key(|o| (o.x, o.y));
    let mut last_x = 0;
    let mut obstacles_in_gap = Vec::new();
    let mut castle_started = false;
    let mut hor_gap_id = 0;
    let mut obst_hor_gap = vec![None; m];
    for obj in &objects {
        if obj.x != last_x {
            obstacles_in_gap.clear();
            castle_started = false;
        }
        match obj.t {
            ObjectType::Castle => {
                if castle_started {
                    if obstacles_in_gap.is_empty() {
                        base += 1;
                    } else {
                        for &obstacle in &obstacles_in_gap {
                            obst_hor_gap[obstacle] = Some(hor_gap_id);
                        }
                        obstacles_in_gap.clear();
                        hor_gap_id += 1;
                    }
                }
                castle_started = true;
            }
            ObjectType::Obstacle => {
                if castle_started {
                    obstacles_in_gap.push(obj.id);
                }
            }
        }
        last_x = obj.x;
    }
    objects.sort_by_key(|o| (o.y, o.x));
    let mut last_y = 0;
    let mut obstacles_in_gap = Vec::new();
    let mut castle_started = false;
    let mut ver_gap_id = 0;
    let mut obst_ver_gap = vec![None; m];
    for obj in &objects {
        if obj.y != last_y {
            obstacles_in_gap.clear();
            castle_started = false;
        }
        match obj.t {
            ObjectType::Castle => {
                if castle_started {
                    if obstacles_in_gap.is_empty() {
                        base += 1;
                    } else {
                        for &obstacle in &obstacles_in_gap {
                            obst_ver_gap[obstacle] = Some(ver_gap_id);
                        }
                        obstacles_in_gap.clear();
                        ver_gap_id += 1;
                    }
                }
                castle_started = true;
            }
            ObjectType::Obstacle => {
                if castle_started {
                    obstacles_in_gap.push(obj.id);
                }
            }
        }
        last_y = obj.y;
    }
    let mut graph = Graph::new(hor_gap_id + ver_gap_id + 3);
    let source = hor_gap_id + ver_gap_id;
    let sink = source + 1;
    let real_source = sink + 1;
    for i in 0..hor_gap_id {
        graph.add_edge(FlowEdge::with_payload(source, i, 1, 0));
    }
    for i in 0..ver_gap_id {
        graph.add_edge(FlowEdge::with_payload(hor_gap_id + i, sink, 1, 0));
    }
    for i in 0..m {
        if let Some(hor_gap) = obst_hor_gap[i] {
            if let Some(ver_gap) = obst_ver_gap[i] {
                graph.add_edge(FlowEdge::with_payload(hor_gap, hor_gap_id + ver_gap, 1, i));
            }
        }
    }
    graph.add_edge(FlowEdge::with_payload(real_source, source, m - k, 0));
    let mut ans = base + hor_gap_id + ver_gap_id;
    let max_flow = graph.max_flow(real_source, sink);
    ans -= 2 * max_flow;
    let mut to_remove = BitSet::new(m);
    to_remove.fill(true);
    let mut left_done = BitSet::new(hor_gap_id);
    let mut right_done = BitSet::new(ver_gap_id);
    for i in 0..hor_gap_id {
        for e in &graph[i] {
            if e.to() != source && e.flow(&graph) == 1 {
                to_remove.unset(*e.payload());
                left_done.set(i);
                right_done.set(e.to() - hor_gap_id);
            }
        }
    }
    let mut can_do = m - k - max_flow;
    for i in 0..m {
        if let Some(hor_gap) = obst_hor_gap[i] {
            if !left_done[hor_gap] && can_do > 0 {
                assert!(to_remove[i]);
                can_do -= 1;
                to_remove.unset(i);
                left_done.set(hor_gap);
                ans -= 1;
            }
        }
        if let Some(ver_gap) = obst_ver_gap[i] {
            if !right_done[ver_gap] && can_do > 0 {
                assert!(to_remove[i]);
                can_do -= 1;
                to_remove.unset(i);
                right_done.set(ver_gap);
                ans -= 1;
            }
        }
    }
    for i in 0..m {
        if to_remove[i] && can_do > 0 {
            can_do -= 1;
            to_remove.unset(i);
        }
    }
    out.print_line(ans);
    out.print_line(to_remove.iter().collect_vec().inc());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
